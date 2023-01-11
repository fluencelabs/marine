use crate::{StoreState, WasmtimeContext, WasmtimeContextMut, WasmtimeWasmBackend, WasmtimeMemory};

use marine_wasm_backend_traits::*;

use wasmtime::AsContext as WasmtimeAsContext;
use wasmtime::AsContextMut as WasmtimeAsContextMut;

pub struct WasmtimeCaller<'c> {
    pub(crate) inner: wasmtime::Caller<'c, StoreState>,
}

impl<'c> Caller<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn memory(&mut self, memory_index: u32) -> WasmtimeMemory {
        let memory = self.inner.get_export("memory").unwrap(); // todo: handle error

        WasmtimeMemory::new(memory.into_memory().unwrap()) // todo: handle error
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
