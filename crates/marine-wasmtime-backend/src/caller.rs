use crate::{StoreState, WasmtimeContext, WasmtimeContextMut, WasmtimeWasmBackend, WasmtimeMemory};

use marine_wasm_backend_traits::*;

use wasmtime::AsContext as WasmtimeAsContext;
use wasmtime::AsContextMut as WasmtimeAsContextMut;

pub struct WasmtimeCaller<'c> {
    pub(crate) inner: wasmtime::Caller<'c, StoreState>,
}

impl<'c> Caller<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn memory(&mut self, _memory_index: u32) -> Option<WasmtimeMemory> {
        let memory = self.inner.get_export("memory")?.into_memory()?;

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
                let export = self
                    .inner
                    .get_export(name)
                    .ok_or(ResolveError::ExportNotFound(name.to_string()))?;

                match export {
                    wasmtime::Extern::Func(f) => {
                        let f = f
                            .typed(&mut self.inner)
                            .map_err(|e| ResolveError::Other(e))?;

                        let closure = move |store: &mut WasmtimeContextMut<'_>, args| {
                            f.call(&mut store.inner, args).map_err(|e| {
                                if let Some(_) = e.downcast_ref::<wasmtime::Trap>() {
                                    RuntimeError::Trap(e)
                                } else {
                                    RuntimeError::Other(e)
                                }
                            })
                        };

                        Ok(Box::new(closure))
                    }
                    wasmtime::Extern::Memory(_) => Err(ResolveError::ExportTypeMismatch(
                        "function".to_string(),
                        "memory".to_string(),
                    )),
                    _ => Err(ResolveError::ExportTypeMismatch(
                        "function".to_string(),
                        "neither memory nor function".to_string(),
                    )),
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
