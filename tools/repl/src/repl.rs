/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod print_state;

use print_state::print_envs;
use print_state::print_fs_state;
use crate::ReplResult;

use fluence_app_service::{AppService, CallParameters};
use fluence_app_service::FaaSModuleConfig;
use fluence_app_service::TomlAppServiceConfig;

use serde::Deserialize;

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
            None => return Err(String::from($error_msg))
        };
    };
}

struct CallModuleArguments<'args> {
    module_name: &'args str,
    func_name: &'args str,
    show_result_arg: bool,
    args: serde_json::Value,
    call_parameters: CallParameters,
}

pub(super) struct REPL {
    app_service: AppService,
}

impl REPL {
    pub fn new<S: Into<PathBuf>>(config_file_path: Option<S>) -> ReplResult<Self> {
        let app_service = Self::create_app_service(config_file_path)?;
        Ok(Self { app_service })
    }

    /// Returns true, it should be the last executed command.
    pub fn execute<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) -> bool {
        match args.next() {
            Some("n") | Some("new") => self.new_service(args),
            Some("l") | Some("load") => self.load_module(args),
            Some("u") | Some("unload") => self.unload_module(args),
            Some("c") | Some("call") => self.call_module(args),
            Some("e") | Some("envs") => self.show_envs(args),
            Some("f") | Some("fs") => self.show_fs(args),
            Some("i") | Some("interface") => self.show_interface(),
            Some("q") | Some("quit") => {
                return false;
            }

            _ => print_help(),
        }

        true
    }

    fn new_service<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        match Self::create_app_service(args.next()) {
            Ok(service) => self.app_service = service,
            Err(e) => println!("failed to create a new application service: {}", e),
        };
    }

    fn load_module<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        next_argument!(module_name, args, "Module name should be specified");
        next_argument!(module_path, args, "Module path should be specified");

        let wasm_bytes = fs::read(module_path);
        if let Err(e) = wasm_bytes {
            println!("failed to read wasm module: {}", e);
            return;
        }

        let start = Instant::now();
        let config = FaaSModuleConfig {
            logger_enabled: true,
            ..<_>::default()
        };
        let result_msg = match self
            .app_service
            .load_module::<String, fluence_app_service::FaaSModuleConfig>(
                module_name.into(),
                &wasm_bytes.unwrap(),
                Some(config),
            ) {
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

    fn call_module<'args>(&mut self, args: impl Iterator<Item = &'args str>) {
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
        let result =
            match self
                .app_service
                .call_module(module_name, func_name, args, call_parameters)
            {
                Ok(result) if show_result_arg => {
                    let elapsed_time = start.elapsed();
                    format!("result: {:?}\n elapsed time: {:?}", result, elapsed_time)
                }
                Ok(_) => {
                    let elapsed_time = start.elapsed();
                    format!("call succeeded, elapsed time: {:?}", elapsed_time)
                }
                Err(e) => format!("call failed with: {}", e),
            };

        println!("{}", result);
    }

    fn show_envs<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        next_argument!(module_name, args, "Module name should be specified");
        match self.app_service.get_wasi_state(module_name) {
            Ok(wasi_state) => print_envs(module_name, wasi_state),
            Err(e) => println!("{}", e),
        };
    }

    fn show_fs<'args>(&mut self, mut args: impl Iterator<Item = &'args str>) {
        next_argument!(module_name, args, "Module name should be specified");
        match self.app_service.get_wasi_state(module_name) {
            Ok(wasi_state) => print_fs_state(wasi_state),
            Err(e) => println!("{}", e),
        };
    }

    fn show_interface(&mut self) {
        let interface = self.app_service.get_full_interface();

        print!("Loaded modules interface:\n{}", interface);
    }

    fn create_app_service<S: Into<PathBuf>>(config_file_path: Option<S>) -> ReplResult<AppService> {
        let tmp_path: String = std::env::temp_dir().to_string_lossy().into();
        let service_id = uuid::Uuid::new_v4().to_string();

        let start = Instant::now();

        let mut config = config_file_path
            .map(|p| TomlAppServiceConfig::load(p.into()))
            .transpose()?
            .unwrap_or_default();
        config.service_base_dir = Some(tmp_path);

        let app_service = AppService::new_with_empty_facade(config, &service_id, HashMap::new())?;

        let duration = start.elapsed();

        println!(
            "app service was created with service id = {}\nelapsed time {:?}",
            service_id, duration
        );

        Ok(app_service)
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

    let args = match serde_json::Value::deserialize(&mut de) {
        Ok(args) => args,
        Err(e) => return Err(format!("invalid args: {}", e)),
    };

    let call_parameters = match de.end() {
        Ok(_) => CallParameters::default(),
        Err(_) => match CallParameters::deserialize(&mut de) {
            Ok(call_parameters) => call_parameters,
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
            e/envs <module_name>                                  print environment variables of a module\n\
            f/fs <module_name>                                    print filesystem state of a module\n\
            h/help                                                print this message\n\
            q/quit/Ctrl-C                                         exit\n\
            \n\
            <args> and [call_params] should be in json"
    );
}
