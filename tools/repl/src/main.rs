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
    println!("Welcome to the Fluence FaaS REPL:");
    let mut rl = rustyline::Editor::<()>::new();
    let mut app_service = fluence_faas_service::FluenceFaaSService::default();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // TODO: improve argument parsing
                let mut args = line.split(' ');
                match args.next() {
                    Some("load") => {
                        next_argument!(module_name, args, "Module name should be specified");
                        next_argument!(module_path, args, "Module path should be specified");

                        let wasm_bytes = fs::read(module_path);
                        if let Err(e) = wasm_bytes {
                            println!("failed to read wasm module: {}", e);
                            continue;
                        }

                        let result_msg = match app_service.load_module::<String, fluence_faas_service::ModuleConfig>(
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
                            },
                        };

                        let result = match app_service.call(module_name, func_name, module_arg)
                        {
                            Ok(result) => format!("result: {:?}", result),
                            Err(e) => format!("execution failed with {:?}", e),
                        };
                        println!("{}", result);
                    }
                    Some("help") | None => {
                        println!(
                            "Enter:\n\
                                load <module_name> <module_path>        - to load a new Wasm module into App service\n\
                                unload <module_name>                    - to unload Wasm module from AppService\n\
                                call <module_name> <func_name> <args>   - to call function with given name on module with given module_name\n\
                                help                                    - to print this message\n\
                                e/exit/q/quit                           - to exit"
                        );
                    }
                    Some("e") | Some("exit") | Some("q") | Some("quit") => break,
                    _ => {
                        println!("unsupported command");
                    }
                }
            }
            Err(e) => {
                println!("a error occurred: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/*
fn call_function_command(
    app_service: &mut FluenceFaaSService,
    args: &[String],
) -> Result<serde_json::Value, String> {
    let module_name = &args[0];
    let func_name = &args[1];
    let module_interfaces = app_service
        .get_interface()
        .modules
        .get(module_name)
        .ok_or_else(|| Err(format!("{} not found in app service", module_name)))?;
    let function_signature = module_interfaces
        .get(module_name)
        .ok_or_else(|| Err(format!("{} not found in {} module", func_name, module_name)))?;

    //    let arguments = serde_json::from
}
*/
