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

use wasm_bindgen::prelude::*;
#[allow(unused)]
use wasm_bindgen::JsValue;
use wasmer_it::ast::{Interfaces, FunctionArg};
use thiserror::Error as ThisError;
#[allow(unused)]
use wasmer_it::interpreter::wasm::structures::{LocalImport, Export,Memory,MemoryView};


pub(crate) mod marine_js;
use marine_js::*;



//mod config;
mod engine;
mod errors;
//mod host_imports;
mod misc;
mod module;

mod it_interface;

//pub use config::MModuleConfig;
//pub use config::HostExportedFunc;
//pub use config::HostImportDescriptor;
//pub use engine::Marine;
pub use engine::MModuleInterface;
pub use errors::MError;
//pub use host_imports::HostImportError;
pub use module::IValue;
pub use module::IRecordType;
pub use module::IFunctionArg;
pub use module::IType;
pub use module::MRecordTypes;
pub use module::MFunctionSignature;
pub use module::from_interface_values;
pub use module::to_interface_value;

pub use wasmer_it::IRecordFieldType;
pub mod ne_vec {
    pub use wasmer_it::NEVec;
}

pub(crate) type MResult<T> = std::result::Result<T, MError>;

use once_cell::sync::Lazy;

use std::str::FromStr;
static MINIMAL_SUPPORTED_SDK_VERSION: Lazy<semver::Version> = Lazy::new(|| {
    semver::Version::from_str("0.6.0").expect("invalid minimal sdk version specified")
});
static MINIMAL_SUPPORTED_IT_VERSION: Lazy<semver::Version> = Lazy::new(|| {
    semver::Version::from_str("0.20.0").expect("invalid minimal sdk version specified")
});

// These locals intended for check that set versions are correct at the start of an application.
thread_local!(static MINIMAL_SUPPORTED_SDK_VERSION_CHECK: &'static semver::Version = Lazy::force(&MINIMAL_SUPPORTED_SDK_VERSION));
thread_local!(static MINIMAL_SUPPORTED_IT_VERSION_CHECK: &'static semver::Version = Lazy::force(&MINIMAL_SUPPORTED_IT_VERSION));

/// Return minimal support version of interface types.
pub fn min_it_version() -> &'static semver::Version {
    Lazy::force(&MINIMAL_SUPPORTED_IT_VERSION)
}

/// Return minimal support version of SDK.
pub fn min_sdk_version() -> &'static semver::Version {
    Lazy::force(&MINIMAL_SUPPORTED_SDK_VERSION)
}


pub struct JsLocalImport {

}

impl LocalImport for JsLocalImport {
    fn name(&self) -> &str {
        todo!()
    }

    fn inputs_cardinality(&self) -> usize {
        todo!()
    }

    fn outputs_cardinality(&self) -> usize {
        todo!()
    }

    fn arguments(&self) -> &[FunctionArg] {
        todo!()
    }

    fn outputs(&self) -> &[IType] {
        todo!()
    }

    fn call(&self, arguments: &[IValue]) -> Result<Vec<IValue>, ()> {
        //call_export(self.module_name, self.name())
        Err(())
    }
}

struct JsExport {
    name: String,

}

impl Export for JsExport {
    fn name(&self) -> &str {
        todo!()
    }

    fn inputs_cardinality(&self) -> usize {
        todo!()
    }

    fn outputs_cardinality(&self) -> usize {
        todo!()
    }

    fn arguments(&self) -> &[FunctionArg] {
        todo!()
    }

    fn outputs(&self) -> &[IType] {
        todo!()
    }

    fn call(&self, arguments: &[IValue]) -> Result<Vec<IValue>, ()> {
        todo!()
    }
}


// Common JS stuff
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn test_call_export() {
    call_export("greeting", "test_export");
}

#[wasm_bindgen]
pub fn test_read_memory() {
    let _result = read_memory("greeting", 0, 4);
}

#[wasm_bindgen]
pub fn test_write_memory() {
    let _result = write_memory("greeting", 0, &[0, 1, 2, 3]);
}

#[wasm_bindgen]
pub fn test_it_section(bytes: &[u8]) {
    let interfaces = extract_it_from_bytes(bytes);
    let result = match interfaces {
        Ok(interfaces) => interfaces.version.to_string(),
        Err(e) => e.to_string()
    };

    log(&result)
}

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

