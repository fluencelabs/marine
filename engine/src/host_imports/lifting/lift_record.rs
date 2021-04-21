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
use it_lilo_utils::memory_reader::SequentialReader;

use wasmer_wit::IRecordType;
use wasmer_wit::NEVec;

pub(super) fn lift_record(
    reader: &MemoryReader<'_>,
    record_type: &IRecordType,
    offset: usize,
    record_types: &RecordTypes,
) -> HostImportResult<IValue> {
    let fields_count = record_type.fields.len();
    let mut values = Vec::with_capacity(fields_count);

    let size = it_lilo_utils::record_size(record_type);
    let seq_reader = reader.sequential_reader(offset, size)?;

    for field in (*record_type.fields).iter() {
        let value = match &field.ty {
            IType::Boolean => IValue::Boolean(seq_reader.read_bool()),
            IType::S8 => IValue::S8(seq_reader.read_i8()),
            IType::S16 => IValue::S16(seq_reader.read_i16()),
            IType::S32 => IValue::S32(seq_reader.read_i32()),
            IType::S64 => IValue::S64(seq_reader.read_i64()),
            IType::I32 => IValue::I32(seq_reader.read_i32()),
            IType::I64 => IValue::I64(seq_reader.read_i64()),
            IType::U8 => IValue::U8(seq_reader.read_u8()),
            IType::U16 => IValue::U16(seq_reader.read_u16()),
            IType::U32 => IValue::U32(seq_reader.read_u32()),
            IType::U64 => IValue::U64(seq_reader.read_u64()),
            IType::F32 => IValue::F32(seq_reader.read_f32()),
            IType::F64 => IValue::F64(seq_reader.read_f64()),
            IType::String => IValue::String(lift_string(reader, &seq_reader)?),
            IType::ByteArray => IValue::ByteArray(lift_bytearray(reader, &seq_reader)?),
            IType::Array(ty) => IValue::Array(lift_array(reader, &seq_reader, ty, record_types)?),
            IType::Record(record_type_id) => {
                lift_record_impl(reader, &seq_reader, record_type_id, record_types)?
            }
        };

        values.push(value);
    }

    let record = NEVec::new(values.into_iter().collect())
        .map_err(|_| HostImportError::EmptyRecord(record_type.name.clone()))?;
    Ok(IValue::Record(record))
}

fn lift_string(
    reader: &MemoryReader<'_>,
    seq_reader: &SequentialReader<'_, '_>,
) -> HostImportResult<String> {
    let offset = seq_reader.read_u32();
    let size = seq_reader.read_u32();

    if size == 0 {
        return Ok(String::new());
    }

    let raw_str = reader.read_raw_u8_array(offset as _, size as _)?;
    let str = String::from_utf8(raw_str)?;
    Ok(str)
}

fn lift_bytearray(
    reader: &MemoryReader<'_>,
    seq_reader: &SequentialReader<'_, '_>,
) -> HostImportResult<Vec<u8>> {
    let offset = seq_reader.read_u32();
    let size = seq_reader.read_u32();

    if size == 0 {
        return Ok(Vec::new());
    }

    let array = reader.read_raw_u8_array(offset as _, size as _)?;
    Ok(array)
}

fn lift_array(
    reader: &MemoryReader<'_>,
    seq_reader: &SequentialReader<'_, '_>,
    ty: &IType,
    record_types: &RecordTypes,
) -> HostImportResult<Vec<IValue>> {
    let offset = seq_reader.read_u32();
    let size = seq_reader.read_u32();

    if size == 0 {
        return Ok(Vec::new());
    }

    let array = super::lift_array(reader, ty, offset as _, size as _, record_types)?;
    Ok(array)
}

fn lift_record_impl(
    reader: &MemoryReader<'_>,
    seq_reader: &SequentialReader<'_, '_>,
    record_type_id: &u64,
    record_types: &RecordTypes,
) -> HostImportResult<IValue> {
    let offset = seq_reader.read_u32();

    let record_type = record_types
        .get(record_type_id)
        .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;

    lift_record(reader, record_type, offset as _, record_types)
}
