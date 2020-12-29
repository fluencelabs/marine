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
use super::WType;
use super::WValue;
use super::HostImportError;
use crate::IValue;
use crate::RecordTypes;
use crate::IType;
use super::Result;

use wasmer_core::memory::ptr::{Array, WasmPtr};
use wasmer_core::vm::Ctx;
use wasmer_wit::IRecordType;
use wasmer_wit::NEVec;

use std::rc::Rc;

pub(super) fn wvalues_to_ivalues(
    ctx: &Ctx,
    wvalues: &[WValue],
    itypes: &[IType],
    record_types: &Rc<RecordTypes>,
) -> Result<Vec<IValue>> {
    let mut result = Vec::new();
    let mut wvalue = wvalues.iter();

    macro_rules! next_wvalue(
        ($wtype:ident) => {
            match wvalue
                .next()
                .ok_or_else(|| HostImportError::MismatchWValuesCount)?
                {
                    WValue::$wtype(v) => *v,
                    v => return Err(HostImportError::MismatchWValues(WType::$wtype, v.clone())),
                };
        }
    );

    macro_rules! simple_wvalue_to_ivalue(
        ($wtype:ident, $ivalue:ident) => {
            {
                let w = next_wvalue!($wtype);
                result.push(IValue::$ivalue(w as _))
            }
        }
    );

    for itype in itypes.iter() {
        match itype {
            IType::S8 => simple_wvalue_to_ivalue!(I32, S8),
            IType::S16 => simple_wvalue_to_ivalue!(I32, S16),
            IType::S32 => simple_wvalue_to_ivalue!(I32, S32),
            IType::S64 => simple_wvalue_to_ivalue!(I64, S64),
            IType::U8 => simple_wvalue_to_ivalue!(I32, U8),
            IType::U16 => simple_wvalue_to_ivalue!(I32, U16),
            IType::U32 => simple_wvalue_to_ivalue!(I32, U32),
            IType::U64 => simple_wvalue_to_ivalue!(I64, U64),
            IType::I32 => simple_wvalue_to_ivalue!(I32, I32),
            IType::I64 => simple_wvalue_to_ivalue!(I64, I64),
            IType::F32 => simple_wvalue_to_ivalue!(F32, F32),
            IType::F64 => simple_wvalue_to_ivalue!(F64, F64),
            // anyrefs aren't supported now, and isn't generated by rust-sdk
            IType::Anyref => unimplemented!("anyrefs aren't supported now"),
            IType::String => {
                let offset = next_wvalue!(I32);
                let size = next_wvalue!(I32);

                let wasm_ptr = WasmPtr::<u8, Array>::new(offset as _);
                let str = wasm_ptr
                    .get_utf8_string(ctx.memory(0), size as _)
                    .ok_or(HostImportError::InvalidMemoryAccess(offset, size))?;

                result.push(IValue::String(str.to_string()));
            }
            IType::Array(ty) => {
                let offset = next_wvalue!(I32);
                let size = next_wvalue!(I32);

                let array = lift_array(ctx, ty, offset as _, size as _, record_types)?;
                result.push(IValue::Array(array));
            }
            IType::Record(record_type_id) => {
                let record_type = record_types
                    .get(record_type_id)
                    .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;
                let offset = next_wvalue!(I32);

                let record = lift_record(ctx, record_type, offset as _, record_types)?;
                result.push(record);
            }
        }
    }

    Ok(result)
}

fn lift_array(
    ctx: &Ctx,
    value_type: &IType,
    offset: usize,
    size: usize,
    record_types: &Rc<RecordTypes>,
) -> Result<Vec<IValue>> {
    if size == 0 {
        return Ok(vec![]);
    }

    let wasm_ptr = WasmPtr::<u64, Array>::new(offset as _);
    let mut data = wasm_ptr
        .deref(ctx.memory(0), offset as _, size as _)
        .ok_or_else(|| HostImportError::InvalidMemoryAccess(offset as _, size as _))?
        .iter();

    macro_rules! simple_type_array_convert(
        ($itype:ident) => {
            {
                data.map(|v| IValue::$itype(v.get() as _)).collect::<Vec<_>>()
            }
        }
    );

    let result_array = match value_type {
        IType::S8 => simple_type_array_convert!(S8),
        IType::S16 => simple_type_array_convert!(S16),
        IType::S32 => simple_type_array_convert!(S32),
        IType::S64 => simple_type_array_convert!(S64),
        IType::U8 => simple_type_array_convert!(U8),
        IType::U16 => simple_type_array_convert!(U16),
        IType::U32 => simple_type_array_convert!(U32),
        IType::U64 => simple_type_array_convert!(U64),
        IType::F32 => simple_type_array_convert!(F32),
        IType::F64 => simple_type_array_convert!(F64),
        IType::I32 => simple_type_array_convert!(I32),
        IType::I64 => simple_type_array_convert!(I64),
        // anyrefs aren't supported now, and isn't generated by rust-sdk
        IType::Anyref => unimplemented!("anyrefs aren't supported now"),
        IType::String => {
            let mut result = Vec::with_capacity(data.len() / 2);

            while let Some(string_offset) = data.next() {
                let string_size = data
                    .next()
                    .ok_or(HostImportError::OddPointersCount(IType::String))?;

                let string = WasmPtr::<u8, Array>::new(string_offset.get() as _)
                    .get_utf8_string(ctx.memory(0), string_size.get() as _)
                    .ok_or_else(|| HostImportError::InvalidMemoryAccess(offset as _, size as _))?;

                result.push(IValue::String(string.to_string()));
            }

            result
        }
        IType::Array(ty) => {
            let mut result = Vec::with_capacity(data.len() / 2);

            while let Some(array_offset) = data.next() {
                let array_size = data
                    .next()
                    .ok_or_else(|| HostImportError::OddPointersCount(IType::Array(ty.clone())))?;
                let array = lift_array(
                    ctx,
                    ty,
                    array_offset.get() as _,
                    array_size.get() as _,
                    record_types,
                )?;

                result.push(IValue::Array(array));
            }

            result
        }
        IType::Record(record_type_id) => {
            let record_type = record_types
                .get(record_type_id)
                .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;

            let mut result = Vec::with_capacity(data.len() / 2);

            for record_offset in data {
                let record =
                    lift_record(ctx, &record_type, record_offset.get() as _, record_types)?;

                result.push(record);
            }

            result
        }
    };

    Ok(result_array)
}

