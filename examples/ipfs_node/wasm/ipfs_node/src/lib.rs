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

#[no_mangle]
pub unsafe fn put(file_content_ptr: *mut u8, file_content_size: usize) {
    let file_content =
        String::from_raw_parts(file_content_ptr, file_content_size, file_content_size);

    let msg = format!(
        "from Wasm ipfs_node.get: file content is {}\n",
        file_content
    );
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    let cmd = format!("put {}", file_content);
    ipfs(cmd.as_ptr() as _, cmd.len() as _);

    let after_ipfs = format!("after ipfs call");
    log_utf8_string(after_ipfs.as_ptr() as _, after_ipfs.len() as _);

    let result = "IPFS node: hash is asdasdsad".to_string();

    *RESULT_PTR.get_mut() = result.as_ptr() as _;
    *RESULT_SIZE.get_mut() = result.len();
    std::mem::forget(result);
}

#[no_mangle]
pub unsafe fn get(hash_ptr: *mut u8, hash_size: usize) {
    let hash = String::from_raw_parts(hash_ptr, hash_size, hash_size);

    let msg = format!("from Wasm ipfs_node.get: file hash is {}\n", hash);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    let cmd = format!("get {}", hash);
    ipfs(cmd.as_ptr() as _, cmd.len() as _);

    let result = "IPFS node: file is hhhhaaa".to_string();

    *RESULT_PTR.get_mut() = result.as_ptr() as _;
    *RESULT_SIZE.get_mut() = result.len();
    std::mem::forget(result);
}

#[link(wasm_import_module = "host")]
extern "C" {
    /// Writes a byte string of size bytes that starts from ptr to a logger.
    fn log_utf8_string(ptr: i32, size: i32);

    /// Put a file to ipfs, returns ipfs hash of the file.
    fn ipfs(ptr: i32, size: i32) -> i32;
}
