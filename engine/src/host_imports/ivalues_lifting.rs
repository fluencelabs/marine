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

/// Contain functions intended to create (lift) IValues from raw WValues (Wasm types).
mod memory_reader;

pub(crate) use memory_reader::MemoryReader;

use super::WType;
use super::WValue;
use super::HostImportError;
use crate::IValue;
use crate::RecordTypes;
use crate::IType;
use super::HostImportResult;

use wasmer_wit::IRecordType;
use wasmer_wit::NEVec;

use std::rc::Rc;

macro_rules! next_wvalue {
    ($wvalue_iter:ident, $wtype:ident) => {
        match $wvalue_iter
            .next()
            .ok_or_else(|| HostImportError::MismatchWValuesCount)?
        {
            WValue::$wtype(v) => *v,
            v => return Err(HostImportError::MismatchWValues(WType::$wtype, v.clone())),
        };
    };
}

macro_rules! simple_wvalue_to_ivalue {
    ($result:ident, $wvalue_iter:ident, $wtype:ident, $ivalue:ident) => {{
        let w = next_wvalue!($wvalue_iter, $wtype);
        $result.push(IValue::$ivalue(w as _))
    }};
}

pub(super) fn wvalues_to_ivalues(
    reader: &MemoryReader<'_>,
    wvalues: &[WValue],
    itypes: &[IType],
    record_types: &Rc<RecordTypes>,
) -> HostImportResult<Vec<IValue>> {
    let mut result = Vec::with_capacity(wvalues.len());
    let mut wvalue = wvalues.iter();

    for itype in itypes.iter() {
        match itype {
            IType::Boolean => {
                let w = next_wvalue!(wvalue, I32);
                result.push(IValue::Boolean(w != 0))
            }
            IType::S8 => simple_wvalue_to_ivalue!(result, wvalue, I32, S8),
            IType::S16 => simple_wvalue_to_ivalue!(result, wvalue, I32, S16),
            IType::S32 => simple_wvalue_to_ivalue!(result, wvalue, I32, S32),
            IType::S64 => simple_wvalue_to_ivalue!(result, wvalue, I64, S64),
            IType::U8 => simple_wvalue_to_ivalue!(result, wvalue, I32, U8),
            IType::U16 => simple_wvalue_to_ivalue!(result, wvalue, I32, U16),
            IType::U32 => simple_wvalue_to_ivalue!(result, wvalue, I32, U32),
            IType::U64 => simple_wvalue_to_ivalue!(result, wvalue, I64, U64),
            IType::I32 => simple_wvalue_to_ivalue!(result, wvalue, I32, I32),
            IType::I64 => simple_wvalue_to_ivalue!(result, wvalue, I64, I64),
            IType::F32 => simple_wvalue_to_ivalue!(result, wvalue, F32, F32),
            IType::F64 => simple_wvalue_to_ivalue!(result, wvalue, F64, F64),
            IType::String => {
                let offset = next_wvalue!(wvalue, I32);
                let elements_count = next_wvalue!(wvalue, I32);

                let raw_str = reader.read_raw_u8_array(offset as _, elements_count as _);
                // TODO: check for errors
                let str = String::from_utf8(raw_str).unwrap();
                result.push(IValue::String(str));
            }
            IType::ByteArray => {
                let offset = next_wvalue!(wvalue, I32);
                let elements_count = next_wvalue!(wvalue, I32);

                let array = reader.read_raw_u8_array(offset as _, elements_count as _);
                result.push(IValue::ByteArray(array));
            }
            IType::Array(ty) => {
                let offset = next_wvalue!(wvalue, I32);
                let size = next_wvalue!(wvalue, I32);

                let array = lift_array(reader, ty, offset as _, size as _, record_types)?;
                result.push(IValue::Array(array));
            }
            IType::Record(record_type_id) => {
                let record_type = record_types
                    .get(record_type_id)
                    .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;
                let offset = next_wvalue!(wvalue, I32);

                let record = lift_record(reader, record_type, offset as _, record_types)?;
                result.push(record);
            }
        }
    }

    Ok(result)
}

