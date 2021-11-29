#![allow(unused_attributes)]

use wasm_bindgen::prelude::*;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use it_utils::{MemSlice2, ByteAccess, MemSlice3};
use marine_it_interfaces::MITInterfaces;
use crate::js_log;
use crate::module::type_converters::{itypes_args_to_wtypes, itypes_output_to_wtypes};

// marine-related imports
#[wasm_bindgen(module = "/marine-js.js")]
extern "C" {
    pub fn call_export(module_name: &str, export_name: &str, args: &str) -> String;
    pub fn read_memory(module_name: &str, module_offset: usize, module_len: usize) -> Vec<u8>;
    pub fn write_memory(module_name: &str, module_offset: usize, data: &[u8]) -> i32;

    pub fn read_byte(module_name: &str, module_offset: usize) -> u8;
    pub fn write_byte(module_name: &str, module_offset: usize, value: u8);
    pub fn get_memory_size(module_name: &str) -> i32;
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
    pub module_name: String,
}

impl Instance {
    pub fn new(mit: &MITInterfaces, module_name: String) -> Self {
        Self {
            exports: Exports::new(mit, module_name.clone()),
            module_name,
        }
    }

    pub fn exports(&self) -> ExportIter {
        ExportIter::new(&self.exports)
    }
}

pub struct DynFunc<'a> {
    pub(crate) signature: FuncSig,
    pub name: String,
    pub module_name: String,
    //pub(crate) instance_inner: &'a InstanceInner,
    //func_index: FuncIndex,
    data3: PhantomData<&'a i32>,
}

impl<'a> DynFunc<'_> {
    pub fn signature(&self) -> &FuncSig {
        &self.signature
    }

    pub fn call(&self, args: &[WValue]) -> Result<Vec<WValue>, String> {
        crate::js_log(&format!(
            "called DynFunc::call name=({}) with n args {}",
            self.name,
            args.len()
        ));

        let result = serde_json::ser::to_string(args);
        if let Err(e) = result {
            js_log(&format!("cannot serialize: {}", e));
            return Err("cannot serialize".to_string());
        }

        let args = result.unwrap();
        let output = call_export(&self.module_name, &self.name, &args);
        js_log(&format!("DynFunc::Call got result json {}", output));

        let value = serde_json::de::from_str::<serde_json::Value>(&output);
        match value {
            Ok(serde_json::Value::Array(values)) => {
                let values = values
                    .iter()
                    .map(|value| WValue::I32(value.as_i64().unwrap() as i32))
                    .collect::<Vec<_>>();
                Ok(values)
            }
            _ => {
                js_log("invalid_json got");
                Err("invalid json got".to_string())
            }
        }
    }
}

#[derive(Clone)]
pub enum Export {
    Memory,
    Function(ProcessedExport),
}

impl Export {
    pub fn name(&self) -> String {
        match self {
            Self::Memory => "memory".to_string(),
            Self::Function(func) => func.name.clone(),
        }
    }
}

pub struct Exports {
    exports: Vec<Export>,
    module_name: String,
}

impl Exports {
    pub fn new(mit: &MITInterfaces, module_name: String) -> Self {
        let mut exports = mit
            .exports()
            .filter_map(|export| {
                crate::js_log(&format!(
                    "processing export {} {}",
                    export.name, export.function_type
                ));
                let fn_type = mit.type_by_idx(export.function_type).unwrap();
                crate::js_log(&format!("got type {}", fn_type.to_string()));
                if let wasmer_it::ast::Type::Function {
                    arguments,
                    output_types,
                } = fn_type
                {
                    let mut arg_types =
                        itypes_args_to_wtypes(arguments.as_slice().iter().map(|arg| &arg.ty));
                    let output_types = itypes_output_to_wtypes(output_types.iter());
                    if export.name == "allocate" {
                        arg_types.push(WType::I32);
                    }

                    let sig = FuncSig {
                        params: Cow::Owned(arg_types),
                        returns: Cow::Owned(output_types),
                    };

                    crate::js_log(&format!("it is a function: {}", export.name.to_string()));
                    Some(Export::Function(ProcessedExport {
                        sig,
                        index: export.function_type,
                        name: export.name.to_string(),
                    }))
                } else {
                    crate::js_log(&format!("it is not a function"));
                    None
                }
            })
            .collect::<Vec<Export>>();
        exports.push(Export::Memory);
        crate::js_log(&format!("processed exports"));
        Self {
            exports,
            module_name,
        }
    }

    pub fn get(&self, name: &str) -> Result<DynFunc<'_>, String> {
        crate::js_log(&format!("Exports.get called with name {}", name));
        let export = self.exports.iter().find(|export| {
            if let Export::Function(func) = export {
                func.name == name
            } else {
                false
            }
        });
        match export {
            Some(Export::Function(function)) => Ok(DynFunc {
                signature: function.sig.clone(),
                name: function.name.clone(),
                module_name: self.module_name.clone(),
                data3: Default::default(),
            }),
            Some(_) | None => Err(format!("cannot find export {}", name)),
        }
    }
}

#[derive(Clone)]
pub struct ProcessedExport {
    sig: FuncSig,
    index: u32,
    name: String,
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    //_data: PhantomData<&'a i32>,
    exports: &'a Exports,
    index: usize,
}

impl<'a> ExportIter<'a> {
    pub(crate) fn new(exports: &'a Exports) -> Self {
        Self {
            //_data: PhantomData::<&'a i32>::default(),
            exports,
            index: 0,
        }
    }
}

impl<'a> Iterator for ExportIter<'a> {
    type Item = (String, Export);
    fn next(&mut self) -> Option<(String, Export)> {
        let export = self.exports.exports.get(self.index);
        self.index += 1;
        export.map(|export| (export.name(), export.clone()))
    }
}

#[derive(Clone)]
pub struct WasmMemory {
    pub module_name: String,
}

impl MemSlice3 for WasmMemory {
    fn len(&self) -> usize {
        //crate::js_log("WasmMemory::len calledx");
        get_memory_size(&self.module_name) as usize
    }

    fn index(&self, index: usize) -> it_utils::ByteAccess {
        //crate::js_log(&format!("WasmMemory::index called with {}", index));
        ByteAccess {
            slice: MemSlice2 { slice_ref: self },
            index,
        }
    }

    fn get(&self, index: usize) -> u8 {
        //crate::js_log(&format!("WasmMemory::get called with {}", index));
        read_byte(&self.module_name, index)
    }

    fn set(&self, index: usize, value: u8) {
        //crate::js_log(&format!("WasmMemory::set called with {} {}", index, value));
        write_byte(&self.module_name, index, value);
    }

    fn range_iter(&self, begin: usize, end: usize) -> it_utils::MemSliceIter {
        //crate::js_log(&format!("WasmMemory::range_iter called with {} {}", begin, end));
        it_utils::MemSliceIter {
            begin,
            end,
            slice: MemSlice2 { slice_ref: self },
        }
    }
}
