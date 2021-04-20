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
mod memory_writer;

pub(crate) use memory_writer::MemoryWriter;

use super::WValue;
use super::AllocateFunc;
use crate::call_wasm_func;
use crate::IValue;

use wasmer_wit::NEVec;

pub(super) fn ivalue_to_wvalues(
    memory: &MemoryWriter<'_>,
    ivalue: Option<IValue>,
    allocate_func: &AllocateFunc,
) -> Vec<WValue> {
    match ivalue {
        Some(IValue::Boolean(v)) => vec![WValue::I32(v as _)],
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
            memory.write_bytes(offset as _, str.as_bytes());

            vec![WValue::I32(offset as _), WValue::I32(str.len() as _)]
        }
        Some(IValue::ByteArray(array)) => {
            let offset = call_wasm_func!(allocate_func, array.len() as i32);
            memory.write_bytes(offset as _, &array);

            vec![WValue::I32(offset as _), WValue::I32(array.len() as _)]
        }
        Some(IValue::Array(values)) => {
            let (offset, size) = lower_array(memory, values, allocate_func);
            vec![WValue::I32(offset as _), WValue::I32(size as _)]
        }
        Some(IValue::Record(values)) => {
            let offset = lower_record(memory, values, allocate_func);
            vec![WValue::I32(offset)]
        }
        None => vec![],
    }
}

fn lower_array(
    memory: &MemoryWriter<'_>,
    values: Vec<IValue>,
    allocate_func: &AllocateFunc,
) -> (usize, usize) {
    if values.is_empty() {
        return (0, 0);
    }

    let elements_count = values.len();
    let ser_array_size = wasmer_wit::ser_value_size(&values[0]) * elements_count;
    let offset = call_wasm_func!(allocate_func, ser_array_size as _) as usize;

    for value in values {
        match value {
            IValue::Boolean(value) => memory.write_u8(offset, value as _),
            IValue::S8(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::S16(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::S32(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::S64(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::U8(value) => memory.write_u8(offset, value),
            IValue::U16(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::U32(value) => memory.write_u32(offset, value),
            IValue::U64(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::I32(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::I64(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::F32(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::F64(value) => memory.write_array(offset, value.to_le_bytes()),
            IValue::String(value) => {
                let str_offset = call_wasm_func!(allocate_func, value.len() as _) as u32;
                memory.write_bytes(str_offset as _, value.as_bytes());

                memory.write_u32(offset, str_offset);
                memory.write_u32(offset + 4, value.len() as u32);
            }

            IValue::ByteArray(value) => {
                let array_offset = call_wasm_func!(allocate_func, value.len() as _) as u32;
                memory.write_bytes(array_offset as _, &value);

                memory.write_u32(offset, array_offset);
                memory.write_u32(offset + 4, value.len() as u32);
            }

            IValue::Array(values) => {
                let (array_offset, array_size) = if !values.is_empty() {
                    lower_array(memory, values, allocate_func)
                } else {
                    (0, 0)
                };

                memory.write_u32(offset, array_offset as u32);
                memory.write_u32(offset + 4, array_size as u32);
            }

            IValue::Record(values) => {
                let record_offset = lower_record(memory, values, allocate_func);

                memory.write_u32(offset, record_offset as u32);
            }
        }
    }

    (offset as _, elements_count as _)
}

fn lower_record(
    memory: &MemoryWriter<'_>,
    values: NEVec<IValue>,
    allocate_func: &AllocateFunc,
) -> i32 {
    // assuming that the average size of a field is 4 bytes
    let mut result: Vec<u8> = Vec::with_capacity(4 * values.len());

    for value in values.into_vec() {
        match value {
            IValue::Boolean(value) => result.push(value as _),
            IValue::S8(value) => result.push(value as _),
            IValue::S16(value) => result.extend(&value.to_le_bytes()),
            IValue::S32(value) => result.extend(&value.to_le_bytes()),
            IValue::S64(value) => result.extend(&value.to_le_bytes()),
            IValue::U8(value) => result.extend(&value.to_le_bytes()),
            IValue::U16(value) => result.extend(&value.to_le_bytes()),
            IValue::U32(value) => result.extend(&value.to_le_bytes()),
            IValue::U64(value) => result.extend(&value.to_le_bytes()),
            IValue::I32(value) => result.extend(&value.to_le_bytes()),
            IValue::I64(value) => result.extend(&value.to_le_bytes()),
            IValue::F32(value) => result.extend(&value.to_le_bytes()),
            IValue::F64(value) => result.extend(&value.to_le_bytes()),
            IValue::String(value) => {
                let offset = if !value.is_empty() {
                    let offset = call_wasm_func!(allocate_func, value.len() as _);
                    memory.write_bytes(offset as _, value.as_bytes());
                    offset
                } else {
                    0
                } as u32;

                result.extend(&offset.to_le_bytes());
                result.extend(&(value.len() as u32).to_le_bytes());
            }

            IValue::ByteArray(value) => {
                let array_pointer = if !value.is_empty() {
                    let offset = call_wasm_func!(allocate_func, value.len() as _);
                    memory.write_bytes(offset as _, &value);
                    offset
                } else {
                    0
                } as u32;

                result.extend(&array_pointer.to_le_bytes());
                result.extend(&(value.len() as u32).to_le_bytes());
            }

            IValue::Array(values) => {
                let (offset, size) = if !values.is_empty() {
                    lower_array(memory, values.clone(), allocate_func)
                } else {
                    (0, 0)
                };

                result.extend(&(offset as u32).to_le_bytes());
                result.extend(&(size as u32).to_le_bytes());
            }

            IValue::Record(values) => {
                let record_ptr = lower_record(memory, values, allocate_func) as u32;

                result.extend(&record_ptr.to_le_bytes());
            }
        }
    }

    let offset = call_wasm_func!(allocate_func, result.len() as _);
    memory.write_bytes(offset as _, &result);

    offset as _
}
