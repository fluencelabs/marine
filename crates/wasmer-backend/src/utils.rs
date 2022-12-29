use wasmer::Value;
use marine_wasm_backend_traits::WValue;

pub(crate) fn generic_val_to_wasmer_val(vals: &[WValue]) -> Vec<wasmer::Value> {
    vals.iter()
        .map(|value| match value {
            WValue::I32(val) => wasmer::Value::I32(val),
            WValue::I64(val) => wasmer::Value::I64(val),
            WValue::F32(val) => wasmer::Value::F32(val),
            WValue::F64(val) => wasmer::Value::F64(val),
        })
        .collect()
}

pub(crate) fn wasmer_val_to_generic_val(vals: &[wasmer::Value]) -> Vec<WValue> {
    vals.iter()
        .map(|value: wasmer::Value| match value {
            Value::I32(val) => WValue::I32(val),
            Value::I64(val) => WValue::I64(val),
            Value::F32(val) => WValue::F32(val),
            Value::F64(val) => WValue::F64(val),
            Value::ExternRef(_) => {
                panic!("ExternRef values are unsupported by marine wasmer backend")
            }
            Value::FuncRef(_) => panic!("FuncRef values are unsupported by marine wasmer backend"),
            Value::V128(_) => panic!("V128 values are unsupported by marine wasmer backend"),
        })
        .collect()
}
