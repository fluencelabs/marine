use crate::{WasmerBackend, WasmerContextMut, generic_val_to_wasmer_val, wasmer_val_to_generic_val};

use marine_wasm_backend_traits::*;

pub struct WasmerFunction {
    pub(crate) sig: FuncSig,
    pub(crate) inner: wasmer::Function,
}

impl Function<WasmerBackend> for WasmerFunction {
    fn signature<'c>(&self, _store: WasmerContextMut<'c>) -> &FuncSig {
        &self.sig
    }

    fn call<'c>(&self, mut store: WasmerContextMut<'c>, args: &[WValue]) -> CallResult<Vec<WValue>> {
        let params = generic_val_to_wasmer_val(args);
        self
            .inner
            .call(&mut store, &params)
            .map_err(|e| CallError::Message(format!("Wasmer failed to call function: {}", e)))
            .map(|rets| wasmer_val_to_generic_val(rets.as_ref()))
    }
}

impl From<wasmer::Function> for WasmerFunction {
    fn from(value: wasmer::Function) -> Self {
        todo!()
    }
}