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

use super::HostImportError;
use super::HostImportResult;
use crate::IValue;
use crate::RecordTypes;
use crate::IType;

use it_lilo_utils::memory_reader::MemoryReader;
use it_lilo_utils::ser_type_size;

pub(super) fn lift_array(
    reader: &MemoryReader<'_>,
    value_type: &IType,
    offset: usize,
    elements_count: usize,
    record_types: &RecordTypes,
) -> HostImportResult<Vec<IValue>> {
    if elements_count == 0 {
        return Ok(vec![]);
    }

    let result_array = match value_type {
        IType::Boolean => reader.read_bool_array(offset, elements_count)?,
        IType::S8 => reader.read_s8_array(offset, elements_count)?,
        IType::S16 => reader.read_s16_array(offset, elements_count)?,
        IType::S32 => reader.read_s32_array(offset, elements_count)?,
        IType::S64 => reader.read_s64_array(offset, elements_count)?,
        IType::U8 => reader.read_u8_array(offset, elements_count)?,
        IType::U16 => reader.read_u16_array(offset, elements_count)?,
        IType::U32 => reader.read_u32_array(offset, elements_count)?,
        IType::U64 => reader.read_u64_array(offset, elements_count)?,
        IType::F32 => reader.read_f32_array(offset, elements_count)?,
        IType::F64 => reader.read_f64_array(offset, elements_count)?,
        IType::I32 => reader.read_i32_array(offset, elements_count)?,
        IType::I64 => reader.read_i64_array(offset, elements_count)?,
        IType::String => lift_string_array(reader, offset, elements_count)?,
        IType::ByteArray => lift_byte_array(reader, offset, elements_count)?,
        IType::Array(ty) => lift_array_array(reader, offset, elements_count, ty, record_types)?,
        IType::Record(record_type_id) => {
            lift_record_array(reader, offset, elements_count, record_type_id, record_types)?
        }
    };

    Ok(result_array)
}

fn lift_string_array(
    reader: &MemoryReader<'_>,
    offset: usize,
    elements_count: usize,
) -> HostImportResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count);
    let size = ser_type_size(&IType::String) * elements_count;
    let seq_reader = reader.sequential_reader(offset, size)?;

    for _ in 0..elements_count {
        let str_offset = seq_reader.read_u32();
        let str_size = seq_reader.read_u32();

        let raw_str = reader.read_raw_u8_array(str_offset as _, str_size as _)?;
        let str = String::from_utf8(raw_str)?;
        result.push(IValue::String(str));
    }

    Ok(result)
}

fn lift_byte_array(
    reader: &MemoryReader<'_>,
    offset: usize,
    elements_count: usize,
) -> HostImportResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count);
    let size = ser_type_size(&IType::ByteArray) * elements_count;
    let seq_reader = reader.sequential_reader(offset, size)?;

    for _ in 0..elements_count {
        let array_offset = seq_reader.read_u32();
        let array_size = seq_reader.read_u32();

        let array = reader.read_raw_u8_array(array_offset as _, array_size as _)?;
        result.push(IValue::ByteArray(array));
    }

    Ok(result)
}

fn lift_array_array(
    reader: &MemoryReader<'_>,
    offset: usize,
    elements_count: usize,
    ty: &Box<IType>,
    record_types: &RecordTypes,
) -> HostImportResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(elements_count);
    let size = ser_type_size(&IType::Array(ty.clone())) * elements_count;
    let seq_reader = reader.sequential_reader(offset, size)?;

    for _ in 0..elements_count {
        let array_offset = seq_reader.read_u32() as usize;
        let elements_count = seq_reader.read_u32() as usize;

        let array = lift_array(reader, ty, array_offset, elements_count, record_types)?;
        result.push(IValue::Array(array));
    }

    Ok(result)
}

fn lift_record_array(
    reader: &MemoryReader<'_>,
    offset: usize,
    elements_count: usize,
    record_type_id: &u64,
    record_types: &RecordTypes,
) -> HostImportResult<Vec<IValue>> {
    let record_type = record_types
        .get(record_type_id)
        .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;

    let mut result = Vec::with_capacity(elements_count);
    let size = it_lilo_utils::record_size(record_type);
    let seq_reader = reader.sequential_reader(offset, size)?;

    for _ in 0..elements_count {
        let record_offset = seq_reader.read_u32();
        let record = super::lift_record(reader, &record_type, record_offset as _, record_types)?;

        result.push(record);
    }

    Ok(result)
}
