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

use it_lilo_utils::memory_writer::MemoryWriter;
use it_lilo_utils::type_tag_form_ivalue;
use it_lilo_utils::ser_value_size;

pub(super) fn lower_array(
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
                let record_offset = super::lower_record(writer, values, allocate_func)?;

                seq_writer.write_u32(record_offset as u32);
            }
        }
    }

    Ok((offset as _, elements_count as _))
}
