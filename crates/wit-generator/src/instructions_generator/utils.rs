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

use super::IType;
use crate::instructions_generator::WITResolver;
use crate::Result;

use fluence_sdk_wit::ParsedType;
use fluence_sdk_wit::RustType;

// return error if there is no record with such name
pub(crate) fn ptype_to_itype_checked(
    pty: &ParsedType,
    wit_resolver: &WITResolver,
) -> Result<IType> {
    match pty {
        ParsedType::Record(record_name) => {
            wit_resolver.get_record_type(record_name)?;
            Ok(IType::Record(record_name.clone()))
        }
        _ => Ok(ptype_to_itype_unchecked(pty)),
    }
}

pub(crate) fn ptype_to_itype_unchecked(pty: &ParsedType) -> IType {
    match pty {
        ParsedType::I8 => IType::S8,
        ParsedType::I16 => IType::S16,
        ParsedType::I32 => IType::S32,
        ParsedType::I64 => IType::S64,
        ParsedType::U8 => IType::U8,
        ParsedType::U16 => IType::U16,
        ParsedType::U32 => IType::U32,
        ParsedType::U64 => IType::U64,
        ParsedType::F32 => IType::F32,
        ParsedType::F64 => IType::F64,
        ParsedType::Boolean => IType::I32,
        ParsedType::Utf8String => IType::String,
        ParsedType::ByteVector => IType::ByteArray,
        ParsedType::Record(record_name) => IType::Record(record_name.clone()),
    }
}

pub(crate) fn wtype_to_itype(pty: &RustType) -> IType {
    match pty {
        RustType::I8 => IType::S8,
        RustType::I16 => IType::S16,
        RustType::I32 => IType::S32,
        RustType::I64 => IType::S64,
        RustType::U8 => IType::U8,
        RustType::U16 => IType::U16,
        RustType::U32 => IType::U32,
        RustType::U64 => IType::U64,
        RustType::F32 => IType::F32,
        RustType::F64 => IType::F64,
    }
}
