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

use crate::global_state::MARINE;


use marine_rs_sdk::CallParameters;

use wasm_bindgen::prelude::*;
use serde_json::Value as JValue;
use serde::Serialize;
use serde::Deserialize;

use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use marine::generic::Marine;
use marine::generic::MarineConfig;
use marine::generic::MarineModuleConfig;
use marine::generic::ModuleDescriptor;
use marine_js_backend::JsWasmBackend;

#[derive(Serialize, Deserialize)]
struct RegisterModuleResult {
    error: String,
}

#[derive(Serialize, Deserialize)]
struct CallModuleResult {
    error: String,
    result: JValue,
}

/// Registers a module inside web-runtime.
///
/// # Arguments
///
/// * `name` - name of module to register
/// * `wasm_bytes` - wasm file bytes
///
/// # Return value
///
/// JSON object with field "error". If error is empty, module is registered.
/// otherwise, it contains error message.
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub fn register_module(name: &str, wasm_bytes: &[u8]) -> String {
    log::debug!("register_module start");
    let modules = maplit::hashmap! {
            name.to_string() => wasm_bytes.to_owned()
        };

    let module_config = MarineModuleConfig {
        mem_pages_count: None,
        max_heap_size: None,
        logger_enabled: false,
        host_imports: Default::default(),
        wasi: None,
        logging_mask: 0
    };

    let module_descriptor = ModuleDescriptor {
        load_from: None,
        file_name: name.to_string(),
        import_name: name.to_string(),
        config: module_config
    };

    let config = MarineConfig {
        modules_dir: None,
        modules_config: vec![module_descriptor],
        default_modules_config: None
    };

    let marine = Marine::<JsWasmBackend>::with_modules(modules, config);
    let new_marine = match marine {
        Err(e) => return {
            log::debug!("register_module fail: {:?}", e);
            make_register_module_result(&e.to_string())
        },
        Ok(marine) => marine,
    };

    MARINE.with(|marine| marine.replace(Some(new_marine)));

    log::debug!("register_module success");
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
/// "result" contains a function return value. Otherwise, "error" contains error message.
#[allow(unused)] // needed because clippy marks this function as unused
#[wasm_bindgen]
pub fn call_module(module_name: &str, function_name: &str, args: &str) -> String {
    MARINE.with(|marine| {
        let args: JValue = match serde_json::from_str(args) {
            Ok(args) => args,
            Err(e) => {
                return make_call_module_result(
                    JValue::Null,
                    &format!("Error deserializing args: {}", e),
                )
            }
        };

        if let Some(marine) = marine.borrow_mut().deref_mut() {
            match marine.call_with_json(module_name, function_name, args, <_>::default()) {
                Ok(result) => make_call_module_result(result, ""),
                Err(e) => make_call_module_result(
                    JValue::Null,
                    &format!("Error calling module function: {}", e),
                ),
            }
        } else {
            make_call_module_result(JValue::Null, "marine is not initialized")
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
fn make_call_module_result(result: JValue, error: &str) -> String {
    let result = CallModuleResult {
        error: error.to_string(),
        result,
    };

    // unwrap is safe because Serialize is derived for that struct and it does not contain maps with non-string keys
    serde_json::ser::to_string(&result).unwrap()
}
