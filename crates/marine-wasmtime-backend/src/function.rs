/*
 * Copyright 2023 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::WasmtimeContextMut;
use crate::WasmtimeWasmBackend;
use crate::WasmtimeCaller;
use crate::val_to_wvalue;
use crate::StoreState;
use crate::sig_to_fn_ty;
use crate::wvalue_to_val;
use crate::utils::fn_ty_to_sig;
use crate::utils::inspect_call_error;

use marine_wasm_backend_traits::*;

use anyhow::anyhow;

pub struct WasmtimeFunction {
    pub(crate) inner: wasmtime::Func,
}

impl Function<WasmtimeWasmBackend> for WasmtimeFunction {
    fn new<F>(store: &mut impl AsContextMut<WasmtimeWasmBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static,
    {
        let ty = sig_to_fn_ty(&sig);
        let func = move |_: wasmtime::Caller<'_, StoreState>,
                         args: &[wasmtime::Val],
                         results: &mut [wasmtime::Val]| {
            let args = args
                .iter()
                .map(val_to_wvalue)
                .collect::<Result<Vec<_>, RuntimeError>>()
                .map_err(anyhow::Error::new)?; // todo move earlier

            let rets = func(&args);
            for i in 0..results.len() {
                results[i] = wvalue_to_val(&rets[i]);
            }

            Ok(())
        };

        let func = wasmtime::Func::new(store.as_context_mut().inner, ty, func);
        WasmtimeFunction { inner: func }
    }

    fn new_with_ctx<F>(
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        sig: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(<WasmtimeWasmBackend as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static,
    {
        let ty = sig_to_fn_ty(&sig);

        let func = move |caller: wasmtime::Caller<'_, StoreState>,
                         args: &[wasmtime::Val],
                         results: &mut [wasmtime::Val]| {
            let caller = WasmtimeCaller { inner: caller };
            let args = args
                .iter()
                .map(val_to_wvalue)
                .collect::<RuntimeResult<Vec<_>>>()
                .map_err(|e| anyhow!(e))?;
            let rets = func(caller, &args);
            for i in 0..results.len() {
                results[i] = wvalue_to_val(&rets[i]);
            }

            Ok(())
        };

        let func = wasmtime::Func::new(store.as_context_mut().inner, ty, func);
        WasmtimeFunction { inner: func }
    }

    fn new_typed<Params, Results, Env>(
        store: &mut impl marine_wasm_backend_traits::AsContextMut<WasmtimeWasmBackend>,
        func: impl IntoFunc<WasmtimeWasmBackend, Params, Results, Env>,
    ) -> Self {
        func.into_func(store)
    }

    fn signature<'c>(&self, store: &mut impl AsContextMut<WasmtimeWasmBackend>) -> FuncSig {
        let ty = self.inner.ty(store.as_context_mut());
        fn_ty_to_sig(&ty)
    }

    fn call<'c>(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        log::debug!("Function call with args: {:?}", args);
        let args = args.iter().map(wvalue_to_val).collect::<Vec<_>>();

        let mut rets = Vec::new();
        rets.resize(
            self.inner.ty(store.as_context_mut()).results().len(),
            wasmtime::Val::null(),
        );

        self.inner
            .call(store.as_context_mut().inner, &args, &mut rets)
            .map_err(|e| {
                log::debug!("Function call failed with: {:?}", &e);
                inspect_call_error(e)
            })?;

        log::debug!("Function call succeed");
        rets.iter()
            .map(val_to_wvalue)
            .collect::<Result<Vec<_>, _>>()
    }
}

macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: WasmtimeContextMut<'_>, func: F) -> WasmtimeFunction
            where F: Fn(WasmtimeCaller<'_>, $(replace_with!($args -> i32),)*) + Send + Sync + 'static {

            let func = move |caller: wasmtime::Caller<'_, StoreState>, $($args,)*| {
                let caller = WasmtimeCaller {inner: caller};
                func(caller, $($args,)*)
            };

            let func = wasmtime::Func::wrap(&mut ctx.inner, func);

            WasmtimeFunction {
                inner: func
            }
        }

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: WasmtimeContextMut<'_>, func: F) -> WasmtimeFunction
            where F: Fn(WasmtimeCaller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static {

            let func = move |caller: wasmtime::Caller<'_, StoreState>, $($args,)*| -> i32{
                let caller = WasmtimeCaller {inner: caller};
                func(caller, $($args,)*)
            };

            let func = wasmtime::Func::wrap(&mut ctx.inner, func);

            WasmtimeFunction {
                inner: func
            }
        }
    });
}

impl FuncConstructor<WasmtimeWasmBackend> for WasmtimeFunction {
    impl_for_each_function_signature!(impl_func_construction);
}
