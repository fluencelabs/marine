/*
 * Copyright 2022 Fluence Labs Limited
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

use crate::faas::FluenceFaaS;
use crate::global_state::INSTANCE;
use crate::global_state::MODULES;

use marine_rs_sdk::CallParameters;

use wasm_bindgen::prelude::*;
use serde_json::Value as JValue;
use serde::{Serialize, Deserialize};
use maplit::hashmap;

#[derive(Serialize, Deserialize)]
struct RegisterModuleResult {
    error: String,
}

#[derive(Serialize, Deserialize)]
struct CallModuleResult {
    error: String,
    result: JValue,
}

/// Registers a module insite web-runtime.
///
/// # Arguments
///
/// * `name` - name of module to register
/// * `wit_section_bytes` - bytes of "interface-types" custom section from wasm file
/// * `instance` - `WebAssembly::Instance` made from target wasm file
///
/// # Return value
///
/// JSON object with field "error". If error is empty, module is registered.
/// otherwise, it contaits error message.
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub fn register_module(name: &str, wit_section_bytes: &[u8], wasm_instance: JsValue) -> String {
    let modules = hashmap! {
        name.to_string() => wit_section_bytes.to_vec(),
    };

    let faas = match FluenceFaaS::with_modules(modules) {
        Ok(faas) => faas,
        Err(e) => return make_register_module_result(e.to_string().as_str()),
    };

    MODULES.with(|modules| modules.replace(Some(faas)));

    INSTANCE.with(|instance| instance.replace(Some(wasm_instance)));

    make_register_module_result("")
}

///  Calls a function from a module.
///
/// # Arguments
///
/// * module_name - name of registered module
/// * function_name - name of the function to call
/// * args - JSON array of function arguments
///
/// # Return value
///
/// JSON object with fields "error" and "result". If "error" is empty string,
/// "result" contains a function return value. Othervise, "error" contains error message.
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub fn call_module(module_name: &str, function_name: &str, args: &str) -> String {
    MODULES.with(|modules| {
        let mut modules = modules.borrow_mut();
        let modules = match modules.as_mut() {
            Some(modules) => modules,
            None => {
                return make_call_module_result(
                    serde_json::Value::Null,
                    "attempt to run a function when module is not loaded",
                )
            }
        };

        let args: serde_json::Value = match serde_json::from_str(args) {
            Ok(args) => args,
            Err(e) => {
                return make_call_module_result(
                    serde_json::Value::Null,
                    &format!("Error deserializing args: {}", e),
                )
            }
        };

        match modules.call_with_json(module_name, function_name, args, CallParameters::default()) {
            Ok(result) => make_call_module_result(result, ""),
            Err(e) => make_call_module_result(
                serde_json::Value::Null,
                &format!("Error calling module function: {}", e),
            ),
        }
    })
}

#[allow(unused)] // needed because clippy marks this function as unused
fn make_register_module_result(error: &str) -> String {
    let result = RegisterModuleResult {
        error: error.to_string(),
    };

    // unwrap is safe because Serialize is derived for that struct and it does not contain maps with non-string keys
    serde_json::ser::to_string(&result).unwrap()
}

#[allow(unused)] // needed because clippy marks this function as unused
fn make_call_module_result(result: serde_json::Value, error: &str) -> String {
    let result = CallModuleResult {
        error: error.to_string(),
        result,
    };

    // unwrap is safe because Serialize is derived for that struct and it does not contain maps with non-string keys
    serde_json::ser::to_string(&result).unwrap()
}
