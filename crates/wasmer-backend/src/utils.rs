use wasmer::{FunctionType, Type, Value};
use marine_wasm_backend_traits::{FuncSig, WType, WValue};

pub(crate) fn generic_val_to_wasmer_val(vals: &[WValue]) -> Vec<wasmer::Value> {
    vals.iter()
        .map(|value| match value {
            WValue::I32(val) => wasmer::Value::I32(*val),
            WValue::I64(val) => wasmer::Value::I64(*val),
            WValue::F32(val) => wasmer::Value::F32(*val),
            WValue::F64(val) => wasmer::Value::F64(*val),
        })
        .collect()
}

pub(crate) fn wasmer_val_to_generic_val(vals: &[wasmer::Value]) -> Vec<WValue> {
    vals.iter()
        .map(|value| match value {
            Value::I32(val) => WValue::I32(*val),
            Value::I64(val) => WValue::I64(*val),
            Value::F32(val) => WValue::F32(*val),
            Value::F64(val) => WValue::F64(*val),
            Value::ExternRef(_) => {
                panic!("ExternRef values are unsupported by marine wasmer backend")
            }
            Value::FuncRef(_) => panic!("FuncRef values are unsupported by marine wasmer backend"),
            Value::V128(_) => panic!("V128 values are unsupported by marine wasmer backend"),
        })
        .collect::<Vec<WValue>>()
}

pub(crate) fn wasmer_ty_to_generic_ty<'t>(
    tys: impl Iterator<Item = &'t wasmer::Type>,
) -> Vec<WType> {
    tys.map(|ty| match ty {
        Type::I32 => WType::I32,
        Type::I64 => WType::I64,
        Type::F32 => WType::F32,
        Type::F64 => WType::F64,
        Type::V128 => panic!("V128 type is unsupported by marine wasmer backend"),
        Type::ExternRef => panic!("ExternRef type is unsupported by marine wasmer backend"),
        Type::FuncRef => panic!("FuncRef type is unsupported by marine wasmer backend"),
    })
    .collect()
}

pub(crate) fn generic_ty_to_wasmer_ty<'t>(
    tys: impl Iterator<Item = &'t WType>,
) -> Vec<wasmer::Type> {
    tys.map(|ty| match ty {
        WType::I32 => Type::I32,
        WType::I64 => Type::I64,
        WType::F32 => Type::F32,
        WType::F64 => Type::F64,
    })
    .collect()
}

pub(crate) fn func_sig_to_function_type(sig: &FuncSig) -> FunctionType {
    let params = generic_ty_to_wasmer_ty(sig.params());
    let results = generic_ty_to_wasmer_ty(sig.returns());
    FunctionType::new(params, results)
}

pub(crate) fn function_type_to_func_sig(ty: &FunctionType) -> FuncSig {
    let params = wasmer_ty_to_generic_ty(ty.params().iter());
    let results = wasmer_ty_to_generic_ty(ty.results().iter());
    FuncSig::new(params, results)
}
