/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::it_interface::IRecordTypes;

use wasmer_it::IType;

/// Converts the supplied IType to a Aqua0compatible text representation.
///
/// SAFETY:
///     It's assumed that arguments are well-formed and all records have a corresponded type in
///     record_types.
pub fn itype_text_view(arg_ty: &IType, record_types: &IRecordTypes) -> String {
    match arg_ty {
        IType::Record(record_type_id) => {
            // assumed that this functions called with well-formed args
            let record = record_types.get(record_type_id).unwrap();
            record.name.clone()
        }
        IType::Array(array_ty) => format!("[]{}", itype_text_view(array_ty, record_types)),
        IType::Boolean => "bool".to_string(),
        IType::S8 => "i8".to_string(),
        IType::S16 => "i16".to_string(),
        IType::S32 => "i32".to_string(),
        IType::S64 => "i64".to_string(),
        IType::U8 => "u8".to_string(),
        IType::U16 => "u16".to_string(),
        IType::U32 => "u32".to_string(),
        IType::U64 => "u64".to_string(),
        IType::F32 => "f32".to_string(),
        IType::F64 => "f64".to_string(),
        IType::String => "string".to_string(),
        IType::ByteArray => "[]u8".to_string(),
        IType::I32 => "i32".to_string(),
        IType::I64 => "i64".to_string(),
    }
}
