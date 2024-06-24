/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

mod print_state;

use print_state::print_envs;
use print_state::print_fs_state;
use crate::ReplResult;

use fluence_app_service::WasmtimeConfig;
use fluence_app_service::AppService;
use fluence_app_service::AppServiceFactory;
use fluence_app_service::CallParameters;
use fluence_app_service::ParticleParameters;
use fluence_app_service::SecurityTetraplet;
use fluence_app_service::MarineModuleConfig;
use fluence_app_service::TomlAppServiceConfig;

use anyhow::anyhow;
use serde::Deserialize;
use serde_json::Value as JValue;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

macro_rules! next_argument {
    ($arg_name:ident, $args:ident, $error_msg:expr) => {
        let $arg_name = match $args.next() {
            Some($arg_name) => $arg_name,
            None => {
                println!($error_msg);
                return;
            }
        };
    };
}

macro_rules! next_argument_or_result {
    ($arg_name:ident, $args:ident, $error_msg:expr) => {
        let $arg_name = match $args.next() {
            Some($arg_name) => $arg_name,
            None => return Err(String::from($error_msg)),
        };
    };
}

struct CallModuleArguments<'args> {
    module_name: &'args str,
    func_name: &'args str,
    show_result_arg: bool,
    args: JValue,
    call_parameters: CallParameters,
}

const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(100);
#[allow(clippy::upper_case_acronyms)]
pub(super) struct REPL {
    app_service: AppService,
    service_working_dir: Option<String>,
    app_service_factory: AppServiceFactory,
    timeout: std::time::Duration,
}

impl REPL {
    pub async fn new<S: Into<PathBuf>>(
        config_file_path: Option<S>,
        working_dir: Option<String>,
        quiet: bool,
    ) -> ReplResult<Self> {
        let mut backend_config = WasmtimeConfig::default();
        backend_config.epoch_interruption(true);

        let (app_service_factory, ticker) = AppServiceFactory::new(backend_config)?;
        let app_service = Self::create_app_service(
            &app_service_factory,
            config_file_path,
            working_dir.clone(),
            quiet,
        )
        .await?;

        Self::spawn_ticker_thread(ticker);
        Ok(Self {
            app_service,
            service_working_dir: working_dir,
            app_service_factory,
            timeout: DEFAULT_TIMEOUT,
        })
    }

