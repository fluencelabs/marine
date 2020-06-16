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
#![allow(clippy::missing_safety_doc)]

mod mem;
mod result;

use crate::result::{RESULT_PTR, RESULT_SIZE};
use std::fs;
use std::path::PathBuf;

const RPC_TMP_FILEPATH: &str = "/tmp/ipfs_rpc_file";

pub unsafe fn main() {
    let msg = "ipfs_rpc.main: WASI initialization finished, env {}";
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);
}

#[no_mangle]
pub unsafe fn invoke(_ptr: *mut u8, _size: usize) {
    let msg = "ipfs_rpc.invoke: invoke called\n";
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    let result = "IFPFS_RPC wasm example, it allows to:\ninvoke\nput\nget".to_string();

    *RESULT_PTR.get_mut() = result.as_ptr() as _;
    *RESULT_SIZE.get_mut() = result.len();
    std::mem::forget(result);
}

#[no_mangle]
pub unsafe fn put(file_content_ptr: *mut u8, file_content_size: usize) {
    let file_content =
        String::from_raw_parts(file_content_ptr, file_content_size, file_content_size);

    let msg = format!("ipfs_rpc.put: file_content is {}\n", file_content);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    let rpc_tmp_filepath = RPC_TMP_FILEPATH.to_string();

    let r = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
    if let Err(e) = r {
        let msg: String = e.to_string();
        log_utf8_string(msg.as_ptr() as _, msg.len() as _);
    }

    ipfs_put(rpc_tmp_filepath.as_ptr() as _, rpc_tmp_filepath.len() as _);
    std::mem::forget(rpc_tmp_filepath);

    let hash = String::from_raw_parts(
        *RESULT_PTR.get_mut() as _,
        *RESULT_SIZE.get_mut(),
        *RESULT_SIZE.get_mut(),
    );

    let msg = format!("ipfs_rpc.put: file add with hash {}\n", hash);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    *RESULT_PTR.get_mut() = hash.as_ptr() as _;
    *RESULT_SIZE.get_mut() = hash.len();
    std::mem::forget(hash);
}

#[no_mangle]
pub unsafe fn get(hash_ptr: *mut u8, hash_size: usize) {
    let hash = String::from_raw_parts(hash_ptr, hash_size, hash_size);

    let msg = format!("ipfs_rpc.get: getting file with hash {}\n", hash);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    ipfs_get(hash.as_ptr() as _, hash.len() as _);

    let file_path = String::from_raw_parts(
        *RESULT_PTR.get_mut() as _,
        *RESULT_SIZE.get_mut(),
        *RESULT_SIZE.get_mut(),
    );

    let msg = format!("ipfs_rpc.get: reading file from {}\n", file_path);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    let file_content = fs::read(file_path).unwrap_or_else(|_| b"error while reading file".to_vec());

    *RESULT_PTR.get_mut() = file_content.as_ptr() as _;
    *RESULT_SIZE.get_mut() = file_content.len();
    std::mem::forget(file_content);
}

#[link(wasm_import_module = "host")]
extern "C" {
    /// Writes a byte string of size bytes that starts from ptr to a logger.
    fn log_utf8_string(ptr: i32, size: i32);
}

#[link(wasm_import_module = "ipfs_node.wasm")]
extern "C" {
    /// Put a file to ipfs, returns ipfs hash of the file.
    #[link_name = "put"]
    fn ipfs_put(ptr: i32, size: i32);

    /// Get file from ipfs by hash.
    #[link_name = "get"]
    fn ipfs_get(ptr: i32, size: i32);
}
