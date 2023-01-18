use wasmer::{AsStoreRef, AsStoreMut, FunctionEnv, FunctionEnvMut};
use marine_wasm_backend_traits::*;
use crate::{StoreState, WasmerBackend, WasmerContext, WasmerContextMut};

pub struct WasmerCaller<'c> {
    pub(crate) inner: FunctionEnvMut<'c, ()>,
    pub(crate) env: FunctionEnv<StoreState>,
}

impl Caller<WasmerBackend> for WasmerCaller<'_> {
    fn memory(&mut self, memory_index: u32) -> <WasmerBackend as WasmBackend>::Memory {
        self.env
            .as_mut(&mut self.inner)
            .current_memory
            .clone()
            .unwrap()
            .into()
    }
}

impl AsContext<WasmerBackend> for WasmerCaller<'_> {
    fn as_context(&self) -> <WasmerBackend as WasmBackend>::Context<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
            env: self.env.clone(),
        }
    }
}

impl AsContextMut<WasmerBackend> for WasmerCaller<'_> {
    fn as_context_mut(&mut self) -> <WasmerBackend as WasmBackend>::ContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut(),
            env: self.env.clone(),
        }
    }
}

macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl FuncGetter<WasmerBackend, $args, $rets> for WasmerCaller<'_> {
            unsafe fn get_func(
                &mut self,
                name: &str,
            ) -> ResolveResult<
                Box<
                    dyn FnMut(
                            &mut <WasmerBackend as WasmBackend>::ContextMut<'_>,
                            $args,
                        ) -> RuntimeResult<$rets>
                        + Sync
                        + Send
                        + 'static,
                >,
            > {
                todo!()
            }
        }
    };
}

impl_func_getter!((), ());
impl_func_getter!((), i32);
impl_func_getter!(i32, ());
impl_func_getter!(i32, i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!((i32, i32), i32);
