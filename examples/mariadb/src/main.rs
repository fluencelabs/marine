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

mod path;

use fluence::fce;
use fluence::WasmLogger;
use crate::path::to_full_path;

use std::path::PathBuf;

const PASSWORD_ENV_NAME: &str = "PASSWORD";
const DB_NAME_ENV_NAME: &str = "DB_NAME";
const PORT_ENV_NAME: &str = "PORT";
const USER_ENV_NAME: &str = "USER";
const HOST_ENV_NAME: &str = "HOST_ENV_NAME";

const TMP_FILE_NAME: &str = "mariadb.sql";

pub fn main() {
    WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[fce]
pub struct ExecutionResult {
    // Contain execution result if error_code is 0, and empty string or error message otherwise
    pub result: String,

    // 0 is success, otherwise error code
    pub error_code: i32,
}

impl ExecutionResult {
    pub fn success(result: String) -> Self {
        Self {
            result,
            error_code: 0,
        }
    }
    pub fn error(error_code: i32) -> Self {
        Self {
            result: String::new(),
            error_code,
        }
    }
}

#[fce]
pub fn sql(sql: String) -> ExecutionResult {
    log::info!("sql called with command {}", sql);

    let sql_cmd_filepath = TMP_FILE_NAME.to_string();

    if let Err(e) = std::fs::write(PathBuf::from(sql_cmd_filepath), sql) {
        return ExecutionResult {
            result: String::new(),
            // it is safe because write return only OS errors
            error_code: e.raw_os_error().unwrap(),
        };
    }

    let sql_cmd_filepath = to_full_path(TMP_FILE_NAME);

    let password = std::env::var(PASSWORD_ENV_NAME).unwrap_or_else(|_| "toor".to_string());
    let db_name = std::env::var(DB_NAME_ENV_NAME).unwrap_or_else(|_| "test".to_string());
    let port = std::env::var(PORT_ENV_NAME).unwrap_or_else(|_| "3306".to_string());
    let user = std::env::var(USER_ENV_NAME).unwrap_or_else(|_| "root".to_string());
    let host = std::env::var(HOST_ENV_NAME).unwrap_or_else(|_| "127.0.0.1".to_string());

    let cmd = format!(
        "-u{} -p{} -h{} -P{} -D{} < {}",
        user, password, host, port, db_name, sql_cmd_filepath
    );

    let result = mariadb(cmd);
    ExecutionResult::success(result)
}

#[fce]
#[link(wasm_import_module = "host")]
extern "C" {
    /// Execute provided sql as a parameters of mariadb cli, return result.
    pub fn mariadb(sql: String) -> String;
}
