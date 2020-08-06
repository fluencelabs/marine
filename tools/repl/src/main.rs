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

fn main() -> Result<(), anyhow::Error> {
    println!("Welcome to the Fluence FaaS REPL:");
    let mut rl = rustyline::Editor::<()>::new();
    let mut app_service = fluence_faas_service::FluenceFaaSService::default();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // TODO: improve argument parsing
                let cmd: Vec<_> = line.split(' ').collect();
                match cmd[0] {
                    "load" => {
                        let module_name = cmd[1].to_string();
                        let wasm_bytes = fs::read(cmd[2]);
                        if let Err(e) = wasm_bytes {
                            println!("failed to read wasm module: {}", e);
                            continue;
                        }

                        let result_msg = match app_service.load_module(
                            module_name,
                            &wasm_bytes.unwrap(),
                            None,
                        ) {
                            Ok(_) => "module successfully loaded into App service".to_string(),
                            Err(e) => format!("module loaded failed with: {:?}", e),
                        };
                        println!("{}", result_msg);
                    }
                    "unload" => {
                        let module_name = cmd[1];
                        let result_msg = match app_service.unload_module(module_name) {
                            Ok(_) => "module successfully unloaded from App service".to_string(),
                            Err(e) => format!("module unloaded failed with: {:?}", e),
                        };
                        println!("{}", result_msg);
                    }
                    "call" => {
                        let module_name = cmd[1];
                        let func_name = cmd[2];
                        let arg = cmd[3..].join(" ");
                        let result = match app_service.call(module_name, func_name, arg.as_bytes()) {
                            Ok(result) => format!("result: {:?}", result),
                            Err(e) => format!("execution failed with {:?}", e),
                        };
                        println!("{}", result);
                    }
                    "help" => {
                        println!(
                            "Enter:\n\
                                load <module_name> <module_path>        - to load a new Wasm module into App service\n\
                                unload <module_name>                    - to unload Wasm module from AppService\n\
                                call <module_name> <func_name> <args>   - to call function with given name on module with given module_name\n\
                                help                                    - to print this message\n\
                                e/exit/q/quit                           - to exit"
                        );
                    }
                    "e" | "exit" | "q" | "quit" => break,
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
