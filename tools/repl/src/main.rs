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

#![deny(
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]

/// Command-line tool intended to test Fluence FaaS.
use std::fs;

macro_rules! next_argument {
    ($arg_name:ident, $args:ident, $error_msg:expr) => {
        let $arg_name = if let Some($arg_name) = $args.next() {
            $arg_name
        } else {
            println!($error_msg);
            continue;
        };
    };
}

fn main() -> Result<(), anyhow::Error> {
    let (args, _) = rustop::opts! {
        synopsis "Fluence Application service REPL";
        param config_file_path: Option<String>, desc: "Path to a service config";
    }
    .parse_or_exit();

    println!("Welcome to the Fluence FaaS REPL:");
    let mut app_service = create_service_from_config(args.config_file_path)?;

    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        let readline = match readline {
            Ok(readline) => readline,
            Err(e) => {
                println!("a error occurred: {}", e);
                break;
            }
        };

        let mut args = readline.split_whitespace();
        match args.next() {
            Some("new") => {
                app_service = match create_service_from_config(args.next()) {
                    Ok(service) => service,
                    Err(e) => {
                        println!("failed to create a new application service: {}", e);
                        app_service
                    }
                };
            }
            Some("load") => {
                next_argument!(module_name, args, "Module name should be specified");
                next_argument!(module_path, args, "Module path should be specified");

                let wasm_bytes = fs::read(module_path);
                if let Err(e) = wasm_bytes {
                    println!("failed to read wasm module: {}", e);
                    continue;
                }

                let result_msg = match app_service
                    .load_module::<String, fluence_app_service::ModuleConfig>(
                        module_name.into(),
                        &wasm_bytes.unwrap(),
                        None,
                    ) {
                    Ok(_) => "module successfully loaded into App service".to_string(),
                    Err(e) => format!("module loaded failed with: {:?}", e),
                };
                println!("{}", result_msg);
            }
            Some("unload") => {
                next_argument!(module_name, args, "Module name should be specified");

                let result_msg = match app_service.unload_module(module_name) {
                    Ok(_) => "module successfully unloaded from App service".to_string(),
                    Err(e) => format!("module unloaded failed with: {:?}", e),
                };
                println!("{}", result_msg);
            }
            Some("call") => {
                next_argument!(module_name, args, "Module name should be specified");
                next_argument!(func_name, args, "Function name should be specified");

                let module_arg: String = args.collect();
                let module_arg: serde_json::Value = match serde_json::from_str(&module_arg) {
                    Ok(module_arg) => module_arg,
                    Err(e) => {
                        println!("incorrect arguments {}", e);
                        continue;
                    }
                };

                let result = match app_service.call(module_name, func_name, module_arg) {
                    Ok(result) => format!("result: {:?}", result),
                    Err(e) => format!("execution failed with {:?}", e),
                };
                println!("{}", result);
            }
            Some("interface") => {
                let interface = app_service.get_interface();
                println!("application service interface: {}", interface);
            }
            Some("h") | Some("help") | None => {
                println!(
                            "Enter:\n\
                                new [config_path]                       - to create a new AppService (old will be removed)
                                load <module_name> <module_path>        - to load a new Wasm module into App service\n\
                                unload <module_name>                    - to unload Wasm module from AppService\n\
                                call <module_name> <func_name> [args]   - to call function with given name on module with given module_name\n\
                                interface                               - to print public interface of current AppService\n\
                                h/help                                  - to print this message\n\
                                e/exit/q/quit                           - to exit"
                        );
            }
            Some("e") | Some("exit") | Some("q") | Some("quit") => break,
            _ => {
                println!("unsupported command");
            }
        }
    }

    Ok(())
}

fn create_service_from_config<S: Into<String>>(
    config_file_path: Option<S>,
) -> Result<fluence_app_service::AppService, anyhow::Error> {
    let tmp_path: String = std::env::temp_dir().to_string_lossy().into();
    let service_id = uuid::Uuid::new_v4().to_string();

    let app_service = match config_file_path {
        Some(config_file_path) => {
            let config_file_path = config_file_path.into();
            fluence_app_service::AppService::with_raw_config(
                config_file_path,
                &service_id,
                Some(&tmp_path),
            )
        }
        None => {
            let mut config: fluence_app_service::RawModulesConfig = <_>::default();
            config.service_base_dir = Some(tmp_path);

            fluence_app_service::AppService::new(std::iter::empty(), config, &service_id)
        }
    }?;

    println!("app service's created with service id = {}", service_id);

    Ok(app_service)
}
