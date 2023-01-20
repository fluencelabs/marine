use anyhow::anyhow;
use wasmer::{AsStoreMut, AsStoreRef, FunctionEnv, FunctionEnvMut};
use crate::{
    WasmerBackend, WasmerContextMut, generic_val_to_wasmer_val, wasmer_val_to_generic_val,
    generic_ty_to_wasmer_ty, function_type_to_func_sig, func_sig_to_function_type, WasmerCaller,
    StoreState,
};

use marine_wasm_backend_traits::*;

pub struct WasmerFunction {
    pub(crate) sig: FuncSig,
    pub(crate) inner: wasmer::Function,
    pub(crate) owner_memory: Option<wasmer::Memory>,
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
        Self {
            sig,
            inner: func,
            owner_memory: None,
        }
    }

    fn new_with_ctx<F>(ctx: &mut impl AsContextMut<WasmerBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(<WasmerBackend as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static,
    {
        let ty = func_sig_to_function_type(&sig);
        let global_env = ctx.as_context_mut().env.clone();
        let env = FunctionEnv::new(&mut ctx.as_context_mut().inner, ());
        let func = move |env: wasmer::FunctionEnvMut<()>,
                         args: &[wasmer::Value]|
              -> Result<Vec<wasmer::Value>, wasmer::RuntimeError> {
            let caller = WasmerCaller {
                inner: env,
                env: global_env.clone(),
            };

            let args = wasmer_val_to_generic_val(args);
            let results = func(caller, &args);
            let results = generic_val_to_wasmer_val(&results);
            Ok(results)
        };

        let func = wasmer::Function::new_with_env(&mut ctx.as_context_mut().inner, &env, ty, func);
        Self {
            sig,
            inner: func,
            owner_memory: None,
        }
    }

    fn new_typed<Params, Results, Env>(
        store: &mut impl AsContextMut<WasmerBackend>,
        func: impl IntoFunc<WasmerBackend, Params, Results, Env>,
    ) -> Self {
        func.into_func(store)
    }

    fn signature<'c>(&self, _ctx: &mut impl AsContextMut<WasmerBackend>) -> FuncSig {
        self.sig.clone()
    }

    fn call<'c>(
        &self,
        ctx: &mut impl AsContextMut<WasmerBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        let mut ctx = ctx.as_context_mut();
        let prev_memory = ctx.env.as_mut(&mut ctx.inner).current_memory.clone();
        ctx.env.as_mut(&mut ctx.inner).current_memory = self.owner_memory.clone();

        let params = generic_val_to_wasmer_val(args);
        let result = self
            .inner
            .call(&mut ctx.inner, &params)
            .map_err(|e| RuntimeError::Other(anyhow!("Wasmer failed to call function: {}", e))) // todo make detailed
            .map(|rets| wasmer_val_to_generic_val(rets.as_ref()));

        ctx.env.as_mut(&mut ctx.inner).current_memory = prev_memory;
        result
    }
}

impl WasmerFunction {
    pub(crate) fn from_func(
        mut ctx: impl AsContextMut<WasmerBackend>,
        func: wasmer::Function,
    ) -> Self {
        let ty = func.ty(&mut ctx.as_context_mut().inner);
        let sig = function_type_to_func_sig(&ty);
        WasmerFunction {
            sig,
            inner: func,
            owner_memory: None,
        }
    }
}
macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: WasmerContextMut<'_>, func: F) -> WasmerFunction
            where F: Fn(WasmerCaller<'_>, $(replace_with!($args -> i32),)*) -> () + Send + Sync + 'static {
            let global_env = ctx.env.clone();
            let func = move |env: FunctionEnvMut<()>, $($args,)*| {
                let caller = WasmerCaller {inner: env, env: global_env.clone()};
                func(caller, $($args,)*)
            };

            let env = wasmer::FunctionEnv::new(&mut ctx.inner, ());
            let func = wasmer::Function::new_typed_with_env(&mut ctx.inner, &env, func);
            use WType::I32;
            let params = vec![$(replace_with!($args -> I32),)*];
            let rets = vec![];
            let sig = FuncSig::new(params, rets);

            WasmerFunction {
                sig,
                inner: func,
                owner_memory: None
            }
        }

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: WasmerContextMut<'_>, func: F) -> WasmerFunction
            where F: Fn(WasmerCaller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static {
            let global_env = ctx.env.clone();
            let func = move |env: FunctionEnvMut<()>, $($args,)*| {
                let caller = WasmerCaller {inner: env, env: global_env.clone()};
                func(caller, $($args,)*)
            };

            let env = wasmer::FunctionEnv::new(&mut ctx.inner, ());
            let func = wasmer::Function::new_typed_with_env(&mut ctx.inner, &env, func);
            use WType::I32;
            let params = vec![$(replace_with!($args -> I32),)*];
            let rets = vec![I32,];
            let sig = FuncSig::new(params, rets);

            WasmerFunction {
                sig,
                inner: func,
                owner_memory: None
            }
        }
    });
}

impl FuncConstructor<WasmerBackend> for WasmerFunction {
    fn new_typed_with_env_0_test<F>(mut ctx: WasmerContextMut<'_>, func: F) -> WasmerFunction
    where
        F: Fn(WasmerCaller<'_>) -> () + Send + Sync + 'static,
    {
        let global_env = ctx.env.clone();
        let func = move |env: FunctionEnvMut<()>| {
            let caller = WasmerCaller {
                inner: env,
                env: global_env.clone(),
            };
            func(caller)
        };
        let f2 = |env2: FunctionEnvMut<()>| {};

        let env = wasmer::FunctionEnv::new(&mut ctx.inner, ());
        let func = wasmer::Function::new_typed_with_env(&mut ctx.inner, &env, f2);
        use WType::I32;
        let params = vec![];
        let rets = vec![];
        let sig = FuncSig::new(params, rets);

        WasmerFunction {
            sig,
            inner: func,
            owner_memory: None,
        }
    }

    impl_for_each_function_signature!(impl_func_construction);
}
