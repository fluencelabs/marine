use crate::{StoreState, WasmtimeContext, WasmtimeContextMut, WasmtimeWasmBackend, WasmtimeMemory};

use marine_wasm_backend_traits::*;

use wasmtime::AsContext as WasmtimeAsContext;
use wasmtime::AsContextMut as WasmtimeAsContextMut;

pub struct WasmtimeCaller<'c> {
    pub(crate) inner: wasmtime::Caller<'c, StoreState>,
}

impl<'c> Caller<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn memory(&mut self, memory_index: u32) -> Option<WasmtimeMemory> {
        let memory = self
            .inner
            .get_export("memory")?
            .into_memory()?;

        Some(WasmtimeMemory::new(memory))
    }
}

impl<'c> AsContext<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn as_context(&self) -> WasmtimeContext<'_> {
        WasmtimeContext {
            inner: self.inner.as_context(),
        }
    }
}

impl<'c> AsContextMut<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn as_context_mut(&mut self) -> <WasmtimeWasmBackend as WasmBackend>::ContextMut<'_> {
        WasmtimeContextMut {
            inner: self.inner.as_context_mut(),
        }
    }
}

macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl<'c> FuncGetter<WasmtimeWasmBackend, $args, $rets> for WasmtimeCaller<'c> {
            unsafe fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(&mut WasmtimeContextMut<'_>, $args) -> Result<$rets, RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let export = self.inner.get_export(name).unwrap(); //todo handle error
                match export {
                   wasmtime::Extern::Func(f) => {
                        let f = f.typed(&mut self.inner).unwrap(); //todo handle error
                        let closure = move |store: &mut WasmtimeContextMut<'_>, args| {
                            let rets = f.call(&mut store.inner, args).unwrap(); //todo handle error
                            return Ok(rets);
                        };

                        Ok(Box::new(closure))
                    }
                    wasmtime::Extern::Memory(m) => {
                        panic!("caller.get_export returned memory");
                    }
                    _ => {
                        panic!("caller.get_export returned neither memory nor func")
                    }
                }
            }
        }
    };
}

impl_func_getter!((i32, i32), i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!(i32, i32);
impl_func_getter!(i32, ());
impl_func_getter!((), i32);
impl_func_getter!((), ());