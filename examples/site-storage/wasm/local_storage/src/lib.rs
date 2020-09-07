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

use std::fs;
use fluence::fce;
use fluence::WasmLogger;
use std::path::PathBuf;

const RPC_TMP_FILEPATH: &str = "/tmp/";

pub fn main() {
    WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[fce]
pub fn put(name: String, file_content: Vec<u8>) -> String {
    log::info!("put called with {:?}", file_content);

    let rpc_tmp_filepath = format!("{}{}", RPC_TMP_FILEPATH, name);
    let read_filepath = format!("{}alala", RPC_TMP_FILEPATH);
    let res = fs::read(read_filepath);
    log::info!("read {:?}", res);
    let r = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
    if let Err(e) = r {
        return format!("file can't be written: {}", e);
    }

    return "Ok".to_string()
}

#[fce]
pub fn get(file_name: String) -> Vec<u8> {
    log::info!("get called with file name: {}", file_name);

    let tmp_filepath = format!("{}{}", RPC_TMP_FILEPATH, file_name);

    fs::read(tmp_filepath).unwrap_or_else(|_| b"error while reading file".to_vec())
}
