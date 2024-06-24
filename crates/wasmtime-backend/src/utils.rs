/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use marine_wasm_backend_traits::prelude::*;

use wasmtime::Val;
use wasmtime::ValType;

pub(crate) fn val_type_to_wtype(ty: &ValType) -> WType {
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

pub(crate) fn wtype_to_val_type(ty: &WType) -> ValType {
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

pub(crate) fn wvalue_to_val(value: &WValue) -> Val {
    match value {
        WValue::I32(value) => Val::I32(*value),
        WValue::I64(value) => Val::I64(*value),
        WValue::F32(value) => Val::F32(value.to_bits()),
        WValue::F64(value) => Val::F64(value.to_bits()),
    }
}

pub(crate) fn val_to_wvalue(value: &Val) -> RuntimeResult<WValue> {
    match value {
        Val::I32(value) => Ok(WValue::I32(*value)),
        Val::I64(value) => Ok(WValue::I64(*value)),
        Val::F32(value) => Ok(WValue::F32(f32::from_bits(*value))),
        Val::F64(value) => Ok(WValue::F64(f64::from_bits(*value))),
        Val::V128(_) => Err(RuntimeError::UnsupportedType(WType::V128)),
        Val::FuncRef(_) => Err(RuntimeError::UnsupportedType(WType::V128)),
        Val::ExternRef(_) => Err(RuntimeError::UnsupportedType(WType::V128)),
    }
}

pub(crate) fn sig_to_fn_ty(sig: &FuncSig) -> wasmtime::FuncType {
    let params = sig.params().iter().map(wtype_to_val_type);
    let rets = sig.returns().iter().map(wtype_to_val_type);

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

pub(crate) fn inspect_call_error(e: anyhow::Error) -> RuntimeError {
    if e.downcast_ref::<wasmtime::Trap>().is_some() {
        RuntimeError::Trap(e)
    } else {
        match e.downcast::<UserError>() {
            Ok(e) => RuntimeError::UserError(e),
            Err(e) => RuntimeError::Other(e),
        }
    }
}

pub(crate) fn inspect_instantiation_error(e: anyhow::Error) -> InstantiationError {
    if e.downcast_ref::<wasmtime::Trap>().is_some() {
        InstantiationError::RuntimeError(RuntimeError::Trap(e))
    } else {
        match e.downcast::<UserError>() {
            Ok(e) => InstantiationError::RuntimeError(RuntimeError::UserError(e)),
            Err(e) => InstantiationError::Other(e),
        }
    }
}
