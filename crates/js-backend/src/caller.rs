use std::marker::PhantomData;
use marine_wasm_backend_traits::prelude::*;

use crate::JsWasmBackend;
use crate::JsContext;

pub struct JsCaller<'c> {
    _data: PhantomData<&'c i32>,
}

impl<'c> Caller<JsWasmBackend> for JsCaller<'c> {
    fn memory(&mut self, memory_index: u32) -> Option<<JsWasmBackend as WasmBackend>::Memory> {
        todo!()
    }
}

impl<'c> AsContext<JsWasmBackend> for JsCaller<'c> {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        todo!()
    }
}

impl<'c> AsContextMut<JsWasmBackend> for JsCaller<'c> {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        todo!()
    }
}

/// Implements func_getter for given function signature.
/// Later `get_func` variant will be statically chosen based on types.
macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl<'c> FuncGetter<JsWasmBackend, $args, $rets> for JsCaller<'c> {
            fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(&mut JsContext<'_>, $args) -> Result<$rets, RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                todo!()
            }
        }
    };
}

// These signatures are sufficient for marine to work.
impl_func_getter!((i32, i32), i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!(i32, i32);
impl_func_getter!(i32, ());
impl_func_getter!((), i32);
impl_func_getter!((), ());
