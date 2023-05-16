use std::marker::PhantomData;
use marine_wasm_backend_traits::prelude::*;

use crate::{JsInstance, JsWasmBackend};
use crate::JsContext;
use crate::JsContextMut;

pub struct JsCaller<'c> {
    pub(crate) store_inner: *mut crate::store::JsStoreInner,
    pub(crate) caller_instance: Option<JsInstance>,
    pub(crate) _data: PhantomData<&'c i32>,
}

impl<'c> Caller<JsWasmBackend> for JsCaller<'c> {
    fn memory(&mut self, memory_index: u32) -> Option<<JsWasmBackend as WasmBackend>::Memory> {
        self.caller_instance
            .clone()
            .and_then(|instance| instance.get_nth_memory(&mut self.as_context_mut(), 0))
    }
}

impl<'c> AsContext<JsWasmBackend> for JsCaller<'c> {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::from_raw_ptr(self.store_inner)
    }
}

impl<'c> AsContextMut<JsWasmBackend> for JsCaller<'c> {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        JsContextMut::from_raw_ptr(self.store_inner)
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
                    dyn FnMut(&mut JsContextMut<'_>, $args) -> Result<$rets, RuntimeError>
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
