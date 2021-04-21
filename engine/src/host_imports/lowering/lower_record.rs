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

use super::AllocateFunc;
use super::HostImportResult;
use crate::call_wasm_func;
use crate::IValue;
use crate::IType;

use wasmer_wit::NEVec;
use it_lilo_utils::memory_writer::MemoryWriter;
use it_lilo_utils::type_tag_form_itype;

pub(super) fn lower_record(
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
                    super::lower_array(memory, values, allocate_func)?
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
