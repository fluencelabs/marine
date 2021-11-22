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

/// Contains converters of types and values between Wasmer and wasmer_interface_types.
use super::{WType, WValue, IType, IValue};

pub(crate) fn wtype_to_itype(ty: &WType) -> IType {
    match ty {
        WType::I32 => IType::I32,
        WType::I64 => IType::I64,
        WType::F32 => IType::F32,
        WType::F64 => IType::F64,
        WType::V128 => unimplemented!(),
    }
}
/*
pub(crate) fn itype_to_wtype(ty: &IType) -> WType {
    match ty {
        IType::S8 => WType::I32,
        IType::S16 => WType::I32,
        IType::S32 => WType::I32,
        IType::S64 => WType::I64,
        IType::U8 => WType::I32,
        IType::U16 => WType::I32,
        IType::U32 => WType::I32,
        IType::U64 => WType::I64,
        IType::I32 => WType::I32,
        IType::I64 => WType::I64,
        IType::F32 => WType::F32,
        IType::F64 => WType::F64,
        ty => {
            eprintln!("trying to convert {:?}", ty);
            unimplemented!()
        }
    }
}

 */
pub(super) fn ival_to_wval(value: &IValue) -> WValue {
    match value {
        IValue::I32(v) => WValue::I32(*v),
        IValue::I64(v) => WValue::I64(*v),
        IValue::F32(v) => WValue::F32(*v),
        IValue::F64(v) => WValue::F64(*v),
        _ => {
            crate::js_log(&format!("called ival_to_wval with unknown value"));
            unimplemented!()
        },
    }
}

pub(super) fn wval_to_ival(value: &WValue) -> IValue {
    match value {
        WValue::I32(v) => IValue::I32(*v),
        WValue::I64(v) => IValue::I64(*v),
        WValue::F32(v) => IValue::F32(*v),
        WValue::F64(v) => IValue::F64(*v),
        _ => unimplemented!(),
    }
}
/*
pub fn itype_arg_to_wtypes(arg: &IType) -> Vec<WType> {
    match arg {
        IType::Boolean
        | IType::S8
        | IType::S16
        | IType::S32
        | IType::I32
        | IType::U8
        | IType::U16
        | IType::U32 => vec![WType::I32],
        IType::S64 | IType::U64 | IType::I64  => vec![WType::I64],
        IType::F32 => vec![WType::F32],
        IType::F64 => vec![WType::F64],
        IType::String => vec![WType::I32, WType::I32],
        _ => {
            crate::js_log("itype_arg_to_wtypes got unexpected type");
            unimplemented!();
        }
    }
}

pub fn itype_to_raw_output_types(ty: &IType) -> Vec<WType> {
    match ty {
        IType::Boolean
        | IType::S8
        | IType::S16
        | IType::S32
        | IType::I32
        | IType::U8
        | IType::U16
        | IType::U32 => vec![WType::I32],
        IType::I64 | IType::U64 | IType::S64 => vec![WType::I64],
        IType::F32 => vec![WType::F32],
        IType::F64 => vec![WType::F64],
        | IType::String
        | IType::Record(..) => vec![],
        _ => {
            crate::js_log("itype_to_raw_output_types got unexpected type");
            unimplemented!();
        }
    }
}
*/
pub fn ival_to_string(val: &IValue) -> String {
    match val {
        IValue::Boolean(val) => {val.to_string()}
        IValue::S8(val) => {val.to_string()}
        IValue::S16(val) => {val.to_string()}
        IValue::S32(val) => {val.to_string()}
        IValue::S64(val) => {val.to_string()}
        IValue::U8(val) => {val.to_string()}
        IValue::U16(val) => {val.to_string()}
        IValue::U32(val) => {val.to_string()}
        IValue::U64(val) => {val.to_string()}
        IValue::F32(val) => {val.to_string()}
        IValue::F64(val) => {val.to_string()}
        IValue::String(val) => {val.to_string()}
        IValue::ByteArray(_) => {"some byte array".to_string()}
        IValue::Array(_) => {"some array".to_string()}
        IValue::I32(val) => {val.to_string()}
        IValue::I64(val) => {val.to_string()}
        IValue::Record(_) => {"some record".to_string()}
    }
}

pub fn itypes_args_to_wtypes<'i>(itypes: impl Iterator<Item = &'i IType>) -> Vec<WType> {
    itypes
        .map(|itype| match itype {
            IType::F32 => vec![WType::F32],
            IType::F64 => vec![WType::F64],
            IType::I64 | IType::U64 => vec![WType::I64],
            IType::String | IType::Array(_) => vec![WType::I32, WType::I32],
            _ => vec![WType::I32],
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn itypes_output_to_wtypes<'i>(itypes: impl Iterator<Item = &'i IType>) -> Vec<WType> {
    itypes
        .map(|itype| match itype {
            IType::F32 => vec![WType::F32],
            IType::F64 => vec![WType::F64],
            IType::I64 | IType::U64 => vec![WType::I64],
            IType::String | IType::Array(_) | IType::Record(_) => vec![],
            _ => vec![WType::I32],
        })
        .flatten()
        .collect::<Vec<_>>()
}
