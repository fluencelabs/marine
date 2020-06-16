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
mod path;

use crate::result::{RESULT_PTR, RESULT_SIZE};
use crate::path::to_full_path;

const RESULT_FILE_PATH: &str = "/tmp/ipfs_rpc_file";

pub unsafe fn main() {
    let msg = "ipfs_node.main: WASI initialization finished, env {}";
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);
}

#[no_mangle]
pub unsafe fn put(file_path_ptr: *mut u8, file_path_size: usize) {
    let file_path = String::from_raw_parts(file_path_ptr, file_path_size, file_path_size);

    let msg = format!("ipfs_node.put: file path is {}\n", file_path);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    let file_path = to_full_path(file_path);

    let cmd = format!("add -Q {}", file_path);
    let result = ipfs(cmd.as_ptr() as _, cmd.len() as _);

    let hash = if result == 0 {
        String::from_raw_parts(
            *RESULT_PTR.get_mut() as _,
            *RESULT_SIZE.get_mut(),
            *RESULT_SIZE.get_mut(),
        )
    } else {
        "host ipfs call failed".to_string()
    };

    let msg = format!("ipfs_node.put: file add with hash is {} \n", hash);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    *RESULT_PTR.get_mut() = hash.as_ptr() as _;
    *RESULT_SIZE.get_mut() = hash.len();
    std::mem::forget(hash);
}

#[no_mangle]
pub unsafe fn get(hash_ptr: *mut u8, hash_size: usize) {
    let hash = String::from_raw_parts(hash_ptr, hash_size, hash_size);

    let msg = format!("ipfs_node.get: file hash is {}\n", hash);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    let result_file_path = to_full_path(RESULT_FILE_PATH);

    let cmd = format!("get -o {}  {}", result_file_path, hash);
    let _result = ipfs(cmd.as_ptr() as _, cmd.len() as _);

    let _output = String::from_raw_parts(
        *RESULT_PTR.get_mut() as _,
        *RESULT_SIZE.get_mut(),
        *RESULT_SIZE.get_mut(),
    );

    // TODO: check output

    let file_path = RESULT_FILE_PATH.to_string();
    let msg = format!("ipfs_node.get: file path is {}", file_path);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    *RESULT_PTR.get_mut() = file_path.as_ptr() as _;
    *RESULT_SIZE.get_mut() = file_path.len();
    std::mem::forget(file_path);
}

#[no_mangle]
pub unsafe fn get_addresses() {
    let msg = "ipfs_node.get_addresses".to_string();
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    let cmd = "id -f'<addrs>'";
    let result = ipfs(cmd.as_ptr() as _, cmd.len() as _);

    let multiaddrs = if result == 0 {
        String::from_raw_parts(
            *RESULT_PTR.get_mut() as _,
            *RESULT_SIZE.get_mut(),
            *RESULT_SIZE.get_mut(),
        )
    } else {
        "host ipfs call failed".to_string()
    };

    let msg = format!(
        "ipfs_node.get_addresses: node addresses are {} \n",
        multiaddrs
    );
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    *RESULT_PTR.get_mut() = multiaddrs.as_ptr() as _;
    *RESULT_SIZE.get_mut() = multiaddrs.len();
    std::mem::forget(multiaddrs);
}

#[link(wasm_import_module = "host")]
extern "C" {
    /// Writes a byte string of size bytes that starts from ptr to a logger.
    fn log_utf8_string(ptr: i32, size: i32);

    /// Put a file to ipfs, returns ipfs hash of the file.
    fn ipfs(ptr: i32, size: i32) -> i32;
}
