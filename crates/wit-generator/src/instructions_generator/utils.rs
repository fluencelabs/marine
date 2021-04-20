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
    wit_resolver: &mut WITResolver<'_>,
) -> Result<IType> {
    match pty {
        ParsedType::I8(_) => Ok(IType::S8),
        ParsedType::I16(_) => Ok(IType::S16),
        ParsedType::I32(_) => Ok(IType::S32),
        ParsedType::I64(_) => Ok(IType::S64),
        ParsedType::U8(_) => Ok(IType::U8),
        ParsedType::U16(_) => Ok(IType::U16),
        ParsedType::U32(_) => Ok(IType::U32),
        ParsedType::U64(_) => Ok(IType::U64),
        ParsedType::F32(_) => Ok(IType::F32),
        ParsedType::F64(_) => Ok(IType::F64),
        ParsedType::Boolean(_) => Ok(IType::Boolean),
        ParsedType::Utf8Str(_) => Ok(IType::String),
        ParsedType::Utf8String(_) => Ok(IType::String),
        ParsedType::Vector(ty, _) => {
            let array_itype = ptype_to_itype_checked(ty, wit_resolver)?;
            Ok(IType::Array(Box::new(array_itype)))
        }
        ParsedType::Record(record_name, _) => {
            let record_type_id = wit_resolver.get_record_type_id(record_name)?;
            Ok(IType::Record(record_type_id as _))
        }
    }
}

pub(crate) fn ptype_to_itype_unchecked(
    pty: &ParsedType,
    wit_resolver: &mut WITResolver<'_>,
) -> IType {
    match pty {
        ParsedType::I8(_) => IType::S8,
        ParsedType::I16(_) => IType::S16,
        ParsedType::I32(_) => IType::S32,
        ParsedType::I64(_) => IType::S64,
        ParsedType::U8(_) => IType::U8,
        ParsedType::U16(_) => IType::U16,
        ParsedType::U32(_) => IType::U32,
        ParsedType::U64(_) => IType::U64,
        ParsedType::F32(_) => IType::F32,
        ParsedType::F64(_) => IType::F64,
        ParsedType::Boolean(_) => IType::Boolean,
        ParsedType::Utf8Str(_) => IType::String,
        ParsedType::Utf8String(_) => IType::String,
        ParsedType::Vector(ty, _) => {
            let array_itype = ptype_to_itype_unchecked(ty, wit_resolver);
            IType::Array(Box::new(array_itype))
        }
        ParsedType::Record(record_name, _) => {
            let record_type_id = wit_resolver.get_record_type_id_unchecked(record_name);
            IType::Record(record_type_id as _)
        }
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
