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

use crate::module::type_converters::{itypes_args_to_wtypes, itypes_output_to_wtypes};
use crate::global_state::INSTANCE;

use marine_it_interfaces::MITInterfaces;
use wasmer_it::ast::FunctionArg;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JValue;
use std::borrow::{Cow};
use std::rc::Rc;

const ALLOCATE_FUNC_NAME: &str = "allocate";

// marine-related imports
#[wasm_bindgen(module = "/marine-js.js")]
extern "C" {
    pub fn call_export(module_name: &JsValue, export_name: &str, args: &str) -> String;
    pub fn write_byte(module_name: &JsValue, module_offset: usize, value: u8);
    pub fn read_byte(module_name: &JsValue, module_offset: usize) -> u8;
    pub fn get_memory_size(module_name: &JsValue) -> i32;
    pub fn read_byte_range(module_name: &JsValue, module_offset: usize, slice: &mut [u8]);
    pub fn write_byte_range(module_name: &JsValue, module_offset: usize, slice: &[u8]);
}

#[derive(Clone)]
pub struct FuncSig {
    params: Cow<'static, [WType]>,
    returns: Cow<'static, [WType]>,
}

impl FuncSig {
    pub fn params(&self) -> &[WType] {
        &self.params
    }

    pub fn returns(&self) -> &[WType] {
        &self.returns
    }
}

pub struct Instance {
    pub exports: Exports,
    pub module_name: Rc<String>,
}

impl Instance {
    pub fn new(mit: &MITInterfaces<'_>, module_name: Rc<String>) -> Self {
        Self {
            exports: Exports::new(mit, module_name.clone()),
            module_name,
        }
    }

    pub fn exports(&self) -> ExportIter<'_> {
        ExportIter::new(&self.exports)
    }
}

pub struct DynFunc {
    pub(crate) signature: FuncSig,
    pub name: Rc<String>,
    pub module_name: Rc<String>,
}

impl DynFunc {
    pub fn signature(&self) -> &FuncSig {
        &self.signature
    }

    pub fn call(&self, args: &[WValue]) -> Result<Vec<WValue>, String> {
        let args = match serde_json::ser::to_string(args) {
            Ok(args) => args,
            Err(e) => return Err(format!("cannot serialize call arguments, error: {}", e)),
        };

        // .unwrap() here is safe because this method can be called only if MODULES
        // is Some, and register_module sets MODULES and INSTANCE to Some at the same time.
        // And at the same time they are set to NONE at the start of the application
        let output = INSTANCE
            .with(|instance| call_export(instance.borrow().as_ref().unwrap(), &self.name, &args));

        let value = serde_json::de::from_str::<JValue>(&output);
        match value {
            Ok(JValue::Array(values)) => {
                let values = values
                    .iter()
                    .map(|value| WValue::I32(value.as_i64().unwrap() as i32))
                    .collect::<Vec<_>>();
                Ok(values)
            }
            _ => Err("invalid json got".to_string()),
        }
    }
}

#[derive(Clone)]
pub enum Export {
    Memory,
    Function(ProcessedExport),
}

impl Export {
    pub fn name(&self) -> &str {
        match self {
            Self::Memory => "memory",
            Self::Function(func) => &func.name,
        }
    }
}

pub struct Exports {
    exports: Vec<Export>,
    module_name: Rc<String>,
}

impl Exports {
    pub fn new(mit: &MITInterfaces<'_>, module_name: Rc<String>) -> Self {
        let mut exports = mit
            .exports()
            .filter_map(|export| Self::process_export(export, mit))
            .collect::<Vec<Export>>();

        // Exports in marine-web are extracted from interface-definition. It is a hack, it is used
        // because extracting exports from JS is harder than extracting it from interface-types.
        // But interface-types do not have a "memory" export, so it is added here manually.
        // TODO: refactor when wasm module creation is fully in control of marine-web.
        exports.push(Export::Memory);

        Self {
            exports,
            module_name,
        }
    }

    fn process_export(
        export: &wasmer_it::ast::Export<'_>,
        mit: &MITInterfaces<'_>,
    ) -> Option<Export> {
        use wasmer_it::ast::Type;
        match mit.type_by_idx(export.function_type) {
            Some(Type::Function {
                arguments,
                output_types,
            }) => Some(Self::process_export_function(
                arguments.as_slice(),
                output_types.as_slice(),
                export.name,
            )),
            Some(_) => None,
            None => unreachable!("code should not reach that arm"),
        }
    }

