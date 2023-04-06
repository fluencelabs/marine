use marine_wasm_backend_traits::impl_for_each_function_signature;
use marine_wasm_backend_traits::replace_with;

use marine_wasm_backend_traits::prelude::*;
use crate::JsWasmBackend;
use crate::JsCaller;
use crate::JsContext;

#[derive(Clone)]
pub struct JsFunction {}

impl Function<JsWasmBackend> for JsFunction {
    fn new<F>(store: &mut impl AsContextMut<JsWasmBackend>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static,
    {
        todo!()
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
        todo!()
    }

    fn new_typed<Params, Results, Env>(
        store: &mut impl AsContextMut<JsWasmBackend>,
        func: impl IntoFunc<JsWasmBackend, Params, Results, Env>,
    ) -> Self {
        todo!()
    }

    fn signature(&self, store: &mut impl AsContextMut<JsWasmBackend>) -> FuncSig {
        todo!()
    }

    fn call(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        args: &[WValue],
    ) -> RuntimeResult<Vec<WValue>> {
        todo!()
    }
}

/// Generates a function that accepts a Fn with $num template parameters and turns it into WasmtimeFunction.
/// Needed to allow users to pass almost any function to `Function::new_typed` without worrying about signature.
macro_rules! impl_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_with_env_ $num >] <F>(mut ctx: JsContext<'_>, func: F) -> JsFunction
            where F: Fn(JsCaller<'_>, $(replace_with!($args -> i32),)*) + Send + Sync + 'static {

            todo!()
        }

        fn [< new_typed_with_env_ $num _r>] <F>(mut ctx: JsContext<'_>, func: F) -> JsFunction
            where F: Fn(JsCaller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static {

            todo!()
        }
    });
}

impl FuncConstructor<JsWasmBackend> for JsFunction {
    impl_for_each_function_signature!(impl_func_construction);
}
