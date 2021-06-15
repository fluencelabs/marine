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

use marine_rs_sdk_main::WASM_LOG_ENV_NAME;

use std::io::Write;
use std::env::var;

const IT_MODULE_PATH: &str = "wasmer_interface_types_fl";
const RUST_LOG_ENV_NAME: &str = "RUST_LOG";

pub(super) fn init_logger() {
    use log::LevelFilter::Info;

    match (var(RUST_LOG_ENV_NAME), var(WASM_LOG_ENV_NAME)) {
        (Ok(_), _) => {}
        (Err(_), Ok(wasm_log_env)) if !wasm_log_env.starts_with("off") => {
            std::env::set_var(RUST_LOG_ENV_NAME, "trace")
        }
        _ => return,
    };

    env_logger::builder()
        .format(|buf, record| {
            match record.module_path() {
                Some(module_path) if module_path.starts_with(IT_MODULE_PATH) => {
                    writeln!(buf, "[host] {}", record.args())
                }
                // due to the log_utf8_string implementation,
                // a log message from a Wasm module always has module path
                None => writeln!(buf, "[host] {}", record.args()),
                Some(module_path) => writeln!(buf, "[{}] {}", module_path, record.args()),
            }
        })
        // set a default level Info for Wasmer components
        .filter(Some("cranelift_codegen"), Info)
        .filter(Some("wasmer_wasi"), Info)
        //.filter(Some(WIT_MODULE_PATH), Info)
        // the same for rustyline and marine
        .filter(Some("rustyline"), Info)
        .filter(Some("marine"), Info)
        .init();
}
