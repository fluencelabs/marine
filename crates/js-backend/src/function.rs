use anyhow::anyhow;
use js_sys::Array;
use marine_wasm_backend_traits::impl_for_each_function_signature;
use marine_wasm_backend_traits::replace_with;

use marine_wasm_backend_traits::prelude::*;
use crate::{JsInstance, JsWasmBackend};
use crate::JsCaller;
use crate::JsContext;
use crate::JsContextMut;
use crate::js_conversions::{js_array_from_wval_array, js_from_wval, wval_from_js, wval_to_i32};

use wasm_bindgen::prelude::*;
use crate::instance::StoredInstance;
use crate::store::JsStoreInner;

#[derive(Clone)]
pub struct JsFunction {
    pub(crate) store_handle: usize,
    pub(crate) bound_instance: Option<JsInstance>,
}

pub(crate) struct JsFunctionStored {
    pub(crate) js_func: js_sys::Function,
    pub(crate) sig: FuncSig,
}

impl JsFunction {
    pub(crate) fn new_stored(
        ctx: &mut impl AsContextMut<JsWasmBackend>,
        func: js_sys::Function,
        sig: FuncSig,
    ) -> Self {
        let stored = JsFunctionStored { js_func: func, sig };

        let ctx = ctx.as_context_mut();
        let handle = ctx.inner.functions.len();
        ctx.inner.functions.push(stored);
        Self {
            store_handle: handle,
            bound_instance: <_>::default(),
        }
    }

    pub(crate) fn stored<'store>(&self, ctx: &JsContext<'store>) -> &'store JsFunctionStored {
        &ctx.inner.functions[self.store_handle]
    }

    pub(crate) fn stored_mut<'store>(
        &self,
        ctx: JsContextMut<'store>,
    ) -> &'store mut JsFunctionStored {
        &mut ctx.inner.functions[self.store_handle]
    }

    fn call_inner(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        // TODO make more efficient

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
                let value = wval_from_js(&result_types[0], &result);
                Ok(vec![value])
            }
            _n => {
                let result_array: Array = result.into();
                Ok(result_array
                    .iter()
                    .enumerate()
                    .map(|(i, js_val)| wval_from_js(&result_types[i], &js_val))
                    .collect::<Vec<_>>())
            }
        }
    }
}

// this is safe because its intended to run in single thread
unsafe impl Send for JsFunction {}
unsafe impl Sync for JsFunction {}

impl Function<JsWasmBackend> for JsFunction {
    fn new<F>(store: &mut impl AsContextMut<JsWasmBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static,
    {
        let enclosed_sig = sig.clone();
        let wrapped = move |args: &js_sys::Array| -> js_sys::Array {
            let args = enclosed_sig
                .params()
                .iter()
                .enumerate()
                .map(|(index, ty)| wval_from_js(ty, &args.get(index as u32)))
                .collect::<Vec<_>>();

            let result = func(&args);
            js_array_from_wval_array(&result)
        };

        let inner = Closure::wrap(Box::new(wrapped) as Box<dyn FnMut(&Array) -> Array>)
            .into_js_value()
            .unchecked_into::<js_sys::Function>();

        JsFunction::new_stored(store, inner, sig)
    }

    fn new_with_caller<F>(
        store: &mut impl AsContextMut<JsWasmBackend>,
        sig: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(<JsWasmBackend as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static,
    {
        // Safety: JsStoreInner is stored inside a Box and the Store is required by wasm-backend traits contract
        // to be valid for function execution. So it is safe to capture this ptr into closure and deferenece there.
        let store_inner_ptr = store.as_context_mut().inner as *mut JsStoreInner;
        let enclosed_sig = sig.clone();
        let wrapped = move |args: &js_sys::Array| -> js_sys::Array {
            log::debug!(
                "function produced by JsFunction:::new_with_caller call, signature: {:?}",
                enclosed_sig
            );

            let store_inner = unsafe { &mut *store_inner_ptr };
            let caller_instance = store_inner.wasm_call_stack.last().map(Clone::clone);
            let caller = JsCaller {
                store_inner,
                caller_instance,
                _data: Default::default(),
            };

            let args = enclosed_sig
                .params()
                .iter()
                .enumerate()
                .map(|(index, ty)| wval_from_js(ty, &args.get(index as u32)))
                .collect::<Vec<_>>();

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

        JsFunction::new_stored(store, bound_func, sig)
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

    fn call(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        if let Some(instance) = &self.bound_instance {
            store
                .as_context_mut()
                .inner
                .wasm_call_stack
                .push(instance.clone())
        }

        let result = self.call_inner(store, args);

        if let Some(instance) = &self.bound_instance {
            store.as_context_mut().inner.wasm_call_stack.pop();
        }

        result
    }
}

/// Generates a function that accepts a Fn with $num template parameters and turns it into WasmtimeFunction.
/// Needed to allow users to pass almost any function to `Function::new_typed` without worrying about signature.
macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: JsContextMut<'_>, func: F) -> JsFunction
            where F: Fn(JsCaller<'_>, $(replace_with!($args -> i32),)*) + Send + Sync + 'static {

            let func = move |caller: JsCaller<'_>, args: &[WValue]| -> Vec<WValue> {
                let [$($args,)*] = args else { todo!() };;
                func(caller, $(wval_to_i32($args),)*);
                vec![]
            };


            let arg_ty = vec![WType::I32; $num];
            let ret_ty = vec![];
            let sig = FuncSig::new(arg_ty, ret_ty);

            JsFunction::new_with_caller(&mut ctx, sig, func)
        }

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: JsContextMut<'_>, func: F) -> JsFunction
            where F: Fn(JsCaller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static {

            let func = move |caller: JsCaller<'_>, args: &[WValue]| -> Vec<WValue> {
                let [$($args,)*] = args else { todo!() };;
                let res = func(caller, $(wval_to_i32(&$args),)*);
                vec![WValue::I32(res)]
            };

            let arg_ty = vec![WType::I32; $num];
            let ret_ty = vec![WType::I32];
            let sig = FuncSig::new(arg_ty, ret_ty);

            JsFunction::new_with_caller(&mut ctx, sig, func)
        }
    });
}

impl FuncConstructor<JsWasmBackend> for JsFunction {
    impl_for_each_function_signature!(impl_func_construction);
}
