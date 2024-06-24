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
        .filter(Some("wasmtime_wasi"), Info)
        //.filter(Some(WIT_MODULE_PATH), Info)
        // the same for rustyline and marine
        .filter(Some("rustyline"), Info)
        .filter(Some("marine"), Info)
        .init();
}
