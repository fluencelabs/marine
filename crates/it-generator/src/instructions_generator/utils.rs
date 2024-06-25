/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::IType;
use crate::instructions_generator::ITResolver;
use crate::Result;

use marine_macro_impl::FnSignature;
use marine_macro_impl::ParsedType;
use marine_macro_impl::RustType;
use wasmer_it::ast::FunctionArg as IFunctionArg;

use std::sync::Arc;

// return error if there is no record with such name
pub(crate) fn ptype_to_itype_checked(
    pty: &ParsedType,
    it_resolver: &mut ITResolver<'_>,
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
            let array_itype = ptype_to_itype_checked(ty, it_resolver)?;
            if let IType::U8 = array_itype {
                Ok(IType::ByteArray)
            } else {
                Ok(IType::Array(Box::new(array_itype)))
            }
        }
        ParsedType::Record(record_name, _) => {
            let record_type_id = it_resolver.get_record_type_id(record_name)?;
            Ok(IType::Record(record_type_id as _))
        }
    }
}

pub(crate) fn ptype_to_itype_unchecked(
    pty: &ParsedType,
    it_resolver: &mut ITResolver<'_>,
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
            let array_itype = ptype_to_itype_unchecked(ty, it_resolver);
            if let IType::U8 = array_itype {
                IType::ByteArray
            } else {
                IType::Array(Box::new(array_itype))
            }
        }
        ParsedType::Record(record_name, _) => {
            let record_type_id = it_resolver.get_record_type_id_unchecked(record_name);
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

pub(crate) fn generate_it_args(
    signature: &FnSignature,
    it_resolver: &mut ITResolver<'_>,
) -> Result<Arc<Vec<IFunctionArg>>> {
    let arguments = signature
        .arguments
        .iter()
        .map(|arg| -> Result<IFunctionArg> {
            Ok(IFunctionArg {
                name: arg.name.clone(),
                ty: ptype_to_itype_checked(&arg.ty, it_resolver)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let arguments = Arc::new(arguments);
    Ok(arguments)
}

pub(crate) fn generate_it_output_type(
    signature: &FnSignature,
    it_resolver: &mut ITResolver<'_>,
) -> Result<Arc<Vec<IType>>> {
    let output_types = signature
        .output_types
        .iter()
        .map(|ty| ptype_to_itype_checked(ty, it_resolver))
        .collect::<Result<Vec<_>>>()?;

    let output_types = Arc::new(output_types);

    Ok(output_types)
}
