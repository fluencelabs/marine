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

use super::utils::get_export_func_by_name;

use wasmer_core::vm::Ctx;
use wasmer_core::typed_func::DynamicFunc;
use wasmer_core::types::Value;
use wasmer_core::types::Type;
use wasmer_core::types::FuncSig;

const ALLOCATE_FUNC_NAME: &str = "allocate";
const SET_PTR_FUNC_NAME: &str = "set_result_ptr";
const SET_SIZE_FUNC_NAME: &str = "set_result_size";

pub(super) fn log_utf8_string(ctx: &mut Ctx, offset: i32, size: i32) {
    use wasmer_core::memory::ptr::{Array, WasmPtr};

    let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
    match wasm_ptr.get_utf8_string(ctx.memory(0), size as _) {
        Some(msg) => print!("{}", msg),
        None => println!("ipfs node logger: incorrect UTF8 string's been supplied to logger"),
    }
}

fn write_to_mem(context: &mut Ctx, address: usize, value: &[u8]) {
    let memory = context.memory(0);

    for (byte_id, cell) in memory.view::<u8>()[address as usize..(address + value.len())]
        .iter()
        .enumerate()
    {
        cell.set(value[byte_id]);
    }
}

pub(super) fn create_host_import_func<S>(host_cmd: S) -> DynamicFunc<'static>
where
    S: Into<String>,
{
    /*
    let mut allocate_func: Option<Func<'static, i32, i32>> = None;
    let mut set_result_ptr: Option<Func<'static, i32, ()>> = None;
    let mut set_result_size: Option<Func<'static, i32, ()>> = None;
     */

    let host_cmd = host_cmd.into();

    let func = move |ctx: &mut Ctx, inputs: &[Value]| -> Vec<Value> {
        use wasmer_core::memory::ptr::{Array, WasmPtr};

        let array_ptr = inputs[0].to_u128() as i32;
        let array_size = inputs[1].to_u128() as i32;

        let wasm_ptr = WasmPtr::<u8, Array>::new(array_ptr as _);
        let result = match wasm_ptr.get_utf8_string(ctx.memory(0), array_size as _) {
            Some(arg_value) => cmd_lib::run_fun!("{} {}", host_cmd, arg_value).unwrap(),
            None => return vec![Value::I32(1)],
        };

        unsafe {
            let mem_address = match get_export_func_by_name::<i32, i32>(ctx, ALLOCATE_FUNC_NAME) {
                Ok(func) => func.call(result.len() as i32).unwrap(),
                Err(_) => return vec![Value::I32(2)],
            };

            write_to_mem(ctx, mem_address as usize, result.as_bytes());

            match get_export_func_by_name::<i32, ()>(ctx, SET_PTR_FUNC_NAME) {
                Ok(func) => func.call(mem_address as i32).unwrap(),
                Err(_) => return vec![Value::I32(3)],
            };

            match get_export_func_by_name::<i32, ()>(ctx, SET_SIZE_FUNC_NAME) {
                Ok(func) => func.call(result.len() as i32).unwrap(),
                Err(_) => return vec![Value::I32(4)],
            };

            vec![Value::I32(0)]
        }
    };

    DynamicFunc::new(
        std::sync::Arc::new(FuncSig::new(vec![Type::I32, Type::I32], vec![Type::I32])),
        func,
    )
}
