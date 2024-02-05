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

use marine_core::MarineCore;
use marine_core::MarineCoreConfig;
use marine_core::IValue;
use marine_wasm_backend_traits::WasmBackend;
use marine_wasmtime_backend::WasmtimeWasmBackend;

use once_cell::sync::Lazy;

static GREETING_WASM_BYTES: Lazy<Vec<u8>> = Lazy::new(|| {
    std::fs::read("../examples/greeting/artifacts/greeting.wasm")
        .expect("../examples/greeting/artifacts/greeting.wasm should presence")
});

#[tokio::test]
pub async fn greeting_basic() {
    let backend = WasmtimeWasmBackend::new_async().unwrap();
    let mut marine_core = MarineCore::new(MarineCoreConfig::new(backend, None)).unwrap();
    marine_core
        .load_module("greeting", &GREETING_WASM_BYTES, <_>::default())
        .await
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    let result1 = marine_core
        .call_async(
            "greeting",
            "greeting",
            &[IValue::String(String::from("Fluence"))],
        )
        .await
        .unwrap_or_else(|e| panic!("can't invoke greeting: {:?}", e));

    let result2 = marine_core
        .call_async("greeting", "greeting", &[IValue::String(String::from(""))])
        .await
        .unwrap_or_else(|e| panic!("can't invoke greeting: {:?}", e));

    assert_eq!(result1, vec![IValue::String(String::from("Hi, Fluence"))]);
    assert_eq!(result2, vec![IValue::String(String::from("Hi, "))]);
}

#[tokio::test]
// test loading module with the same name twice
pub async fn non_unique_module_name() {
    let backend = WasmtimeWasmBackend::new_async().unwrap();
    let mut marine_core = MarineCore::new(MarineCoreConfig::new(backend, None)).unwrap();
    let module_name = String::from("greeting");
    marine_core
        .load_module(&module_name, &GREETING_WASM_BYTES, <_>::default())
        .await
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    let load_result = marine_core
        .load_module(&module_name, &GREETING_WASM_BYTES, <_>::default())
        .await;
    assert!(load_result.is_err());
    assert!(std::matches!(
        load_result.err().unwrap(),
        marine_core::MError::NonUniqueModuleName(_)
    ));
}

#[tokio::test]
#[allow(unused_variables)]
// test calling Marine with non-exist module and function names
pub async fn non_exist_module_func() {
    let backend = WasmtimeWasmBackend::new_async().unwrap();
    let mut marine_core = MarineCore::new(MarineCoreConfig::new(backend, None)).unwrap();
    marine_core
        .load_module("greeting", &GREETING_WASM_BYTES, <_>::default())
        .await
        .unwrap_or_else(|e| panic!("can't load a module into Marine: {:?}", e));

    let module_name = "greeting";
    let function_name = "greeting";
    let non_exist_name = String::from("_");

    let call_result1 = marine_core
        .call_async(
            non_exist_name.as_str(),
            function_name,
            &[IValue::String(String::from("Fluence"))],
        )
        .await;

    let call_result2 = marine_core
        .call_async(
            module_name,
            non_exist_name.as_str(),
            &[IValue::String(String::from("Fluence"))],
        )
        .await;

    let call_result3 = marine_core
        .call_async(
            non_exist_name.as_str(),
            non_exist_name.as_str(),
            &[IValue::String(String::from("Fluence"))],
        )
        .await;

    assert!(call_result1.is_err());
    assert!(matches!(
        call_result1.err().unwrap(),
        marine_core::MError::NoSuchModule(non_exist_name)
    ));

    assert!(call_result2.is_err());
    assert!(matches!(
        call_result2.err().unwrap(),
        marine_core::MError::NoSuchFunction(module_name, non_exist_name)
    ));

    assert!(call_result3.is_err());
    // at first, the module name should be checked
    assert!(matches!(
        call_result3.err().unwrap(),
        marine_core::MError::NoSuchModule(non_exist_name)
    ));
}
