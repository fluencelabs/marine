use wasmer::{AsStoreMut, AsStoreRef, FunctionEnv};
use crate::{
    WasmerBackend, WasmerContextMut, generic_val_to_wasmer_val, wasmer_val_to_generic_val,
    generic_ty_to_wasmer_ty, function_type_to_func_sig, func_sig_to_function_type, WasmerCaller,
};

use marine_wasm_backend_traits::*;

pub struct WasmerFunction {
    pub(crate) sig: FuncSig,
    pub(crate) inner: wasmer::Function,
}

impl Function<WasmerBackend> for WasmerFunction {
    fn new<F>(store: &mut impl AsContextMut<WasmerBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static,
    {
        let ty = func_sig_to_function_type(&sig);
        let func =
            move |args: &[wasmer::Value]| -> Result<Vec<wasmer::Value>, wasmer::RuntimeError> {
                let args = wasmer_val_to_generic_val(args);
                let results = func(&args);
                let results = generic_val_to_wasmer_val(&results);
                Ok(results)
            };

        let func = wasmer::Function::new(&mut store.as_context_mut().inner, ty, func);
        Self { sig, inner: func }
    }

    fn new_with_ctx<F>(ctx: &mut impl AsContextMut<WasmerBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(<WasmerBackend as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static,
    {
        let ty = func_sig_to_function_type(&sig);
        let env = FunctionEnv::new(&mut ctx.as_context_mut().inner, ());
        let func = move |env: wasmer::FunctionEnvMut<()>,
                         args: &[wasmer::Value]|
              -> Result<Vec<wasmer::Value>, wasmer::RuntimeError> {
            let caller = WasmerCaller { inner: env };

            let args = wasmer_val_to_generic_val(args);
            let results = func(caller, &args);
            let results = generic_val_to_wasmer_val(&results);
            Ok(results)
        };

        let func = wasmer::Function::new_with_env(&mut ctx.as_context_mut().inner, &env, ty, func);
        Self { sig, inner: func }
    }

    fn signature<'c>(&self, _ctx: &mut impl AsContextMut<WasmerBackend>) -> &FuncSig {
        &self.sig
    }

    fn call<'c>(
        &self,
        ctx: &mut impl AsContextMut<WasmerBackend>,
        args: &[WValue],
    ) -> CallResult<Vec<WValue>> {
        let params = generic_val_to_wasmer_val(args);
        self.inner
            .call(&mut ctx.as_context_mut().inner, &params)
            .map_err(|e| CallError::Message(format!("Wasmer failed to call function: {}", e)))
            .map(|rets| wasmer_val_to_generic_val(rets.as_ref()))
    }
}
