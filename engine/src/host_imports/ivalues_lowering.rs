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
use super::HostImportResult;
use crate::call_wasm_func;
use crate::IValue;
use crate::IType;

use wasmer_wit::NEVec;
use it_lilo_utils::memory_writer::MemoryWriter;
use it_lilo_utils::type_tag_form_itype;
use it_lilo_utils::type_tag_form_ivalue;
use it_lilo_utils::ser_value_size;

pub(super) fn ivalue_to_wvalues(
    memory: &MemoryWriter<'_>,
    ivalue: Option<IValue>,
    allocate_func: &AllocateFunc,
) -> HostImportResult<Vec<WValue>> {
    let result = match ivalue {
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
            let type_tag = type_tag_form_itype(&IType::String);
            let offset = call_wasm_func!(allocate_func, str.len() as i32, type_tag as i32);
            memory.write_bytes(offset as _, str.as_bytes())?;

            vec![WValue::I32(offset as _), WValue::I32(str.len() as _)]
        }
        Some(IValue::ByteArray(array)) => {
            let type_tag = type_tag_form_itype(&IType::U8);
            let offset = call_wasm_func!(allocate_func, array.len() as i32, type_tag as i32);
            memory.write_bytes(offset as _, &array)?;

            vec![WValue::I32(offset as _), WValue::I32(array.len() as _)]
        }
        Some(IValue::Array(values)) => {
            let (offset, size) = lower_array(memory, values, allocate_func)?;
            vec![WValue::I32(offset as _), WValue::I32(size as _)]
        }
        Some(IValue::Record(values)) => {
            let offset = lower_record(memory, values, allocate_func)?;
            vec![WValue::I32(offset)]
        }
        None => vec![],
    };

    Ok(result)
}

fn lower_array(
    writer: &MemoryWriter<'_>,
    values: Vec<IValue>,
    allocate_func: &AllocateFunc,
) -> HostImportResult<(usize, usize)> {
    if values.is_empty() {
        return Ok((0, 0));
    }

    let elements_count = values.len();
    let ser_array_size = ser_value_size(&values[0]) * elements_count;
    let type_tag = type_tag_form_ivalue(&values[0]);
    let offset = call_wasm_func!(allocate_func, ser_array_size as _, type_tag as _) as usize;

    let seq_writer = writer.sequential_writer(offset, ser_array_size)?;

    for value in values {
        match value {
            IValue::Boolean(value) => seq_writer.write_u8(value as _),
            IValue::S8(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::S16(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::S32(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::S64(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::U8(value) => seq_writer.write_u8(value),
            IValue::U16(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::U32(value) => seq_writer.write_u32(value),
            IValue::U64(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::I32(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::I64(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::F32(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::F64(value) => seq_writer.write_array(value.to_le_bytes()),
            IValue::String(value) => {
                let str_offset =
                    call_wasm_func!(allocate_func, value.len() as _, type_tag as _) as u32;
                writer.write_bytes(str_offset as _, value.as_bytes())?;

                seq_writer.write_u32(str_offset);
                seq_writer.write_u32(value.len() as u32);
            }

            IValue::ByteArray(value) => {
                let array_offset =
                    call_wasm_func!(allocate_func, value.len() as _, type_tag as _) as u32;
                writer.write_bytes(array_offset as _, &value)?;

                seq_writer.write_u32(array_offset);
                seq_writer.write_u32(value.len() as u32);
            }

            IValue::Array(values) => {
                let (array_offset, array_size) = if !values.is_empty() {
                    lower_array(writer, values, allocate_func)?
                } else {
                    (0, 0)
                };

                seq_writer.write_u32(array_offset as u32);
                seq_writer.write_u32(array_size as u32);
            }

            IValue::Record(values) => {
                let record_offset = lower_record(writer, values, allocate_func)?;

                seq_writer.write_u32(record_offset as u32);
            }
        }
    }

    Ok((offset as _, elements_count as _))
}

fn lower_record(
    memory: &MemoryWriter<'_>,
    values: NEVec<IValue>,
    allocate_func: &AllocateFunc,
) -> HostImportResult<i32> {
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
                    let type_tag = type_tag_form_itype(&IType::String);
                    let offset = call_wasm_func!(allocate_func, value.len() as _, type_tag as _);
                    memory.write_bytes(offset as _, value.as_bytes())?;
                    offset
                } else {
                    0
                } as u32;

                result.extend(&offset.to_le_bytes());
                result.extend(&(value.len() as u32).to_le_bytes());
            }

            IValue::ByteArray(value) => {
                let array_pointer = if !value.is_empty() {
                    let type_tag = type_tag_form_itype(&IType::ByteArray);
                    let offset = call_wasm_func!(allocate_func, value.len() as _, type_tag as _);
                    memory.write_bytes(offset as _, &value)?;
                    offset
                } else {
                    0
                } as u32;

                result.extend(&array_pointer.to_le_bytes());
                result.extend(&(value.len() as u32).to_le_bytes());
            }

            IValue::Array(values) => {
                let (offset, size) = if !values.is_empty() {
                    lower_array(memory, values.clone(), allocate_func)?
                } else {
                    (0, 0)
                };

                result.extend(&(offset as u32).to_le_bytes());
                result.extend(&(size as u32).to_le_bytes());
            }

            IValue::Record(values) => {
                let record_ptr = lower_record(memory, values, allocate_func)? as u32;

                result.extend(&record_ptr.to_le_bytes());
            }
        }
    }

    let type_tag = type_tag_form_itype(&IType::U8);
    let offset = call_wasm_func!(allocate_func, result.len() as _, type_tag as _);
    memory.write_bytes(offset as _, &result)?;

    Ok(offset as _)
}
