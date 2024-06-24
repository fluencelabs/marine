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

#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use std::fs;
use std::path::PathBuf;

const RPC_TMP_FILEPATH: &str = "/tmp/ipfs_rpc_file";

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn invoke() -> String {
    "IPFS_RPC wasm example, it allows to:\ninvoke\nput\nget".to_string()
}

#[marine]
pub fn put(file_content: Vec<u8>) -> String {
    log::info!("put called with {:?}", file_content);

    let rpc_tmp_filepath = RPC_TMP_FILEPATH.to_string();

    let r = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
    if let Err(e) = r {
        return format!("file can't be written: {}", e);
    }

    ipfs_put(rpc_tmp_filepath)
}

#[marine]
pub fn get(hash: String) -> Vec<u8> {
    log::info!("get called with hash: {}", hash);

    let file_path = ipfs_get(hash);
    fs::read(file_path).unwrap_or_else(|_| b"error while reading file".to_vec())
}

#[marine]
#[module_import("ipfs_effector")]
extern "C" {
    /// Put provided file to ipfs, return ipfs hash of the file.
    #[link_name = "put"]
    pub fn ipfs_put(file_path: String) -> String;

    /// Get file from ipfs by hash.
    #[link_name = "get"]
    pub fn ipfs_get(hash: String) -> String;
}