fn lift_array(
    reader: &MemoryReader<'_>,
    value_type: &IType,
    offset: usize,
    elements_count: usize,
    record_types: &Rc<RecordTypes>,
) -> HostImportResult<Vec<IValue>> {
    if elements_count == 0 {
        return Ok(vec![]);
    }

    let result_array = match value_type {
        IType::Boolean => reader.read_bool_array(offset, elements_count),
        IType::S8 => reader.read_s8_array(offset, elements_count),
        IType::S16 => reader.read_s16_array(offset, elements_count),
        IType::S32 => reader.read_s32_array(offset, elements_count),
        IType::S64 => reader.read_s64_array(offset, elements_count),
        IType::U8 => reader.read_u8_array(offset, elements_count),
        IType::U16 => reader.read_u16_array(offset, elements_count),
        IType::U32 => reader.read_u32_array(offset, elements_count),
        IType::U64 => reader.read_u64_array(offset, elements_count),
        IType::F32 => reader.read_f32_array(offset, elements_count),
        IType::F64 => reader.read_f64_array(offset, elements_count),
        IType::I32 => reader.read_i32_array(offset, elements_count),
        IType::I64 => reader.read_i64_array(offset, elements_count),
        IType::String => {
            let mut result = Vec::with_capacity(elements_count);
            let seq_reader = reader.sequential_reader(offset);

            for _ in 0..elements_count {
                let str_offset = seq_reader.read_u32();
                let str_size = seq_reader.read_u32();

                let raw_str = reader.read_raw_u8_array(str_offset as _, str_size as _);
                let str = String::from_utf8(raw_str).unwrap();
                result.push(IValue::String(str));
            }

            result
        }
        IType::ByteArray => {
            let mut result = Vec::with_capacity(elements_count);
            let seq_reader = reader.sequential_reader(offset);

            for _ in 0..elements_count {
                let array_offset = seq_reader.read_u32();
                let array_size = seq_reader.read_u32();

                let array = reader.read_raw_u8_array(array_offset as _, array_size as _);
                result.push(IValue::ByteArray(array));
            }

            result
        }
        IType::Array(ty) => {
            let mut result = Vec::with_capacity(elements_count);
            let seq_reader = reader.sequential_reader(offset);

            for _ in 0..elements_count {
                let array_offset = seq_reader.read_u32() as usize;
                let elements_count = seq_reader.read_u32() as usize;

                let array = lift_array(reader, ty, array_offset, elements_count, record_types)?;
                result.push(IValue::Array(array));
            }

            result
        }
        IType::Record(record_type_id) => {
            let record_type = record_types
                .get(record_type_id)
                .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;

            let mut result = Vec::with_capacity(elements_count);
            let seq_reader = reader.sequential_reader(offset);

            for _ in 0..elements_count {
                let record_offset = seq_reader.read_u32();
                let record = lift_record(reader, &record_type, record_offset as _, record_types)?;

                result.push(record);
            }

            result
        }
    };

    Ok(result_array)
}

fn lift_record(
    reader: &MemoryReader<'_>,
    record_type: &IRecordType,
    offset: usize,
    record_types: &Rc<RecordTypes>,
) -> HostImportResult<IValue> {
    let fields_count = record_type.fields.len();
    let mut values = Vec::with_capacity(fields_count);

    // let size = wasmer_wit::record_size(record_type);
    let seq_reader = reader.sequential_reader(offset);

    for field in (*record_type.fields).iter() {
        match &field.ty {
            IType::Boolean => values.push(IValue::Boolean(seq_reader.read_bool())),
            IType::S8 => values.push(IValue::S8(seq_reader.read_i8())),
            IType::S16 => values.push(IValue::S16(seq_reader.read_i16())),
            IType::S32 => values.push(IValue::S32(seq_reader.read_i32())),
            IType::S64 => values.push(IValue::S64(seq_reader.read_i64())),
            IType::I32 => values.push(IValue::I32(seq_reader.read_i32())),
            IType::I64 => values.push(IValue::I64(seq_reader.read_i64())),
            IType::U8 => values.push(IValue::U8(seq_reader.read_u8())),
            IType::U16 => values.push(IValue::U16(seq_reader.read_u16())),
            IType::U32 => values.push(IValue::U32(seq_reader.read_u32())),
            IType::U64 => values.push(IValue::U64(seq_reader.read_u64())),
            IType::F32 => values.push(IValue::F32(seq_reader.read_f32())),
            IType::F64 => values.push(IValue::F64(seq_reader.read_f64())),
            IType::String => {
                let string_offset = seq_reader.read_u32();
                let string_size = seq_reader.read_u32();

                if string_size != 0 {
                    let raw_str = reader.read_raw_u8_array(string_offset as _, string_size as _);
                    let str = String::from_utf8(raw_str).unwrap();
                    values.push(IValue::String(str));
                } else {
                    values.push(IValue::String(String::new()));
                }
            }
            IType::ByteArray => {
                let array_offset = seq_reader.read_u32();
                let array_size = seq_reader.read_u32();

                if array_size != 0 {
                    let array = reader.read_raw_u8_array(array_offset as _, array_size as _);
                    values.push(IValue::ByteArray(array));
                } else {
                    values.push(IValue::ByteArray(vec![]));
                }
            }
            IType::Array(ty) => {
                let array_offset = seq_reader.read_u32();
                let array_size = seq_reader.read_u32();

                if array_size != 0 {
                    let array =
                        lift_array(reader, ty, array_offset as _, array_size as _, record_types)?;
                    values.push(IValue::Array(array));
                } else {
                    values.push(IValue::Array(vec![]));
                }
            }
            IType::Record(record_type_id) => {
                let record_offset = seq_reader.read_u32();

                let record_type = record_types
                    .get(record_type_id)
                    .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;
                let record = lift_record(reader, record_type, record_offset as _, record_types)?;
                values.push(record);
            }
        }
    }

    Ok(IValue::Record(
        NEVec::new(values.into_iter().collect())
            .expect("Record must have at least one field, zero given"),
    ))
}
