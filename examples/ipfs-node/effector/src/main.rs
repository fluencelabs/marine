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

mod path;

use crate::path::to_full_path;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use marine_rs_sdk::MountedBinaryResult;

const RESULT_FILE_PATH: &str = "/tmp/ipfs_rpc_file";
const IPFS_ADDR_ENV_NAME: &str = "IPFS_ADDR";
const TIMEOUT_ENV_NAME: &str = "timeout";

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

/// Put file from specified path to IPFS and return its hash.
#[marine]
pub fn put(file_path: String) -> String {
    log::info!("put called with file path {}", file_path);

    let file_path = to_full_path(file_path);

    let timeout = std::env::var(TIMEOUT_ENV_NAME).unwrap_or_else(|_| "1s".to_string());
    let cmd = vec![
        String::from("add"),
        String::from("--timeout"),
        timeout,
        String::from("-Q"),
        file_path,
    ];

    let ipfs_result = ipfs(cmd);
    ipfs_result
        .into_std()
        .unwrap()
        .unwrap_or_else(std::convert::identity)
}

/// Get file by provided hash from IPFS, saves it to a temporary file and returns a path to it.
#[marine]
pub fn get(hash: String) -> String {
    log::info!("get called with hash {}", hash);

    let result_file_path = to_full_path(RESULT_FILE_PATH);

    let timeout = std::env::var(TIMEOUT_ENV_NAME).unwrap_or_else(|_| "1s".to_string());
    let cmd = vec![
        String::from("get"),
        String::from("--timeout"),
        timeout,
        String::from("-o"),
        result_file_path,
        hash,
    ];

    ipfs(cmd);
    RESULT_FILE_PATH.to_string()
}

#[marine]
pub fn get_address() -> String {
    match std::env::var(IPFS_ADDR_ENV_NAME) {
        Ok(addr) => addr,
        Err(e) => format!(
            "getting {} env variable failed with error {:?}",
            IPFS_ADDR_ENV_NAME, e
        ),
    }
}

#[marine]
#[host_import]
extern "C" {
    /// Execute provided cmd as a parameters of ipfs cli, return result.
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}
