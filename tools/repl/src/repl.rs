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

use crate::Result;

use fluence_app_service::AppService;
use fluence_app_service::TomlAppServiceConfig;

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
            return;
        };
    };
}

pub(super) struct REPL {
    app_service: AppService,
}

impl REPL {
    pub fn new<S: Into<PathBuf>>(config_file_path: Option<S>) -> Result<Self> {
        let app_service = Self::create_app_service(config_file_path)?;
        Ok(Self { app_service })
    }

    pub fn execute<'a>(&mut self, mut args: impl Iterator<Item = &'a str>) {
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
                    return;
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
                        return;
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
                    Ok(wasi_state) => Self::print_envs(wasi_state),
                    Err(e) => println!("{}", e),
                };
            }
            Some("fs") => {
                next_argument!(module_name, args, "Module name should be specified");
                match self.app_service.get_wasi_state(module_name) {
                    Ok(wasi_state) => Self::print_fs_state(wasi_state),
                    Err(e) => println!("{}", e),
                };
            }
            Some("interface") => {
                let interface = self.app_service.get_interface();
                print!("Application service interface:\n{}", interface);
            }
            Some("h") | Some("help") | None => {
                println!(
                    "Enter:\n\
                                new [config_path]                       - to create a new AppService (current will be removed)\n\
                                load <module_name> <module_path>        - to load a new Wasm module into App service\n\
                                unload <module_name>                    - to unload Wasm module from AppService\n\
                                call <module_name> <func_name> [args]   - to call function with func_name of module with module_name\n\
                                interface                               - to print public interface of current AppService\n\
                                envs <module_name>                      - to print environment variables of module with module_name\n\
                                fs <module_name>                        - to print filesystem state of module with module_name\n\
                                h/help                                  - to print this message\n\
                                Ctrl-C                                  - to exit"
                );
            }
            _ => {
                println!("unsupported command");
            }
        }
    }

    fn create_app_service<S: Into<PathBuf>>(config_file_path: Option<S>) -> Result<AppService> {
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

    fn print_envs(wasi_state: &wasmer_wasi::state::WasiState) {
        let envs = &wasi_state.envs;

        println!("Environment variables:");
        for env in envs.iter() {
            match String::from_utf8(env.clone()) {
                Ok(string) => println!("{}", string),
                Err(_) => println!("{:?}", env),
            }
        }
    }

    fn print_fs_state(wasi_state: &wasmer_wasi::state::WasiState) {
        let wasi_fs = &wasi_state.fs;

        println!("preopened file descriptors:\n{:?}\n", wasi_fs.preopen_fds);

        println!("name map:");
        for (name, inode) in &wasi_fs.name_map {
            println!("{} - {:?}", name, inode);
        }

        println!("\nfile descriptors map:");
        for (id, fd) in &wasi_fs.fd_map {
            println!("{} - {:?}", id, fd);
        }

        println!("\norphan file descriptors:");
        for (fd, inode) in &wasi_fs.orphan_fds {
            println!("{:?} - {:?}", fd, inode);
        }

        println!("\ninodes:");
        for (id, inode) in wasi_fs.inodes.iter().enumerate() {
            println!("{}: {:?}", id, inode);
        }
    }
}