    /// Returns true, it should be the last executed command.
    pub async fn execute<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) -> bool {
        // Explicit statements on "h"/"help" options is more convenient, as we have such commands.
        #[allow(clippy::wildcard_in_or_patterns)]
        match args.next() {
            Some("n") | Some("new") => self.new_service(args).await,
            Some("l") | Some("load") => self.load_module(args).await,
            Some("u") | Some("unload") => self.unload_module(args),
            Some("c") | Some("call") => self.call_module(args).await,
            Some("e") | Some("envs") => self.show_envs(args),
            Some("f") | Some("fs") => self.show_fs(args),
            Some("i") | Some("interface") => self.show_interface(),
            Some("s") | Some("stats") => self.show_memory_stats(),
            Some("q") | Some("quit") => {
                return false;
            }

            Some("h") | Some("help") | _ => print_help(),
        }

        true
    }

    async fn new_service<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        match Self::create_app_service(
            &self.app_service_factory,
            args.next(),
            self.service_working_dir.clone(),
            false,
        )
        .await
        {
            Ok(service) => self.app_service = service,
            Err(e) => println!("failed to create a new application service: {}", e),
        };
    }

    async fn load_module<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        next_argument!(module_name, args, "Module name should be specified");
        next_argument!(module_path, args, "Module path should be specified");

        let wasm_bytes = fs::read(module_path);
        if let Err(e) = wasm_bytes {
            println!("failed to read wasm module: {}", e);
            return;
        }

        let start = Instant::now();
        let config = MarineModuleConfig {
            logger_enabled: true,
            host_imports: Default::default(),
            wasi: Default::default(),
            logging_mask: Default::default(),
        };
        let result_msg = match self
            .app_service
            .load_module::<MarineModuleConfig, String>(
                module_name.into(),
                &wasm_bytes.unwrap(),
                Some(config),
            )
            .await
        {
            Ok(_) => {
                let elapsed_time = start.elapsed();
                format!(
                    "module successfully loaded into App service\nelapsed time: {:?}",
                    elapsed_time
                )
            }
            Err(e) => format!("loading failed with: {}", e),
        };
        println!("{}", result_msg);
    }

    fn unload_module<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        next_argument!(module_name, args, "Module name should be specified");

        let start = Instant::now();
        let result_msg = match self.app_service.unload_module(module_name) {
            Ok(_) => {
                let elapsed_time = start.elapsed();
                format!(
                    "module successfully unloaded from App service\nelapsed time: {:?}",
                    elapsed_time
                )
            }
            Err(e) => format!("unloading failed with: {}", e),
        };
        println!("{}", result_msg);
    }

    async fn call_module<'args>(&mut self, args: impl Iterator<Item = &'args str>) {
        let CallModuleArguments {
            module_name,
            func_name,
            show_result_arg,
            args,
            call_parameters,
        } = match parse_call_module_arguments(args) {
            Ok(call_module_arguments) => call_module_arguments,
            Err(message) => {
                println!("{}", message);
                return;
            }
        };

        let start = Instant::now();
        let call_future =
            self.app_service
                .call_module(module_name, func_name, args, call_parameters);
        let result = match tokio::time::timeout(self.timeout, call_future).await {
            Ok(Ok(result)) if show_result_arg => {
                let elapsed_time = start.elapsed();

                let result_string = match serde_json::to_string_pretty(&result) {
                    Ok(pretty_printed) => pretty_printed,
                    Err(_) => format!("{:?}", result),
                };

                format!(
                    "result: {}\n elapsed time: {:?}",
                    result_string, elapsed_time
                )
            }
            Ok(Ok(_)) => {
                let elapsed_time = start.elapsed();
                format!("call succeeded, elapsed time: {:?}", elapsed_time)
            }
            Ok(Err(e)) => format!("call failed with: {}", e),
            Err(elapsed) => format!("call interrupted: {} ({:#?})", elapsed, self.timeout),
        };

        println!("{}", result);
    }

    fn show_envs<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        next_argument!(module_name, args, "Module name should be specified");
        match self.app_service.get_wasi_state(module_name) {
            Ok(wasi_state) => print_envs(module_name, wasi_state.as_ref()),
            Err(e) => println!("{}", e),
        };
    }

    fn show_fs<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        next_argument!(module_name, args, "Module name should be specified");
        match self.app_service.get_wasi_state(module_name) {
            Ok(wasi_state) => print_fs_state(wasi_state.as_ref()),
            Err(e) => println!("{}", e),
        };
    }

    fn show_interface(&mut self) {
        let interface = self.app_service.get_full_interface();

        print!("Loaded modules interface:\n{}", interface);
    }

    fn show_memory_stats(&self) {
        let statistic = self.app_service.module_memory_stats();

        print!("Loaded modules heap sizes:\n{}", statistic);
    }

    async fn create_app_service<S: Into<PathBuf>>(
        app_service_factory: &AppServiceFactory,
        config_file_path: Option<S>,
        working_dir: Option<String>,
        quiet: bool,
    ) -> ReplResult<AppService> {
        let service_id = uuid::Uuid::new_v4().to_string();
        let config_file_path: Option<PathBuf> = config_file_path.map(Into::into);
        let working_dir = working_dir.unwrap_or_else(|| ".".to_string());

        let start = Instant::now();

        let mut config = config_file_path
            .as_ref()
            .map(TomlAppServiceConfig::load)
            .transpose()
            .map_err(|e| {
                anyhow!(
                    "failed to load \"{}\": {}",
                    config_file_path
                        .as_ref()
                        .unwrap_or_else(|| panic!(
                            "config_file_path is Some because it is used to load file"
                        ))
                        .display(),
                    e
                )
            })?
            .unwrap_or_default();

        config.service_working_dir = Some(working_dir);

        config.toml_marine_config.base_path = config_file_path
            .and_then(|path| path.parent().map(PathBuf::from))
            .unwrap_or_default();

        let config = config.try_into()?;

        let app_service = app_service_factory
            .new_app_service_empty_facade(config, &service_id, HashMap::new())
            .await?;

        let duration = start.elapsed();

        if !quiet {
            println!(
                "app service was created with service id = {}\nelapsed time {:?}",
                service_id, duration
            );
        }

        Ok(app_service)
    }

    fn spawn_ticker_thread(ticker: fluence_app_service::EpochTicker) {
        std::thread::spawn(move || {
            let period = std::time::Duration::from_millis(10);

            loop {
                std::thread::sleep(period);
                ticker.increment_epoch();
            }
        });
    }
}

