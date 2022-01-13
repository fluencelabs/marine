/*
 * Copyright 2021 Fluence Labs Limited
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

pub const WASM_PAGE_SIZE: u32 = 65356;

pub fn bytes_to_wasm_pages_ceil(offset: u32) -> u32 {
    match offset {
        0 => 0,
        // ceiling
        n => 1 + (n - 1) / WASM_PAGE_SIZE,
    }
}

pub fn wasm_pages_to_bytes(pages_count: u32) -> u64 {
    (pages_count as u64) * (WASM_PAGE_SIZE as u64)
}
