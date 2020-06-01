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


use std::sync::atomic::AtomicUsize;
use crate::log_utf8_string;

pub static mut RESULT_PTR: AtomicUsize = AtomicUsize::new(0);
pub static mut RESULT_SIZE: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub unsafe fn get_result_ptr() -> usize {
    let msg = format!("wasm_node: calling get_result_ptr\n");
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    *RESULT_PTR.get_mut()
}

#[no_mangle]
pub unsafe fn get_result_size() -> usize {
    let msg = format!("wasm_node: calling get_result_size\n");
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    *RESULT_SIZE.get_mut()
}

#[no_mangle]
pub unsafe fn set_result_ptr(ptr: usize) {
    let msg = format!("wasm_node: calling set_result_ptr with {}\n", ptr);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    *RESULT_PTR.get_mut() = ptr;
}

#[no_mangle]
pub unsafe fn set_result_size(size: usize) {
    let msg = format!("wasm_node: calling set_result_size with {}\n", size);
    log_utf8_string(msg.as_ptr() as _, msg.len() as _);

    *RESULT_SIZE.get_mut() = size;
}
