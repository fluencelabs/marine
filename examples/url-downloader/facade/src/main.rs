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
#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

/// Combining of modules: `curl` and `local_storage`.
/// Calls `curl` and stores returned result into a file.
#[marine]
pub fn get_n_save(url: String, file_name: String) -> String {
    log::info!("get_n_save called with {} {}\n", url, file_name);

    let result = download(url);
    file_put(file_name, result.into_bytes());

    String::from("Ok")
}

/// Importing `curl` module
#[marine]
#[module_import("curl_adapter")]
extern "C" {
    pub fn download(url: String) -> String;
}

/// Importing `local_storage` module
#[marine]
#[module_import("local_storage")]
extern "C" {
    #[link_name = "get"]
    pub fn file_get(file_name: String) -> Vec<u8>;

    #[link_name = "put"]
    pub fn file_put(name: String, file_content: Vec<u8>) -> String;
}
