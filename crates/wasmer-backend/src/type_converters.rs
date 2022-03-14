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
use wasmer_runtime::Value as WValue;
use wasmer_runtime::types::Type as WType;

use marine_wasm_backend_traits::WValue as GeneralWValue;
use marine_wasm_backend_traits::WType as GeneralWType;

pub(super) fn wtype_to_general_wtype(ty: &WType) -> GeneralWType {
    match ty {
        WType::I32 => GeneralWType::I32,
        WType::I64 => GeneralWType::I64,
        WType::F32 => GeneralWType::F32,
        WType::F64 => GeneralWType::F64,
        WType::V128 => unimplemented!(),
    }
}

pub(super) fn general_wtype_to_wtype(ty: &GeneralWType) -> WType {
    match ty {
        GeneralWType::I32 => WType::I32,
        GeneralWType::I64 => WType::I64,
        GeneralWType::F32 => WType::F32,
        GeneralWType::F64 => WType::F64,
        ty => {
            eprintln!("trying to convert {:?}", ty);
            unimplemented!()
        }
    }
}

pub(super) fn general_wval_to_wval(value: &GeneralWValue) -> WValue {
    match value {
        GeneralWValue::I32(v) => WValue::I32(*v),
        GeneralWValue::I64(v) => WValue::I64(*v),
        GeneralWValue::F32(v) => WValue::F32(*v),
        GeneralWValue::F64(v) => WValue::F64(*v),
        _ => unimplemented!(),
    }
}

pub(super) fn wval_to_general_wval(value: &WValue) -> GeneralWValue {
    match value {
        WValue::I32(v) => GeneralWValue::I32(*v),
        WValue::I64(v) => GeneralWValue::I64(*v),
        WValue::F32(v) => GeneralWValue::F32(*v),
        WValue::F64(v) => GeneralWValue::F64(*v),
        _ => unimplemented!(),
    }
}
