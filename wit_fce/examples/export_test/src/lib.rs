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
pub unsafe fn strlen(str_ptr: *mut u8, str_len: usize) -> usize {
    let user_name = String::from_raw_parts(str_ptr, str_len, str_len);
    user_name.len()
}

#[no_mangle]
pub unsafe fn greeting(user_name_ptr: *mut u8, user_name_size: usize) {
    let user_name = String::from_raw_parts(user_name_ptr, user_name_size, user_name_size);
    let user_name = format!("Hi, {}\n", user_name);

    log_utf8_string(user_name.as_ptr() as i32, user_name.len() as i32);

    *RESULT_PTR.get_mut() = user_name.as_ptr() as _;
    *RESULT_SIZE.get_mut() = user_name.len();
    std::mem::forget(user_name);
}

#[link(wasm_import_module = "logger")]
extern "C" {
    // Writes a byte string of size bytes that starts from ptr to a logger
    fn log_utf8_string(ptr: i32, size: i32);
}
