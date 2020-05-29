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

mod mem;
mod result;

use crate::result::{RESULT_PTR, RESULT_SIZE};

#[no_mangle]
pub unsafe fn invoke(file_content_ptr: *mut u8, file_content_size: usize) {
    put(file_content_ptr as _, file_content_size as _);
    let hash = String::from_raw_parts(
        *RESULT_PTR.get_mut(),
        *RESULT_SIZE.get_mut(),
        *RESULT_SIZE.get_mut(),
    );

    log_utf8_string(hash.as_ptr() as _, hash.len() as _);
}

#[link(wasm_import_module = "logger")]
extern "C" {
    /// Writes a byte string of size bytes that starts from ptr to a logger.
    fn log_utf8_string(ptr: i32, size: i32);
}

#[link(wasm_import_module = "node")]
extern "C" {
    /// Put a file to ipfs, returns ipfs hash of the file.
    fn put(ptr: i32, size: i32);

    /// Get file from ipfs by hash.
    fn get(ptr: i32, size: i32);
}
