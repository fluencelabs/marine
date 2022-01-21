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
#![warn(rust_2018_idioms)]
#![feature(get_mut_unchecked)]
#![feature(new_uninit)]
#![feature(stmt_expr_attributes)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

pub(crate) mod marine_js;
mod engine;
mod errors;
mod misc;
mod module;
mod faas;
mod config;

use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use module::MModule;

pub use engine::MModuleInterface;
pub use engine::Marine;
pub use errors::MError;
pub use module::IValue;
pub use module::IRecordType;
pub use module::IFunctionArg;
pub use module::IType;
pub use module::MRecordTypes;
pub use module::MFunctionSignature;
pub use module::from_interface_values;
pub use module::to_interface_value;
pub use wasmer_it::IRecordFieldType;
pub use config::MModuleConfig;
pub use config::HostImportDescriptor;
use crate::faas::FluenceFaaS;
use marine_rs_sdk::CallParameters;

use once_cell::sync::Lazy;

use std::str::FromStr;
pub use wasmer_it::ne_vec;


pub(crate) type MResult<T> = std::result::Result<T, MError>;

pub(crate) use it_lilo::value_der;

static MINIMAL_SUPPORTED_IT_VERSION: Lazy<semver::Version> = Lazy::new(|| {
    semver::Version::from_str("0.20.0").expect("invalid minimal sdk version specified")
});

// These locals intended for check that set versions are correct at the start of an application.
thread_local!(static MINIMAL_SUPPORTED_IT_VERSION_CHECK: &'static semver::Version = Lazy::force(&MINIMAL_SUPPORTED_IT_VERSION));
thread_local!(static MODULES: RefCell<Option<FluenceFaaS>> = RefCell::new(None));
thread_local!(static INSTANCE: RefCell<Option<JsValue>> = RefCell::new(None));

/// Return minimal support version of interface types.
pub fn min_it_version() -> &'static semver::Version {
    Lazy::force(&MINIMAL_SUPPORTED_IT_VERSION)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn js_log(_s: &str) {
    //log(_s)
}

#[wasm_bindgen(getter_with_clone)]
pub struct RegisterModuleResult {
    pub error: String,
}

#[wasm_bindgen(getter_with_clone)]
pub struct CallModuleResult {
    pub result: String,
    pub error: String,
}

#[wasm_bindgen]
pub fn register_module(name: &str, wit_section_bytes: &[u8], wasm_instance: JsValue) -> RegisterModuleResult {
    console_error_panic_hook::set_once();
    let mut map = HashMap::new();
    map.insert(name.to_string(), Vec::<u8>::from(wit_section_bytes));
    let faas = match FluenceFaaS::with_modules(map) {
        Ok(faas) => faas,
        Err(e) => return RegisterModuleResult{ error: e.to_string() }
    };

    MODULES.with(|modules| modules.replace(Some(faas)));

    INSTANCE.with(|instance| instance.replace(Some(wasm_instance)));

    RegisterModuleResult { error: "".to_string() }
}

#[wasm_bindgen]
pub fn call_module(module_name: &str, function_name: &str, args: &str) -> CallModuleResult {
    js_log("ar123123");
    js_log(&format!(
        "call_module called with args: module_name={}, function_name={}, args={}",
        module_name, function_name, args
    ));
    MODULES.with(|modules| {
        let mut modules = modules.borrow_mut();
        match modules.as_mut() {
            Some(modules) => {
                js_log(&format!(
                    "call_module called with args: module_name={}, function_name={}, args={}",
                    module_name, function_name, args
                ));
                let args: serde_json::Value = match serde_json::from_str(args) {
                    Ok(args) => args,
                    Err(e) => return CallModuleResult {
                        result: "".to_string(),
                        error: format!("Error deserializing args: {}", e),
                    }
                };

                match modules
                    .call_with_json(module_name, function_name, args, CallParameters::default()) {
                        Ok(result) => {
                            CallModuleResult {
                                result: result.to_string(),
                                error: "".to_string(),
                            }
                        },
                        Err(e) => {
                            CallModuleResult {
                                result: "".to_string(),
                                error: format!("Error calling module function: {}", e),
                            }
                        }
                }
            }
            None => {
                CallModuleResult {
                    result: "".to_string(),
                    error: "attempt to run a function when module is not loaded".to_string(),
                }
            }
        }
    })
}