    fn process_export_function(
        arguments: &[FunctionArg],
        output_types: &[wasmer_it::IType],
        function_name: &str,
    ) -> Export {
        let mut arg_types = itypes_args_to_wtypes(arguments.iter().map(|arg| &arg.ty));
        let output_types = itypes_output_to_wtypes(output_types.iter());

        // raw export function as a slightly different signature: it takes also "tag" argument
        // it is used in marine-runtime, and interface-types pass an argument there
        // so here signature is updated to match the expectations
        if function_name == ALLOCATE_FUNC_NAME {
            arg_types.push(WType::I32);
        }

        let sig = FuncSig {
            params: Cow::Owned(arg_types),
            returns: Cow::Owned(output_types),
        };

        Export::Function(ProcessedExport {
            sig,
            name: Rc::new(function_name.to_string()),
        })
    }

    pub fn get(&self, name: &str) -> Result<DynFunc, String> {
        let export = self.exports.iter().find(|export| match export {
            Export::Function(func) => func.name.as_str() == name,
            _ => false,
        });

        match export {
            Some(Export::Function(function)) => Ok(DynFunc {
                signature: function.sig.clone(),
                name: function.name.clone(),
                module_name: self.module_name.clone(),
            }),
            Some(_) | None => Err(format!("cannot find export {}", name)),
        }
    }
}

#[derive(Clone)]
pub struct ProcessedExport {
    sig: FuncSig,
    name: Rc<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WType {
    /// The `i32` type.
    I32,
    /// The `i64` type.
    I64,
    /// The `f32` type.
    F32,
    /// The `f64` type.
    F64,
    /// The `v128` type.
    V128,
}

impl std::fmt::Display for WType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents a WebAssembly value.
///
/// As the number of types in WebAssembly expand,
/// this structure will expand as well.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WValue {
    /// The `i32` type.
    I32(i32),
    /// The `i64` type.
    I64(i64),
    /// The `f32` type.
    F32(f32),
    /// The `f64` type.
    F64(f64),
    /// The `v128` type.
    V128(u128),
}

/// An iterator to an instance's exports.
pub struct ExportIter<'a> {
    exports: &'a Exports,
    index: usize,
}

impl<'a> ExportIter<'a> {
    pub(crate) fn new(exports: &'a Exports) -> Self {
        Self { exports, index: 0 }
    }
}

impl<'a> Iterator for ExportIter<'a> {
    type Item = (&'a str, Export);
    fn next(&mut self) -> Option<Self::Item> {
        let export = self.exports.exports.get(self.index);
        self.index += 1;
        export.map(|export| (export.name(), export.clone()))
    }
}

#[derive(Clone)]
pub struct JsWasmMemoryProxy {
    pub module_name: Rc<String>,
}

// .unwrap() on INSTANCE in these methords is safe because they can be called only if MODULES
// is Some, and register_module sets MODULES and INSTANCE to Some at the same time.
// And at the same time they are set to NONE at the start of the application
impl JsWasmMemoryProxy {
    pub fn new(module_name: Rc<String>) -> Self {
        Self { module_name }
    }

    pub fn get(&self, index: usize) -> u8 {
        INSTANCE.with(|instance| read_byte(instance.borrow().as_ref().unwrap(), index))
    }

    pub fn set(&self, index: usize, value: u8) {
        INSTANCE.with(|instance| write_byte(instance.borrow().as_ref().unwrap(), index, value))
    }

    pub fn len(&self) -> usize {
        INSTANCE.with(|instance| get_memory_size(instance.borrow().as_ref().unwrap()) as usize)
    }

    pub fn get_range(&self, offset: usize, size: usize) -> Vec<u8> {
        INSTANCE.with(|instance| {
            let mut result = vec![0; size];
            read_byte_range(instance.borrow().as_ref().unwrap(), offset, &mut result);
            result
        })
    }

    pub fn set_range(&self, offset: usize, data: &[u8]) {
        INSTANCE.with(|instance| {
            write_byte_range(instance.borrow().as_ref().unwrap(), offset, data);
        })
    }
}
