/*
 * Copyright 2023 Fluence Labs Limited
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
#![allow(clippy::all)]
#![allow(non_snake_case)]

use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub fn allocate_single_module_single_piece(size: i64) -> u32 {
    let addr = Vec::<u8>::with_capacity(size as usize).leak().as_ptr();
    unsafe { std::mem::transmute::<*const u8, usize>(addr) as u32 }
}

#[marine]
pub fn allocate_single_module_64KB_pieces(n_pieces: u32) -> u32 {
    let mut acc: u32 = 0;

    for _ in 0..n_pieces {
        unsafe {
            let addr = Box::leak(Box::new([0u8; 1024 * 64]));
            acc ^= std::mem::transmute::<*const u8, usize>(addr.as_ptr()) as u32;
        }
    }

    acc
}
