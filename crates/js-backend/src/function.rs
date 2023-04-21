use anyhow::anyhow;
use js_sys::Array;
use marine_wasm_backend_traits::impl_for_each_function_signature;
use marine_wasm_backend_traits::replace_with;

use marine_wasm_backend_traits::prelude::*;
use crate::JsWasmBackend;
use crate::JsCaller;
use crate::JsContext;
use crate::js_conversions::{js_array_from_wval_array, js_from_wval, wval_from_js, wval_to_i32};

use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct JsFunction {
    pub(crate) inner: js_sys::Function,
    pub(crate) sig: FuncSig,
}

// this is safe because its intended to run in single thread
unsafe impl Send for JsFunction {}
unsafe impl Sync for JsFunction {}

impl Function<JsWasmBackend> for JsFunction {
    fn new<F>(_store: &mut impl AsContextMut<JsWasmBackend>, sig: FuncSig, func: F) -> Self
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

        Self {
            inner,
            sig,
        }
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
        let enclosed_sig = sig.clone();
        let wrapped = move |args: &js_sys::Array| -> js_sys::Array {
            let caller = JsCaller { _data: Default::default() };
            let args = enclosed_sig
                .params()
                .iter()
                .enumerate()
                .map(|(index, ty)| wval_from_js(ty, &args.get(index as u32)))
                .collect::<Vec<_>>();
            let result = func(caller, &args);
            js_array_from_wval_array(&result)
        };

        let func = Closure::wrap(Box::new(wrapped) as Box<dyn FnMut(&Array) -> Array>)
            .into_js_value();

        Self::from_js(sig, func)
    }

    fn new_typed<Params, Results, Env>(
        store: &mut impl AsContextMut<JsWasmBackend>,
        func: impl IntoFunc<JsWasmBackend, Params, Results, Env>,
    ) -> Self {
        func.into_func(store)
    }

    fn signature(&self, store: &mut impl AsContextMut<JsWasmBackend>) -> FuncSig {
        self.sig.clone()
    }

    fn call(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        // TODO make more efficient
        let params = js_array_from_wval_array(args);
        let result = js_sys::Reflect::apply(
            &self.inner,
            &JsValue::NULL,
            &params
        ).map_err(|e| {
            web_sys::console::log_2(&"failed to apply func".into(), &e);
            RuntimeError::Other(anyhow!("Failed to apply func"))
        })?;

        let result_types = self.sig.returns();
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

impl JsFunction {
    pub(crate) fn from_js(sig: FuncSig, func: JsValue) -> Self {
        Self {
            inner: func.unchecked_into(),
            sig
        }
    }
}

/// Generates a function that accepts a Fn with $num template parameters and turns it into WasmtimeFunction.
/// Needed to allow users to pass almost any function to `Function::new_typed` without worrying about signature.
macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: JsContext<'_>, func: F) -> JsFunction
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

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: JsContext<'_>, func: F) -> JsFunction
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
