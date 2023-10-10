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

use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use crate::WasmtimeContextMut;
use crate::WasmtimeWasmBackend;
use crate::WasmtimeImportCallContext;
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
use async_trait::async_trait;

//use std::future::Future;
#[derive(Clone)]
pub struct WasmtimeFunction {
    pub(crate) inner: wasmtime::Func,
}

impl HostFunction<WasmtimeWasmBackend> for WasmtimeFunction {
    fn new<F>(store: &mut impl AsContextMut<WasmtimeWasmBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static,
    {
        let ty = sig_to_fn_ty(&sig);
        let func = move |_: wasmtime::Caller<'_, StoreState>,
                         args: &[wasmtime::Val],
                         results_out: &mut [wasmtime::Val]|
              -> Result<(), anyhow::Error> {
            let args = process_func_args(args).map_err(|e| anyhow!(e))?; // TODO move earlier
            let results = func(&args);
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
            ) -> Vec<WValue>
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
            let results = func(caller, &args);
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
            ) -> Box<dyn Future<Output = Vec<WValue>> + Send + 'c>
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
                    let results = std::pin::Pin::from(func(caller, &args)).await;
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
        F: for<'c> Fn(&'c [WValue]) -> Box<dyn Future<Output = Vec<WValue>> + Send + 'c>
            + Sync
            + Send
            + 'static,
    {
        todo!()
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

#[async_trait]
impl ExportFunction<WasmtimeWasmBackend> for WasmtimeFunction {
    fn signature<'c>(&self, store: &mut impl AsContextMut<WasmtimeWasmBackend>) -> FuncSig {
        let ty = self.inner.ty(store.as_context_mut());
        fn_ty_to_sig(&ty)
    }

    async fn call(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        let args = args.iter().map(wvalue_to_val).collect::<Vec<_>>();

        let results_count = self.inner.ty(store.as_context_mut()).results().len();
        let mut results = vec![wasmtime::Val::null(); results_count];

        self.inner
            .call(store.as_context_mut().inner, &args, &mut results)
            .map_err(inspect_call_error)?;

        results
            .iter()
            .map(val_to_wvalue)
            .collect::<Result<Vec<_>, _>>()
    }
}

#[async_trait]
impl AsyncFunction<WasmtimeWasmBackend> for WasmtimeFunction {
    async fn call_async<CTX>(&self, store: &mut CTX, args: &[WValue]) -> RuntimeResult<Vec<WValue>>
    where
        CTX: AsContextMut<WasmtimeWasmBackend> + Send,
    {
        let mut context = store.as_context_mut().inner;

        let args = args.iter().map(wvalue_to_val).collect::<Vec<_>>();
        let results_count = self.inner.ty(&mut context).results().len();
        let mut results = vec![wasmtime::Val::null(); results_count];

        self.inner
            .call_async(&mut context, &args, &mut results)
            .await
            .map_err(inspect_call_error)?;

        results
            .iter()
            .map(val_to_wvalue)
            .collect::<Result<Vec<_>, _>>()
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

/*
struct FuncCallFuture<'f, F> {
    caller: wasmtime::Caller<'f, StoreState>,
    args: Vec<WValue>,
    func: &'f F,
}

impl<F> std::future::Future for FuncCallFuture<F> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}*/
