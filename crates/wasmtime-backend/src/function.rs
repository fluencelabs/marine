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

use crate::WasmtimeWasmBackend;
use crate::WasmtimeImportCallContext;
use crate::WasmtimeContextMut;
use crate::val_to_wvalue;
use crate::StoreState;
use crate::sig_to_fn_ty;
use crate::wvalue_to_val;
use crate::utils::fn_ty_to_sig;
use crate::utils::inspect_call_error;

use marine_wasm_backend_traits::prelude::*;
use marine_wasm_backend_traits::impl_for_each_function_signature;
use marine_wasm_backend_traits::replace_with;

use anyhow::anyhow;
use futures::future::BoxFuture;
use futures::FutureExt;

use std::future::Future;

#[derive(Clone)]
pub struct WasmtimeFunction {
    pub(crate) inner: wasmtime::Func,
}

impl HostFunction<WasmtimeWasmBackend> for WasmtimeFunction {
    fn new<F>(store: &mut impl AsContextMut<WasmtimeWasmBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> anyhow::Result<Vec<WValue>> + Sync + Send + 'static,
    {
        let ty = sig_to_fn_ty(&sig);
        let func = move |_: wasmtime::Caller<'_, StoreState>,
                         args: &[wasmtime::Val],
                         results_out: &mut [wasmtime::Val]|
              -> Result<(), anyhow::Error> {
            let args = process_func_args(args).map_err(|e| anyhow!(e))?; // TODO move earlier
            let results = func(&args)?;
            process_func_results(&results, results_out).map_err(|e| anyhow!(e))
        };

        let func = wasmtime::Func::new(store.as_context_mut().inner, ty, func);
        WasmtimeFunction { inner: func }
    }

    fn new_with_caller<F>(
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        sig: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(
                <WasmtimeWasmBackend as WasmBackend>::ImportCallContext<'c>,
                &[WValue],
            ) -> anyhow::Result<Vec<WValue>>
            + Sync
            + Send
            + 'static,
    {
        let ty = sig_to_fn_ty(&sig);

        let func = move |caller: wasmtime::Caller<'_, StoreState>,
                         args: &[wasmtime::Val],
                         results_out: &mut [wasmtime::Val]|
              -> Result<(), anyhow::Error> {
            let caller = WasmtimeImportCallContext { inner: caller };
            let args = process_func_args(args).map_err(|e| anyhow!(e))?;
            let results = func(caller, &args)?;
            process_func_results(&results, results_out).map_err(|e| anyhow!(e))
        };

        let func = wasmtime::Func::new(store.as_context_mut().inner, ty, func);
        WasmtimeFunction { inner: func }
    }

    fn new_with_caller_async<F>(
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        sig: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(
                <WasmtimeWasmBackend as WasmBackend>::ImportCallContext<'c>,
                &'c [WValue],
            ) -> BoxFuture<'c, anyhow::Result<Vec<WValue>>>
            + Sync
            + Send
            + 'static,
    {
        let ty = sig_to_fn_ty(&sig);
        let user_func = std::sync::Arc::new(func);
        let func = lifetimify_wrapped_closure(
            move |caller: wasmtime::Caller<StoreState>,
                  args: &[wasmtime::Val],
                  results_out: &mut [wasmtime::Val]|
                  -> Box<dyn Future<Output = Result<(), anyhow::Error>> + Send> {
                let func = user_func.clone();
                Box::new(async move {
                    let caller = WasmtimeImportCallContext { inner: caller };
                    let args = process_func_args(args).map_err(|e| anyhow!(e))?;
                    let results = func(caller, &args).await?;
                    process_func_results(&results, results_out).map_err(|e| anyhow!(e))
                })
            },
        );

        let func = wasmtime::Func::new_async(store.as_context_mut().inner, ty, func);
        WasmtimeFunction { inner: func }
    }

    fn new_async<F>(
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        sig: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(&'c [WValue]) -> BoxFuture<'c, anyhow::Result<Vec<WValue>>>
            + Sync
            + Send
            + 'static,
    {
        Self::new_with_caller_async(store, sig, move |_caller, args| func(args))
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
}

impl ExportFunction<WasmtimeWasmBackend> for WasmtimeFunction {
    fn signature<'c>(&self, store: &mut impl AsContextMut<WasmtimeWasmBackend>) -> FuncSig {
        let ty = self.inner.ty(store.as_context_mut());
        fn_ty_to_sig(&ty)
    }

    fn call_async<'args>(
        &'args self,
        store: &'args mut impl AsContextMut<WasmtimeWasmBackend>,
        args: &'args [WValue],
    ) -> BoxFuture<'args, RuntimeResult<Vec<WValue>>> {
        let args = args.iter().map(wvalue_to_val).collect::<Vec<_>>();

        let results_count = self.inner.ty(store.as_context_mut()).results().len();
        let mut results = vec![wasmtime::Val::null(); results_count];
        let func = self.inner;
        async move {
            func.call_async(store.as_context_mut().inner, &args, &mut results)
                .await
                .map_err(inspect_call_error)?;

            results
                .iter()
                .map(val_to_wvalue)
                .collect::<Result<Vec<_>, _>>()
        }
        .boxed()
    }
}

/// Generates a function that accepts a Fn with $num template parameters and turns it into WasmtimeFunction.
/// Needed to allow users to pass almost any function to `Function::new_typed` without worrying about signature.
macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: WasmtimeContextMut<'_>, func: F) -> WasmtimeFunction
            where F: Fn(WasmtimeImportCallContext<'_>, $(replace_with!($args -> i32),)*) + Send + Sync + 'static {

            let func = move |caller: wasmtime::Caller<'_, StoreState>, $($args,)*| {
                let caller = WasmtimeImportCallContext {inner: caller};
                func(caller, $($args,)*)
            };

            let func = wasmtime::Func::wrap(&mut ctx.inner, func);

            WasmtimeFunction {
                inner: func
            }
        }

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: WasmtimeContextMut<'_>, func: F) -> WasmtimeFunction
            where F: Fn(WasmtimeImportCallContext<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static {

            let func = move |caller: wasmtime::Caller<'_, StoreState>, $($args,)*| -> i32{
                let caller = WasmtimeImportCallContext {inner: caller};
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

fn process_func_args(args: &[wasmtime::Val]) -> RuntimeResult<Vec<WValue>> {
    args.iter()
        .map(val_to_wvalue)
        .collect::<RuntimeResult<Vec<_>>>()
}

fn process_func_results(
    results_in: &[WValue],
    results_out: &mut [wasmtime::Val],
) -> RuntimeResult<()> {
    if results_in.len() != results_out.len() {
        return Err(RuntimeError::IncorrectResultsNumber {
            expected: results_out.len(),
            actual: results_in.len(),
        });
    }

    for id in 0..results_in.len() {
        results_out[id] = wvalue_to_val(&results_in[id]);
    }

    Ok(())
}

fn lifetimify_wrapped_closure<F>(func: F) -> F
where
    for<'c> F: Fn(
            wasmtime::Caller<'c, StoreState>,
            &'c [wasmtime::Val],
            &'c mut [wasmtime::Val],
        ) -> Box<dyn Future<Output = Result<(), anyhow::Error>> + Send + 'c>
        + Send
        + Sync
        + 'static,
{
    func
}
