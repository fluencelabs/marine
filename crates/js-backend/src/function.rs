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

use crate::JsInstance;
use crate::JsWasmBackend;
use crate::JsImportCallContext;
use crate::JsContext;
use crate::JsContextMut;
use crate::js_conversions::js_array_from_wval_array;
use crate::js_conversions::wval_array_from_js_array;
use crate::js_conversions::wval_from_js;
use crate::js_conversions::wval_to_i32;
use crate::store::JsStoreInner;
use crate::store::FunctionHandle;
use crate::single_shot_async_executor::execute_future_blocking;

use marine_wasm_backend_traits::impl_for_each_function_signature;
use marine_wasm_backend_traits::replace_with;
use marine_wasm_backend_traits::prelude::*;

use anyhow::anyhow;
use futures::future::BoxFuture;
use futures::FutureExt;
use js_sys::Array;
use wasm_bindgen::prelude::*;

/// Safety: js-backend is expected to run in single-threaded environment,
/// so it is safe to assume that every type is Send + Sync
unsafe impl Send for HostImportFunction {}
unsafe impl Sync for HostImportFunction {}
unsafe impl Send for WasmExportFunction {}
unsafe impl Sync for WasmExportFunction {}

#[derive(Clone)]
pub struct HostImportFunction {
    pub(crate) store_handle: FunctionHandle,
}

#[derive(Clone)]
pub struct WasmExportFunction {
    pub(crate) store_handle: FunctionHandle,

    /// Instance this export is extracted from.
    pub(crate) bound_instance: JsInstance,
}

pub(crate) struct StoredFunction {
    pub(crate) js_function: js_sys::Function,
    pub(crate) signature: FuncSig,
}

impl StoredFunction {
    pub(crate) fn new(js_function: js_sys::Function, signature: FuncSig) -> Self {
        Self {
            js_function,
            signature,
        }
    }
}

impl WasmExportFunction {
    pub(crate) fn new_stored(
        ctx: &mut impl AsContextMut<JsWasmBackend>,
        instance: JsInstance,
        func: js_sys::Function,
        signature: FuncSig,
    ) -> Self {
        let handle = ctx
            .as_context_mut()
            .inner
            .store_function(StoredFunction::new(func, signature));

        Self {
            store_handle: handle,
            bound_instance: instance,
        }
    }

    pub(crate) fn stored_mut<'store>(
        &self,
        ctx: JsContextMut<'store>,
    ) -> &'store mut StoredFunction {
        &mut ctx.inner.functions[self.store_handle]
    }

    fn call_inner(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        let params = js_array_from_wval_array(args);
        let stored_func = self.stored_mut(store.as_context_mut());
        let result = js_sys::Reflect::apply(&stored_func.js_function, &JsValue::NULL, &params)
            .map_err(|e| {
                web_sys::console::log_2(&"failed to apply func".into(), &e);
                RuntimeError::Other(anyhow!("Failed to apply func"))
            })?;

        extract_function_results(result, stored_func.signature.returns())
    }
}

fn extract_function_results(result: JsValue, result_types: &[WType]) -> RuntimeResult<Vec<WValue>> {
    match result_types.len() {
        0 => Ok(vec![]),
        1 => {
            // Single value returned as is.
            let value = wval_from_js(&result_types[0], &result);
            Ok(vec![value])
        }
        results_number => {
            // Multiple return values are returned as JS array of values.
            let result_array: Array = result.into();
            if result_array.length() as usize != results_number {
                Err(RuntimeError::IncorrectResultsNumber {
                    expected: results_number,
                    actual: result_array.length() as usize,
                })
            } else {
                Ok(wval_array_from_js_array(&result_array, result_types.iter()))
            }
        }
    }
}

impl HostImportFunction {
    pub(crate) fn stored<'store>(&self, ctx: &JsContext<'store>) -> &'store StoredFunction {
        &ctx.inner.functions[self.store_handle]
    }

    pub(crate) fn stored_mut<'store>(
        &self,
        ctx: JsContextMut<'store>,
    ) -> &'store mut StoredFunction {
        &mut ctx.inner.functions[self.store_handle]
    }
}

