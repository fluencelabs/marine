/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::it_interface::MRecordTypes;

use wasmer_it::IType;

/// Converts the supplied IType to a Aqua0compatible text representation.
///
/// SAFETY:
///     It's assumed that arguments are well-formed and all records have a corresponded type in
///     record_types.
pub fn itype_text_view(arg_ty: &IType, record_types: &MRecordTypes) -> String {
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
