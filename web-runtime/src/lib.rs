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
//use wasmer_it::ast::Interfaces;
//use thiserror::Error as ThisError;
use module::MModule;

//use marine_js::*;

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
//pub use config::HostExportedFunc;
use crate::faas::FluenceFaaS;
use marine_rs_sdk::CallParameters;


use once_cell::sync::Lazy;

use std::str::FromStr;
pub use wasmer_it::ne_vec;
//use crate::module::type_converters::ival_to_string;

pub(crate) type MResult<T> = std::result::Result<T, MError>;

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

// Common JS stuff
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn js_log(s: &str) {
    log(s)
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
/*
#[wasm_bindgen]
pub fn test_call_export() {
    call_export("greeting", "test_export", "[0]");
}

#[wasm_bindgen]
pub fn test_read_memory() {
    let result: u8 = read_byte("greeting", 0);
    log(&result.to_string())
}

#[wasm_bindgen]
pub fn test_write_memory() {
    write_byte("greeting", 0, 42);
}

#[wasm_bindgen]
pub fn test_it_section(bytes: &[u8]) {
    let interfaces = extract_it_from_bytes(bytes);
    let result = match interfaces {
        Ok(interfaces) => interfaces.version.to_string(),
        Err(e) => e.to_string(),
    };

    log(&result)
}
*/
#[wasm_bindgen]
pub fn register_module(name: &str, wit_section_bytes: &[u8], wasm_instance: JsValue) {
    //#[allow(unused)]
        //let module = MModule::new(name, wit_section_bytes).unwrap();
    let mut map = HashMap::new();
    map.insert(name.to_string(), Vec::<u8>::from(wit_section_bytes));
    let faas = FluenceFaaS::with_modules(map).unwrap();

    MODULES.with(|modules| {
        modules.replace(Some(faas))
    });

    INSTANCE.with(|instance| {
        instance.replace(Some(wasm_instance))
    });
/*
    INSTANCE.with(|instance| {
        instance.borrow().as_ref().unwrap().to_string();
    });*/
}

/*
#[wasm_bindgen]
pub fn call(module_name: &str, function_name: &str, args: &str) -> String {
    MODULES.with(|modules| -> String {
        let mut modules = modules.borrow_mut();
        let module = match modules.get_mut(module_name) {
            Some(module) => module,
            None => {
                js_log(&format!(r#"No "{}" module in registered"#, module_name));
                unreachable!();
            }
        };

        let args = serde_json::de::from_str::<Vec<IValue>>(args).unwrap();

        let output = match module.call(
            module_name,
            function_name,
            &args,
        ) {
            Ok(output) => output,
            Err(e) => {
                crate::js_log(&format!(r#"{}.{} call error: {}"#, module_name, function_name, e));
                unreachable!();
            }
        };

        serde_json::ser::to_string(&output).unwrap()
            /*
        for out in output {
            js_log(&format!("got output: {}", ival_to_string(&out)));
        }*/
    })
}
*/
/*
#[wasm_bindgen]
pub fn test_call_avm() {
    MODULES.with(|modules| {
        let mut modules = modules.borrow_mut();
        let module = match modules.get_mut("avm") {
            Some(module) => module,
            None => {
                js_log("No AVM module in registered");
                unreachable!();
            }
        };

        let vm_peer_id = "some_vm_peer_id";

        let script = format!(
            r#"
        (seq
            (par
                (call "{0}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "{0}" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
            vm_peer_id
        );
        let air = IValue::String(script);
        let prev_data = IValue::Array(vec![]);
        let data = IValue::Array(vec![]);
        let run_parameters = IValue::Record(ne_vec![
            IValue::String("some_peer_id".to_string()),
            IValue::String("some_current_peer_id".to_string())
        ]);
        let call_results = IValue::ByteArray(Vec::from("{}".as_bytes()));

        let output = match module.call(
            "avm",
            "invoke",
            &[air, prev_data, data, run_parameters, call_results],
        ) {
            Ok(output) => output,
            Err(e) => {
                crate::js_log(&format!("invoke call error: {}", e));
                unreachable!();
            }
        };

        for out in output {
            js_log(&format!("got output: {}", ival_to_string(&out)));
        }
    })
}

#[wasm_bindgen]
pub fn test_call_greeting_array() {
    MODULES.with(|modules| {
        let mut modules = modules.borrow_mut();
        let module = match modules.get_mut("greeting") {
            Some(module) => module,
            None => {
                js_log("No AVM module in registered");
                unreachable!();
            }
        };

        //let data = IValue::Array(vec![IValue::U8(48), IValue::U8(49), IValue::U8(50), IValue::U8(51)]);
        let data = IValue::ByteArray(vec![48, 49, 50, 51]);

        let output = match module.call("greeting", "greeting_array", &[data]) {
            Ok(output) => output,
            Err(e) => {
                crate::js_log(&format!("invoke call error: {}", e));
                unreachable!();
            }
        };

        for out in output {
            js_log(&format!("got output: {}", ival_to_string(&out)));
        }
    })
}
*/
#[wasm_bindgen]
pub fn call_module(module_name: &str, function_name: &str, args: &str) -> String {
    MODULES.with(|modules| {
        let mut modules = modules.borrow_mut();
        match modules.as_mut() {
            Some(modules) => {
                let args = serde_json::from_str(args).unwrap();
                let result = modules.call_with_json(module_name, function_name, args, CallParameters::default()).unwrap();
                result.to_string()
            }
            None => {
                js_log("attempt to run a function when module is not loaded");
                unreachable!();
            }
        }
    })
}
/*
pub(crate) fn extract_it_from_bytes(wit_section_bytes: &[u8]) -> Result<Interfaces<'_>, MyError> {
    match wasmer_it::decoders::binary::parse::<(&[u8], nom::error::ErrorKind)>(wit_section_bytes) {
        Ok((remainder, it)) if remainder.is_empty() => Ok(it),
        Ok(_) => Err(MyError::ITRemainderNotEmpty),
        Err(e) => Err(MyError::CorruptedITSection(e.to_string())),
    }
}

#[derive(Debug, ThisError)]
enum MyError {
    #[error("ITRemainderNotEmpty")]
    ITRemainderNotEmpty,
    #[error("CorruptedITSection {0}")]
    CorruptedITSection(String),
}
*/