fn lift_record(
    ctx: &Ctx,
    record_type: &IRecordType,
    offset: usize,
    record_types: &Rc<RecordTypes>,
) -> Result<IValue> {
    // TODO: make it export from wasmer-interface-types crate
    fn record_size(record_type: &IRecordType) -> usize {
        let mut record_size = 0;

        for field_type in record_type.fields.iter() {
            let params_count = match field_type.ty {
                IType::String | IType::Array(_) => 2,
                _ => 1,
            };

            record_size += std::mem::size_of::<u64>() * params_count;
        }

        record_size
    }

    let length = record_type.fields.len();
    let mut values = Vec::with_capacity(length);
    let size = record_size(record_type);
    let data = WasmPtr::<u64, Array>::new(offset as _)
        .deref(ctx.memory(0), offset as _, size as _)
        .ok_or_else(|| HostImportError::InvalidMemoryAccess(offset as _, size as _))?;

    let mut field_id = 0;
    for field in (*record_type.fields).iter() {
        let value = data[field_id].get();
        match &field.ty {
            IType::S8 => {
                values.push(IValue::S8(value as _));
            }
            IType::S16 => {
                values.push(IValue::S16(value as _));
            }
            IType::S32 => {
                values.push(IValue::S32(value as _));
            }
            IType::S64 => {
                values.push(IValue::S64(value as _));
            }
            IType::I32 => {
                values.push(IValue::I32(value as _));
            }
            IType::I64 => {
                values.push(IValue::I64(value as _));
            }
            IType::U8 => {
                values.push(IValue::U8(value as _));
            }
            IType::U16 => {
                values.push(IValue::U16(value as _));
            }
            IType::U32 => {
                values.push(IValue::U32(value as _));
            }
            IType::U64 => {
                values.push(IValue::U64(value as _));
            }
            IType::F32 => {
                values.push(IValue::F32(value as _));
            }
            IType::F64 => values.push(IValue::F64(f64::from_bits(value))),
            // anyrefs aren't supported now, and isn't generated by rust-sdk
            IType::Anyref => unimplemented!("anyrefs aren't supported now"),
            IType::String => {
                let string_offset = value;
                field_id += 1;
                let string_size = data[field_id].get();

                if string_size != 0 {
                    let string = WasmPtr::<u8, Array>::new(string_offset as _)
                        .get_utf8_string(ctx.memory(0), size as _)
                        .ok_or(HostImportError::OddPointersCount(IType::String))?;
                    values.push(IValue::String(string.to_string()));
                } else {
                    values.push(IValue::String(String::new()));
                }
            }
            IType::Array(ty) => {
                let array_offset = value;
                field_id += 1;
                let array_size = data[field_id].get();

                if array_size != 0 {
                    let array =
                        lift_array(ctx, &ty, array_offset as _, array_size as _, record_types)?;
                    values.push(IValue::Array(array));
                } else {
                    values.push(IValue::Array(vec![]));
                }
            }
            IType::Record(record_type_id) => {
                let offset = value;

                let record_type = record_types
                    .get(record_type_id)
                    .ok_or_else(|| HostImportError::RecordTypeNotFound(*record_type_id))?;
                values.push(lift_record(ctx, record_type, offset as _, record_types)?);
            }
        }
        field_id += 1;
    }

    Ok(IValue::Record(
        NEVec::new(values.into_iter().collect())
            .expect("Record must have at least one field, zero given"),
    ))
}
