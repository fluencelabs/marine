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
use wasmer_core::typed_func::DynamicFunc;

pub(super) fn log_utf8_string(ctx: &mut Ctx, offset: i32, size: i32) {
    use wasmer_core::memory::ptr::{Array, WasmPtr};

    let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => print!("{}", msg),
        None => print!("ipfs node logger: incorrect UTF8 string's been supplied to logger"),
    }
}

pub(super) fn create_host_import_func(host_cmd: String) -> DynamicFunc<'static> {
    use wasmer_core::types::Value;
    use wasmer_core::types::Type;
    use wasmer_core::types::FuncSig;

    let func = Box::new(move |ctx: &mut Ctx, inputs: &[Value]| -> Vec<Value> {
        use wasmer_core::memory::ptr::{Array, WasmPtr};

        // TODO: refactor this
        let array_ptr = inputs[0].to_u128() as i32;
        let array_size = inputs[1].to_u128() as i32;

        let wasm_ptr = WasmPtr::<u8, Array>::new(array_ptr as _);
        match wasm_ptr.get_utf8_string(ctx.memory(0), array_size as _) {
            Some(msg) => print!("{}", msg),
            None => print!("ipfs node logger: incorrect UTF8 string's been supplied to logger"),
        }
        vec![]
    });

    DynamicFunc::new(
        std::sync::Arc::new(FuncSig::new(vec![Type::I32, Type::I32], vec![])),
        func,
    )
}