impl HostFunction<JsWasmBackend> for HostImportFunction {
    fn new<F>(store: &mut impl AsContextMut<JsWasmBackend>, signature: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&'c [WValue]) -> anyhow::Result<Vec<WValue>> + Sync + Send + 'static,
    {
        let with_caller = move |_, args: &'_ [WValue]| func(args);
        Self::new_with_caller(store, signature, with_caller)
    }

    fn new_with_caller<F>(
        store: &mut impl AsContextMut<JsWasmBackend>,
        signature: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(JsImportCallContext, &[WValue]) -> anyhow::Result<Vec<WValue>>
            + Sync
            + Send
            + 'static,
    {
        // Safety: JsStoreInner is stored inside a Box and the Store is required by wasm-backend traits contract
        // to be valid for function execution. So it is safe to capture this ptr into closure and deference there
        let store_inner_ptr = store.as_context_mut().inner as *mut JsStoreInner;

        let wrapped = wrap_raw_host_fn(signature.clone(), store_inner_ptr, func);
        let closure = prepare_js_closure(wrapped);

        let handle = store
            .as_context_mut()
            .inner
            .store_function(StoredFunction::new(closure, signature));

        Self {
            store_handle: handle,
        }
    }

    fn new_with_caller_async<F>(
        store: &mut impl AsContextMut<JsWasmBackend>,
        signature: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(
                <JsWasmBackend as WasmBackend>::ImportCallContext<'c>,
                &'c [WValue],
            ) -> BoxFuture<'c, anyhow::Result<Vec<WValue>>>
            + Sync
            + Send
            + 'static,
    {
        // Safety: JsStoreInner is stored inside a Box and the Store is required by wasm-backend traits contract
        // to be valid for function execution. So it is safe to capture this ptr into closure and deference there
        let store_inner_ptr = store.as_context_mut().inner as *mut JsStoreInner;

        let wrapped = wrap_raw_host_fn_async(signature.clone(), store_inner_ptr, func);
        let closure = prepare_js_closure(wrapped);

        let handle = store
            .as_context_mut()
            .inner
            .store_function(StoredFunction::new(closure, signature));

        Self {
            store_handle: handle,
        }
    }

    fn new_async<F>(store: &mut impl AsContextMut<JsWasmBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&'c [WValue]) -> BoxFuture<'c, anyhow::Result<Vec<WValue>>>
            + Sync
            + Send
            + 'static,
    {
        Self::new_with_caller_async(store, sig, move |_caller, args| func(args))
    }

    fn new_typed<Params, Results, Env>(
        store: &mut impl AsContextMut<JsWasmBackend>,
        func: impl IntoFunc<JsWasmBackend, Params, Results, Env>,
    ) -> Self {
        func.into_func(store)
    }

    fn signature(&self, store: &mut impl AsContextMut<JsWasmBackend>) -> FuncSig {
        self.stored_mut(store.as_context_mut()).signature.clone()
    }
}

fn wrap_raw_host_fn<F>(
    signature: FuncSig,
    store_inner_ptr: *mut JsStoreInner,
    raw_host_function: F,
) -> Box<dyn FnMut(&Array) -> Array>
where
    F: for<'c> Fn(JsImportCallContext, &[WValue]) -> anyhow::Result<Vec<WValue>>
        + Sync
        + Send
        + 'static,
{
    let func = move |args: &js_sys::Array| -> js_sys::Array {
        log::debug!(
            "function produced by JsFunction:::new_with_caller call, signature: {:?}",
            signature
        );

        let store_inner = unsafe { &mut *store_inner_ptr };
        let caller_instance = store_inner.wasm_call_stack.last().cloned().expect(
            "Import cannot be called outside of an export call, when wasm_call_stack is empty",
        );

        let caller = JsImportCallContext {
            store_inner,
            caller_instance,
        };

        let args = wval_array_from_js_array(args, signature.params().iter());
        let result = raw_host_function(caller, &args).unwrap_throw(); // TODO is it right?

        js_array_from_wval_array(&result)
    };

    Box::new(func)
}

fn wrap_raw_host_fn_async<F>(
    signature: FuncSig,
    store_inner_ptr: *mut JsStoreInner,
    raw_host_function: F,
) -> Box<dyn FnMut(&Array) -> Array>
where
    F: for<'c> Fn(JsImportCallContext, &[WValue]) -> BoxFuture<'_, anyhow::Result<Vec<WValue>>>
        + Sync
        + Send
        + 'static,
{
    let func = move |args: &js_sys::Array| -> js_sys::Array {
        log::debug!(
            "function produced by JsFunction:::new_with_caller call, signature: {:?}",
            signature
        );

        let store_inner = unsafe { &mut *store_inner_ptr };
        let caller_instance = store_inner.wasm_call_stack.last().cloned().expect(
            "Import cannot be called outside of an export call, when wasm_call_stack is empty",
        );

        let caller = JsImportCallContext {
            store_inner,
            caller_instance,
        };

        let args = wval_array_from_js_array(args, signature.params().iter());
        let result = execute_future_blocking(raw_host_function(caller, &args)).unwrap_throw();

        js_array_from_wval_array(&result)
    };

    Box::new(func)
}

fn prepare_js_closure(func: Box<dyn FnMut(&Array) -> Array>) -> js_sys::Function {
    let closure = Closure::wrap(func).into_js_value();

    // Make a function that converts function args into array and wrap our func with it.
    // Otherwise our closure will get only first argument.
    let wrapper = js_sys::Function::new_with_args(
        "wrapped_func",
        "return wrapped_func(Array.prototype.slice.call(arguments, 1))",
    );

    wrapper.bind1(&JsValue::UNDEFINED, &closure)
}

impl ExportFunction<JsWasmBackend> for WasmExportFunction {
    fn signature(&self, store: &mut impl AsContextMut<JsWasmBackend>) -> FuncSig {
        self.stored_mut(store.as_context_mut()).signature.clone()
    }

    fn call_async<'args>(
        &'args self,
        store: &'args mut impl AsContextMut<JsWasmBackend>,
        args: &'args [WValue],
    ) -> BoxFuture<'args, RuntimeResult<Vec<WValue>>> {
        async move {
            store
                .as_context_mut()
                .inner
                .wasm_call_stack
                .push(self.bound_instance.clone());

            let result = self.call_inner(store, args);

            store.as_context_mut().inner.wasm_call_stack.pop();

            result
        }
        .boxed()
    }
}

/// Generates a function that accepts a Fn with $num template parameters and turns it into WasmtimeFunction.
/// Needed to allow users to pass almost any function to `Function::new_typed` without worrying about signature.
macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: JsContextMut<'_>, func: F) -> HostImportFunction
            where F: Fn(JsImportCallContext, $(replace_with!($args -> i32),)*) + Send + Sync + 'static {

            let func = move |caller: JsImportCallContext, args: &[WValue]| -> anyhow::Result<Vec<WValue>> {
                let [$($args,)*] = args else { todo!() }; // TODO: Safety: explain why it will never fire
                func(caller, $(wval_to_i32($args),)*);
                Ok(vec![])
            };

            let arg_ty = vec![WType::I32; $num];
            let ret_ty = vec![];
            let signature = FuncSig::new(arg_ty, ret_ty);

            HostImportFunction::new_with_caller(&mut ctx, signature, func)
        }

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: JsContextMut<'_>, func: F) -> HostImportFunction
            where F: Fn(JsImportCallContext, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static {

            let func = move |caller: JsImportCallContext, args: &[WValue]| -> anyhow::Result<Vec<WValue>> {
                let [$($args,)*] = args else { panic!("args do not match signature") }; // Safety: signature should b
                let res = func(caller, $(wval_to_i32(&$args),)*);
                Ok(vec![WValue::I32(res)])
            };

            let arg_ty = vec![WType::I32; $num];
            let ret_ty = vec![WType::I32];
            let signature = FuncSig::new(arg_ty, ret_ty);

            HostImportFunction::new_with_caller(&mut ctx, signature, func)
        }
    });
}

impl FuncConstructor<JsWasmBackend> for HostImportFunction {
    impl_for_each_function_signature!(impl_func_construction);
}
