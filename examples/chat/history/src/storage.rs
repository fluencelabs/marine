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

use fluence::fce;

pub fn init() {
    unsafe {
        invoke("CREATE TABLE IF NOT EXISTS history(msg_id INTEGER PRIMARY KEY, msg TEXT NOT NULL, author TEXT NOT NULL);".to_string());
    }
}

pub fn add_msg(msg: String, author: String) -> String {
    unsafe {
        invoke(format!(
            "INSERT INTO history (msg,author) VALUES ('{}','{}')",
            msg, author
        ))
    }
}

pub fn get_msg(limit: u64) -> String {
    unsafe {
        invoke(format!(
            "SELECT * FROM history ORDER BY msg_id DESC LIMIT '{}';",
            limit
        ))
    }
}

pub fn get_all_msgs() -> String {
    unsafe { invoke(format!("SELECT * FROM history;")) }
}

#[fce]
#[link(wasm_import_module = "sqlite")]
extern "C" {
    #[link_name = "invoke"]
    pub fn invoke(cmd: String) -> String;

}
