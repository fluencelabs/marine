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
use fluence_sdk_wit::ParsedType;
use fluence_sdk_wit::WasmType;

pub(crate) fn ptype_to_itype(pty: &ParsedType) -> IType {
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
        ParsedType::Record(_) => unimplemented!(),
    }
}

pub(crate) fn wtype_to_itype(pty: &WasmType) -> IType {
    match pty {
        WasmType::I32 => IType::I32,
        WasmType::I64 => IType::I64,
        WasmType::F32 => IType::F32,
        WasmType::F64 => IType::F64,
    }
}
