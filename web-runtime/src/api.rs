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
use std::collections::HashMap;

/**
* doc comment
*/
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub fn register_module(name: &str, wit_section_bytes: &[u8], wasm_instance: JsValue) -> String {
    let mut map = HashMap::new();
    map.insert(name.to_string(), wit_section_bytes.to_vec());
    let faas = match FluenceFaaS::with_modules(map) {
        Ok(faas) => faas,
        Err(e) => return make_register_module_result(e.to_string().as_str()),
    };

    MODULES.with(|modules| modules.replace(Some(faas)));

    INSTANCE.with(|instance| instance.replace(Some(wasm_instance)));

    return make_register_module_result("");
}

/**
 * doc comment
 */
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
    serde_json::json!({ "error": error }).to_string()
}

#[allow(unused)] // needed because clippy marks this function as unused
fn make_call_module_result(result: serde_json::Value, error: &str) -> String {
    serde_json::json!({
        "result": result,
        "error": error,
    })
    .to_string()
}
