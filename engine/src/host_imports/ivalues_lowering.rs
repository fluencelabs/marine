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

/// Contain functions intended to put (lower) IValues to Wasm memory
/// and pass it to a Wasm module as raw WValues (Wasm types).
use super::WValue;
use super::AllocateFunc;
use super::utils::write_to_wasm_mem;
use crate::call_wasm_func;
use crate::IValue;

use wasmer_core::vm::Ctx;
use wasmer_wit::NEVec;

pub(super) fn ivalue_to_wvalues(
    ctx: &mut Ctx,
    ivalue: Option<IValue>,
    allocate_func: &AllocateFunc,
) -> Vec<WValue> {
    match ivalue {
        Some(IValue::S8(v)) => vec![WValue::I32(v as _)],
        Some(IValue::S16(v)) => vec![WValue::I32(v as _)],
        Some(IValue::S32(v)) => vec![WValue::I32(v as _)],
        Some(IValue::S64(v)) => vec![WValue::I64(v as _)],
        Some(IValue::U8(v)) => vec![WValue::I32(v as _)],
        Some(IValue::U16(v)) => vec![WValue::I32(v as _)],
        Some(IValue::U32(v)) => vec![WValue::I32(v as _)],
        Some(IValue::U64(v)) => vec![WValue::I64(v as _)],
        Some(IValue::I32(v)) => vec![WValue::I32(v as _)],
        Some(IValue::I64(v)) => vec![WValue::I64(v as _)],
        Some(IValue::F32(v)) => vec![WValue::F32(v)],
        Some(IValue::F64(v)) => vec![WValue::F64(v)],
        Some(IValue::String(str)) => {
            let offset = call_wasm_func!(allocate_func, str.len() as i32);
            write_to_wasm_mem(ctx, offset as usize, str.as_bytes());

            vec![WValue::I32(offset as _), WValue::I32(str.len() as _)]
        }
        Some(IValue::Array(values)) => {
            let (offset, size) = lower_array(ctx, values, allocate_func);
            vec![WValue::I32(offset as _), WValue::I32(size as _)]
        }
        Some(IValue::Record(values)) => {
            println!("lowering values: {:?}", values);
            let offset = lower_record(ctx, values, allocate_func);
            println!("resulted offset: {}", offset);
            vec![WValue::I32(offset)]
        }
        None => vec![],
    }
}

fn lower_array(
    ctx: &mut Ctx,
    array_values: Vec<IValue>,
    allocate_func: &AllocateFunc,
) -> (usize, usize) {
    let mut result: Vec<u64> = Vec::with_capacity(array_values.len());

    for value in array_values {
        match value {
            IValue::S8(value) => result.push(value as _),
            IValue::S16(value) => result.push(value as _),
            IValue::S32(value) => result.push(value as _),
            IValue::S64(value) => result.push(value as _),
            IValue::U8(value) => result.push(value as _),
            IValue::U16(value) => result.push(value as _),
            IValue::U32(value) => result.push(value as _),
            IValue::U64(value) => result.push(value as _),
            IValue::I32(value) => result.push(value as _),
            IValue::I64(value) => result.push(value as _),
            IValue::F32(value) => result.push(value as _),
            IValue::F64(value) => result.push(value.to_bits()),
            IValue::String(value) => {
                let offset = call_wasm_func!(allocate_func, value.len() as _);
                write_to_wasm_mem(ctx, offset as _, value.as_bytes());

                result.push(offset as _);
                result.push(value.len() as _);
            }

            IValue::Array(values) => {
                let (array_offset, array_size) = if !values.is_empty() {
                    lower_array(ctx, values, allocate_func)
                } else {
                    (0, 0)
                };

                result.push(array_offset as _);
                result.push(array_size as _);
            }

            IValue::Record(values) => {
                let record_offset = lower_record(ctx, values, allocate_func);
                result.push(record_offset as _);
            }
        }
    }

    let result = safe_transmute::transmute_to_bytes::<u64>(&result);
    let result_pointer = call_wasm_func!(allocate_func, result.len() as _);
    write_to_wasm_mem(ctx, result_pointer as _, result);

    (result_pointer as _, result.len() as _)
}

fn lower_record(ctx: &mut Ctx, values: NEVec<IValue>, allocate_func: &AllocateFunc) -> i32 {
    let mut result: Vec<u64> = Vec::with_capacity(values.len());

    for value in values.into_vec() {
        match value {
            IValue::S8(value) => result.push(value as _),
            IValue::S16(value) => result.push(value as _),
            IValue::S32(value) => result.push(value as _),
            IValue::S64(value) => result.push(value as _),
            IValue::U8(value) => result.push(value as _),
            IValue::U16(value) => result.push(value as _),
            IValue::U32(value) => result.push(value as _),
            IValue::U64(value) => result.push(value as _),
            IValue::I32(value) => result.push(value as _),
            IValue::I64(value) => result.push(value as _),
            IValue::F32(value) => result.push(value as _),
            IValue::F64(value) => result.push(value.to_bits()),
            IValue::String(value) => {
                let string_pointer = if !value.is_empty() {
                    let offset = call_wasm_func!(allocate_func, value.len() as _);
                    write_to_wasm_mem(ctx, offset as usize, value.as_bytes());
                    offset
                } else {
                    0
                };

                result.push(string_pointer as _);
                result.push(value.len() as _);
            }

            IValue::Array(values) => {
                let (offset, size) = if !values.is_empty() {
                    lower_array(ctx, values, allocate_func)
                } else {
                    (0, 0)
                };

                result.push(offset as _);
                result.push(size as _);
            }

            IValue::Record(values) => {
                let record_ptr = lower_record(ctx, values, allocate_func);

                result.push(record_ptr as _);
            }
        }
    }

    let result = safe_transmute::transmute_to_bytes::<u64>(&result);
    let offset = call_wasm_func!(allocate_func, result.len() as _);
    write_to_wasm_mem(ctx, offset as _, result);

    offset as _
}
