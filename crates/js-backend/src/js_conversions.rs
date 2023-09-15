/*
 * Copyright 2023 Fluence Labs Limited
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

use marine_wasm_backend_traits::WType;
use marine_wasm_backend_traits::WValue;

use wasm_bindgen::JsValue;

pub(crate) fn js_from_wval(val: &WValue) -> JsValue {
    match val {
        WValue::I32(val) => (*val).into(),
        WValue::I64(val) => (*val).into(),
        WValue::F32(val) => (*val).into(),
        WValue::F64(val) => (*val).into(),
    }
}

pub(crate) fn js_array_from_wval_array(values: &[WValue]) -> js_sys::Array {
    js_sys::Array::from_iter(values.iter().map(js_from_wval))
}

pub(crate) fn wval_to_i32(val: &WValue) -> i32 {
    match val {
        WValue::I32(val) => *val as _,
        WValue::I64(val) => *val as _,
        WValue::F32(val) => *val as _,
        WValue::F64(val) => *val as _,
    }
}

pub(crate) fn wval_from_js(ty: &WType, value: &JsValue) -> WValue {
    match ty {
        WType::I32 => WValue::I32(value.as_f64().unwrap() as _),
        WType::I64 => WValue::I64(value.clone().try_into().unwrap()),
        WType::F32 => WValue::F32(value.as_f64().unwrap() as _),
        WType::F64 => WValue::F64(value.as_f64().unwrap() as _),
        WType::V128 => panic!("V128 is unsupported here"),
        WType::ExternRef => panic!("ExternRef is unsupported here"),
        WType::FuncRef => panic!("FuncRef is unsupported here"),
    }
}

pub(crate) fn wval_array_from_js_array<'a>(
    js_values: &js_sys::Array,
    types: impl Iterator<Item = &'a WType>,
) -> Vec<WValue> {
    types
        .enumerate()
        .map(|(index, ty)| wval_from_js(ty, &js_values.get(index as u32)))
        .collect::<Vec<_>>()
}
