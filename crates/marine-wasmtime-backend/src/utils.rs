use wasmtime::{Val, ValType};
use marine_wasm_backend_traits::{FuncSig, InstantiationError, RuntimeError, RuntimeResult, UserError, WasmBackendError, WType, WValue};

pub(crate) fn val_type_to_wtype(ty: &wasmtime::ValType) -> WType {
    match ty {
        ValType::I32 => WType::I32,
        ValType::I64 => WType::I64,
        ValType::F32 => WType::F32,
        ValType::F64 => WType::F64,
        ValType::V128 => WType::V128,
        ValType::FuncRef => WType::FuncRef,
        ValType::ExternRef => WType::ExternRef,
    }
}

pub(crate) fn wtype_to_val_type(ty: &WType) -> wasmtime::ValType {
    match ty {
        WType::I32 => ValType::I32,
        WType::I64 => ValType::I64,
        WType::F32 => ValType::F32,
        WType::F64 => ValType::F64,
        WType::V128 => ValType::V128,
        WType::FuncRef => ValType::FuncRef,
        WType::ExternRef => ValType::ExternRef,
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

pub(crate) fn val_to_wvalue(value: &wasmtime::Val) -> RuntimeResult<WValue> {
    match value {
        Val::I32(value) => Ok(WValue::I32(value.clone())),
        Val::I64(value) => Ok(WValue::I64(value.clone())),
        Val::F32(value) => Ok(WValue::F32(f32::from_bits(value.clone()))),
        Val::F64(value) => Ok(WValue::F64(f64::from_bits(value.clone()))),
        Val::V128(_) => Err(RuntimeError::UnsupportedType(WType::V128)),
        Val::FuncRef(_) => Err(RuntimeError::UnsupportedType(WType::V128)),
        Val::ExternRef(_) => Err(RuntimeError::UnsupportedType(WType::V128)),
    }
}

pub(crate) fn sig_to_fn_ty(sig: &FuncSig) -> wasmtime::FuncType {
    let params = sig.params().map(wtype_to_val_type);
    let rets = sig.returns().map(wtype_to_val_type);

    wasmtime::FuncType::new(params, rets)
}

pub(crate) fn fn_ty_to_sig(ty: &wasmtime::FuncType) -> FuncSig {
    let params = ty
        .params()
        .map(|ty| val_type_to_wtype(&ty))
        .collect::<Vec<_>>();

    let rets = ty
        .results()
        .map(|ty| val_type_to_wtype(&ty))
        .collect::<Vec<_>>();

    FuncSig::new(params, rets)
}

pub(crate) fn inspect_call_error(mut e: anyhow::Error) -> RuntimeError {
    if let Some(trap) = e.downcast_ref::<wasmtime::Trap>() {
        RuntimeError::Trap(e)
    } else {
        match e.downcast::<UserError>() {
            Ok(e) => RuntimeError::UserError(e),
            Err(e) => RuntimeError::Other(e),
        }
    }
}

pub(crate) fn inspect_instantiation_error(e: anyhow::Error) -> InstantiationError {
    if let Some(trap) = e.downcast_ref::<wasmtime::Trap>() {
        InstantiationError::RuntimeError(RuntimeError::Trap(e))
    } else {
        match e.downcast::<UserError>() {
            Ok(e) => InstantiationError::RuntimeError(RuntimeError::UserError(e)),
            Err(e) => InstantiationError::Other(e),
        }
    }
}