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

use wasmer_core::vm::Ctx;
use wasmer_core::memory::ptr::{Array, WasmPtr};

pub(super) fn log_utf8_string(ctx: &mut Ctx, offset: i32, size: i32) {
    let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => log::info!("{}", msg),
        None => log::warn!("logger: incorrect UTF8 string's been supplied to logger"),
    }
}
