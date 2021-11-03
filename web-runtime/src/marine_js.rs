#![allow(unused_attributes)]

use wasm_bindgen::prelude::*;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// marine-related imports
#[wasm_bindgen(module = "/marine-js.js")]
extern {
    pub fn call_export(module_name: &str, export_name: &str) -> i32;
    pub fn read_memory(module_name: &str, module_offset: usize, module_len: usize) -> Vec<u8>;
    pub fn write_memory(module_name: &str, module_offset: usize, data: &[u8]) -> i32;
}



pub struct Ctx {}

pub struct Func<'a, Args: 'a, Rets> {
    data: PhantomData<Args>,
    data2: PhantomData<Rets>,
    data3: PhantomData<&'a i32>
}

pub struct DynamicFunc<'a> {
    data: PhantomData<&'a i32>
}

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
    pub exports: Exports
}

impl Instance {
    pub fn exports(&self) -> &Exports {
        &self.exports
    }
}

pub struct DynFunc<'a> {
    pub(crate) signature: FuncSig,
    //pub(crate) instance_inner: &'a InstanceInner,
    //func_index: FuncIndex,
    data3: PhantomData<&'a i32>
}

impl<'a> DynFunc<'_> {
    pub fn signature(&self) -> &FuncSig {
        &self.signature
    }

    pub fn call(&self, _args: &[WValue]) -> Result<Vec<WValue>, String> {
        Err("not implemented".to_string())
    }
}


pub struct SigRegistry {}
pub struct ExportIndex {}
pub struct WasmTypeList {}
pub struct ResolveError {}
pub struct LocalOrImport {}
pub struct Namespace {}
pub struct ImportObject {}
pub struct Module {}
pub struct LocalMemory {}

pub enum Export {
    Memory(i32)
}

pub struct Exports {
    some_export: DynFunc<'static>
}

impl Exports {
    pub fn get(&self, _name: &str) -> Result<DynFunc<'_>, String> {
        Ok(self.some_export)
    }
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

impl WValue {
    /// The `Type` of this `Value`.
    pub fn ty(&self) -> WType {
        match self {
            WValue::I32(_) => WType::I32,
            WValue::I64(_) => WType::I64,
            WValue::F32(_) => WType::F32,
            WValue::F64(_) => WType::F64,
            WValue::V128(_) => WType::V128,
        }
    }

    /// Convert this `Value` to a u128 binary representation.
    pub fn to_u128(&self) -> u128 {
        match *self {
            WValue::I32(x) => x as u128,
            WValue::I64(x) => x as u128,
            WValue::F32(x) => f32::to_bits(x) as u128,
            WValue::F64(x) => f64::to_bits(x) as u128,
            WValue::V128(x) => x,
        }
    }
}