#[derive(Clone, PartialEq, Default, Eq, Debug, Deserialize)]
struct PartialParticleParameters {
    /// Id of the particle which execution resulted a call this service.
    #[serde(default)]
    pub id: String,

    /// Peer id of the AIR script initiator.
    #[serde(default)]
    pub init_peer_id: String,

    /// Unix timestamp of the particle start time.
    #[serde(default)]
    pub timestamp: u64,

    /// Time to live for this particle in milliseconds.
    #[serde(default)]
    pub ttl: u32,

    /// AIR script in this particle.
    #[serde(default)]
    pub script: String,

    /// Signature made by particle initiator -- init_peer_id.
    #[serde(default)]
    pub signature: Vec<u8>,

    /// particle.signature signed by host_id -- used for FS access.
    #[serde(default)]
    pub token: String,
}

#[derive(Clone, PartialEq, Default, Eq, Debug, Deserialize)]
struct PartialCallParameters {
    /// Peer id of the AIR script initiator.
    #[serde(default)]
    pub particle: PartialParticleParameters,

    /// Id of the current service.
    #[serde(default)]
    pub service_id: String,

    /// Id of the service creator.
    #[serde(default)]
    pub service_creator_peer_id: String,

    /// PeerId of the peer who hosts worker with this service.
    #[serde(default)]
    pub host_id: String,

    /// PeerId of the worker who hosts this service.
    #[serde(default)]
    pub worker_id: String,

    /// Security tetraplets which described origin of the arguments.
    #[serde(default)]
    pub tetraplets: Vec<Vec<SecurityTetraplet>>,
}

impl From<PartialCallParameters> for CallParameters {
    fn from(partial_call_params: PartialCallParameters) -> Self {
        let PartialCallParameters {
            particle,
            service_id,
            service_creator_peer_id,
            host_id,
            worker_id,
            tetraplets,
        } = partial_call_params;

        Self {
            particle: ParticleParameters {
                id: particle.id,
                init_peer_id: particle.init_peer_id,
                timestamp: particle.timestamp,
                ttl: particle.ttl,
                script: particle.script,
                signature: particle.signature,
                token: particle.token,
            },
            service_id,
            service_creator_peer_id,
            host_id,
            worker_id,
            tetraplets,
        }
    }
}

fn parse_call_module_arguments<'args>(
    args: impl Iterator<Item = &'args str>,
) -> Result<CallModuleArguments<'args>, String> {
    use itertools::Itertools;

    let mut args = args.peekable();
    next_argument_or_result!(module_name, args, "Module name should be specified");
    next_argument_or_result!(func_name, args, "Function name should be specified");
    let show_result_arg = match args.peek() {
        Some(option) if *option == "-nr" => {
            args.next();
            false
        }
        Some(_) => true,
        None => true,
    };

    let module_arg: String = args.join(" ");
    let mut de = serde_json::Deserializer::from_str(&module_arg);

    let args = match JValue::deserialize(&mut de) {
        Ok(args) => args,
        Err(e) => return Err(format!("invalid args: {}", e)),
    };

    let call_parameters = match de.end() {
        Ok(_) => CallParameters::default(),
        Err(_) => match PartialCallParameters::deserialize(&mut de) {
            Ok(call_parameters) => call_parameters.into(),
            Err(e) => return Err(format!("invalid call parameters: {}", e)),
        },
    };

    if de.end().is_err() {
        return Err(String::from(
            "trailing characters after call parameters are not supported",
        ));
    }

    Ok(CallModuleArguments {
        module_name,
        func_name,
        show_result_arg,
        args,
        call_parameters,
    })
}

fn print_help() {
    println!(
        "Commands:\n\n\
            n/new [config_path]                                   create a new service (current will be removed)\n\
            l/load <module_name> <module_path>                    load a new Wasm module\n\
            u/unload <module_name>                                unload a Wasm module\n\
            c/call <module_name> <func_name> <args> [call_params] call function with given name from given module\n\
            i/interface                                           print public interface of all loaded modules\n\
            s/stats                                               print memory size of all loaded modules\n\
            e/envs <module_name>                                  print environment variables of a module\n\
            f/fs <module_name>                                    print filesystem state of a module\n\
            s/stats                                               print consumed memory size of each module\n\
            h/help                                                print this message\n\
            q/quit/Ctrl-C                                         exit\n\
            \n\
            <args> and [call_params] should be in json"
    );
}
