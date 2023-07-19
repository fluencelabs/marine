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

use marine_wasm_backend_traits::impl_for_each_function_signature;
use marine_wasm_backend_traits::replace_with;
use marine_wasm_backend_traits::prelude::*;

use anyhow::anyhow;
use js_sys::Array;
use wasm_bindgen::prelude::*;

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
    pub(crate) js_func: js_sys::Function,
    pub(crate) sig: FuncSig,
}

impl StoredFunction {
    pub(crate) fn new(js_func: js_sys::Function, sig: FuncSig) -> Self {
        Self { js_func, sig }
    }
}

impl WasmExportFunction {
    pub(crate) fn new_stored(
        ctx: &mut impl AsContextMut<JsWasmBackend>,
        instance: JsInstance,
        func: js_sys::Function,
        sig: FuncSig,
    ) -> Self {
        let handle = ctx
            .as_context_mut()
            .inner
            .store_function(StoredFunction::new(func, sig));

        Self {
            store_handle: handle,
            bound_instance: instance,
        }
    }

    pub(crate) fn stored<'store>(&self, ctx: &JsContext<'store>) -> &'store StoredFunction {
        &ctx.inner.functions[self.store_handle]
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
        let result = js_sys::Reflect::apply(&stored_func.js_func, &JsValue::NULL, &params)
            .map_err(|e| {
                web_sys::console::log_2(&"failed to apply func".into(), &e);
                RuntimeError::Other(anyhow!("Failed to apply func"))
            })?;

        let result_types = stored_func.sig.returns();
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

// Safety: this is safe because its intended to run in single thread
unsafe impl Send for HostImportFunction {}
unsafe impl Sync for HostImportFunction {}
unsafe impl Send for WasmExportFunction {}
unsafe impl Sync for WasmExportFunction {}

impl HostFunction<JsWasmBackend> for HostImportFunction {
    fn new<F>(store: &mut impl AsContextMut<JsWasmBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&'c [WValue]) -> Vec<WValue> + Sync + Send + 'static,
    {
        let with_caller = move |_, args: &'_ [WValue]| func(args);
        Self::new_with_caller(store, sig, with_caller)
    }

    fn new_with_caller<F>(
        store: &mut impl AsContextMut<JsWasmBackend>,
        sig: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(JsImportCallContext, &[WValue]) -> Vec<WValue> + Sync + Send + 'static,
    {
        // Safety: JsStoreInner is stored inside a Box and the Store is required by wasm-backend traits contract
        // to be valid for function execution. So it is safe to capture this ptr into closure and deference there.
        let store_inner_ptr = store.as_context_mut().inner as *mut JsStoreInner;
        let enclosed_sig = sig.clone();
        let wrapped = move |args: &js_sys::Array| -> js_sys::Array {
            log::debug!(
                "function produced by JsFunction:::new_with_caller call, signature: {:?}",
                enclosed_sig
            );

            let store_inner = unsafe { &mut *store_inner_ptr };
            let caller_instance = store_inner.wasm_call_stack.last().cloned().expect(
                "Import cannot be called outside of an export call, when wasm_call_stack is empty",
            );

            let caller = JsImportCallContext {
                store_inner,
                caller_instance,
            };

            let args = wval_array_from_js_array(args, enclosed_sig.params().iter());
            let result = func(caller, &args);
            js_array_from_wval_array(&result)
        };

        let func =
            Closure::wrap(Box::new(wrapped) as Box<dyn FnMut(&Array) -> Array>).into_js_value();

        // Make a function that converts function args into array and wrap our func with it.
        // Otherwise our closure will get only first argument.
        let dyn_func = js_sys::Function::new_with_args(
            "wrapped_func",
            "return wrapped_func(Array.prototype.slice.call(arguments, 1))",
        );
        let bound_func = dyn_func.bind1(&JsValue::UNDEFINED, &func);

        let handle = store
            .as_context_mut()
            .inner
            .store_function(StoredFunction::new(bound_func, sig));

        Self {
            store_handle: handle,
        }
    }

    fn new_typed<Params, Results, Env>(
        store: &mut impl AsContextMut<JsWasmBackend>,
        func: impl IntoFunc<JsWasmBackend, Params, Results, Env>,
    ) -> Self {
        func.into_func(store)
    }

    fn signature(&self, store: &mut impl AsContextMut<JsWasmBackend>) -> FuncSig {
        self.stored_mut(store.as_context_mut()).sig.clone()
    }
}

impl ExportFunction<JsWasmBackend> for WasmExportFunction {
    fn signature(&self, store: &mut impl AsContextMut<JsWasmBackend>) -> FuncSig {
        self.stored_mut(store.as_context_mut()).sig.clone()
    }

    fn call(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        store
            .as_context_mut()
            .inner
            .wasm_call_stack
            .push(self.bound_instance.clone());

        let result = self.call_inner(store, args);

        store.as_context_mut().inner.wasm_call_stack.pop();

        result
    }
}

/// Generates a function that accepts a Fn with $num template parameters and turns it into WasmtimeFunction.
/// Needed to allow users to pass almost any function to `Function::new_typed` without worrying about signature.
macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: JsContextMut<'_>, func: F) -> HostImportFunction
            where F: Fn(JsImportCallContext, $(replace_with!($args -> i32),)*) + Send + Sync + 'static {

            let func = move |caller: JsImportCallContext, args: &[WValue]| -> Vec<WValue> {
                let [$($args,)*] = args else { todo!() }; // TODO: Safety: explain why it will never fire
                func(caller, $(wval_to_i32($args),)*);
                vec![]
            };

            let arg_ty = vec![WType::I32; $num];
            let ret_ty = vec![];
            let sig = FuncSig::new(arg_ty, ret_ty);

            HostImportFunction::new_with_caller(&mut ctx, sig, func)
        }

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: JsContextMut<'_>, func: F) -> HostImportFunction
            where F: Fn(JsImportCallContext, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static {

            let func = move |caller: JsImportCallContext, args: &[WValue]| -> Vec<WValue> {
                let [$($args,)*] = args else { panic!("args do not match signature") }; // Safety: signature should b
                let res = func(caller, $(wval_to_i32(&$args),)*);
                vec![WValue::I32(res)]
            };

            let arg_ty = vec![WType::I32; $num];
            let ret_ty = vec![WType::I32];
            let sig = FuncSig::new(arg_ty, ret_ty);

            HostImportFunction::new_with_caller(&mut ctx, sig, func)
        }
    });
}

impl FuncConstructor<JsWasmBackend> for HostImportFunction {
    impl_for_each_function_signature!(impl_func_construction);
}
