use wasmtime::{Val, ValType};
use marine_wasm_backend_traits::{WType, WValue};

pub(crate) fn val_type_to_wtype(ty: &wasmtime::ValType) -> Option<WType> {
    match ty {
        ValType::I32 => Some(WType::I32),
        ValType::I64 => Some(WType::I32),
        ValType::F32 => Some(WType::F32),
        ValType::F64 => Some(WType::F64),
        ValType::V128 => None,
        ValType::FuncRef => None,
        ValType::ExternRef => None
    }
}

pub(crate) fn wtype_to_val_type(ty: &WType) -> wasmtime::ValType {
    match ty {
        WType::I32 => wasmtime::ValType::I32,
        WType::I64 => wasmtime::ValType::I64,
        WType::F32 => wasmtime::ValType::F32,
        WType::F64 => wasmtime:: ValType::F64,
    }
}

pub(crate) fn wvalue_to_val(value: &WValue) -> wasmtime::Val {
    match value {
        WValue::I32(value) => wasmtime::Val::I32(value.clone()),
        WValue::I64(value) => wasmtime::Val::I64(value.clone()),
        WValue::F32(value) => wasmtime::Val::F32(value.to_bits()),
        WValue::F64(value) => wasmtime::Val::F64(value.to_bits()),
    }
}

pub(crate) fn val_to_wvalue(value: &wasmtime::Val) -> Result<WValue, ()> {
    match value {
        Val::I32(value) => Ok(WValue::I32(value.clone())),
        Val::I64(value) => Ok(WValue::I64(value.clone())),
        Val::F32(value) => Ok(WValue::F32(f32::from_bits(value.clone()))),
        Val::F64(value) => Ok(WValue::F64(f64::from_bits(value.clone()))),
        Val::V128(_) => Err(()),
        Val::FuncRef(_) => Err(()),
        Val::ExternRef(_) => Err(()),
    }
}