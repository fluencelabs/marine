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

use crate::ReplResult;

use fluence_app_service::AppService;
use fluence_app_service::TomlAppServiceConfig;
use print_state::print_envs;
use print_state::print_fs_state;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

macro_rules! next_argument {
    ($arg_name:ident, $args:ident, $error_msg:expr) => {
        let $arg_name = if let Some($arg_name) = $args.next() {
            $arg_name
        } else {
            println!($error_msg);
            return true;
        };
    };
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
    pub fn execute<'a>(&mut self, mut args: impl Iterator<Item = &'a str>) -> bool {
        match args.next() {
            Some("new") => {
                match Self::create_app_service(args.next()) {
                    Ok(service) => self.app_service = service,
                    Err(e) => println!("failed to create a new application service: {}", e),
                };
            }
            Some("load") => {
                next_argument!(module_name, args, "Module name should be specified");
                next_argument!(module_path, args, "Module path should be specified");

                let wasm_bytes = fs::read(module_path);
                if let Err(e) = wasm_bytes {
                    println!("failed to read wasm module: {}", e);
                    return true;
                }

                let start = Instant::now();
                let result_msg = match self
                    .app_service
                    .load_module::<String, fluence_app_service::FaaSModuleConfig>(
                        module_name.into(),
                        &wasm_bytes.unwrap(),
                        None,
                    ) {
                    Ok(_) => {
                        let elapsed_time = start.elapsed();
                        format!(
                            "module successfully loaded into App service\nelapsed time: {:?}",
                            elapsed_time
                        )
                    }
                    Err(e) => format!("module loaded failed with: {:?}", e),
                };
                println!("{}", result_msg);
            }
            Some("unload") => {
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
                    Err(e) => format!("module unloaded failed with: {:?}", e),
                };
                println!("{}", result_msg);
            }
            Some("call") => {
                use itertools::Itertools;

                next_argument!(module_name, args, "Module name should be specified");
                next_argument!(func_name, args, "Function name should be specified");

                let module_arg: String = args.join(" ");
                let module_arg: serde_json::Value = match serde_json::from_str(&module_arg) {
                    Ok(module_arg) => module_arg,
                    Err(e) => {
                        println!("incorrect arguments {}", e);
                        return true;
                    }
                };

                let start = Instant::now();
                // TODO: add support of call parameters
                let result = match self.app_service.call_with_module_name(
                    module_name,
                    func_name,
                    module_arg,
                    <_>::default(),
                ) {
                    Ok(result) => {
                        let elapsed_time = start.elapsed();
                        format!("result: {:?}\n elapsed time: {:?}", result, elapsed_time)
                    }
                    Err(e) => format!("execution failed with {:?}", e),
                };
                println!("{}", result);
            }
            Some("envs") => {
                next_argument!(module_name, args, "Module name should be specified");
                match self.app_service.get_wasi_state(module_name) {
                    Ok(wasi_state) => print_envs(module_name, wasi_state),
                    Err(e) => println!("{}", e),
                };
            }
            Some("fs") => {
                next_argument!(module_name, args, "Module name should be specified");
                match self.app_service.get_wasi_state(module_name) {
                    Ok(wasi_state) => print_fs_state(wasi_state),
                    Err(e) => println!("{}", e),
                };
            }
            Some("interface") => {
                let interface = self.app_service.get_full_interface();
                print!("Application service interface:\n{}", interface);
            }
            Some("q") | Some("quit") => {
                return false;
            }

            _ => print_help(),
        }

        true
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
            "app service's created with service id = {}\nelapsed time {:?}",
            service_id, duration
        );

        Ok(app_service)
    }
}

fn print_help() {
    println!(
        "Commands:\n\
            new [config_path]                       create new service (current will be removed)\n\
            load <module_name> <module_path>        load new Wasm module\n\
            unload <module_name>                    unload Wasm module\n\
            call <module_name> <func_name> [args]   call function with given name from given module\n\
            interface                               print public interfaces of all loaded modules\n\
            envs <module_name>                      print environment variables of a module\n\
            fs <module_name>                        print filesystem state of a module\n\
            h/help                                  print this message\n\
            q/quit/Ctrl-C                           exit"
    );
}
