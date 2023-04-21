use it_memory_traits::MemoryAccessError;
use js_sys::WebAssembly;
use wasm_bindgen::{JsCast, JsValue};
use marine_wasm_backend_traits::prelude::*;
use crate::JsWasmBackend;

#[derive(Clone)]
pub struct JsMemory {
    pub(crate) inner: WebAssembly::Memory,
}

impl JsMemory {
    pub(crate) fn try_from_js(mem: JsValue) -> Option<Self> {
       mem
           .dyn_into::<WebAssembly::Memory>()
           .ok()
           .map(|mem| Self { inner: mem,})
    }
}

// this is safe because its intended to run in single thread
unsafe impl Send for JsMemory {}
unsafe impl Sync for JsMemory {}

impl Memory<JsWasmBackend> for JsMemory {
    fn size(&self, store: &mut <JsWasmBackend as WasmBackend>::ContextMut<'_>) -> usize {
        let buffer = self.inner.buffer();

    }
}

impl it_memory_traits::Memory<JsMemory, DelayedContextLifetime<JsWasmBackend>> for JsMemory {
    fn view(&self) -> JsMemory {
        self.clone()
    }
}

impl it_memory_traits::MemoryView<DelayedContextLifetime<JsWasmBackend>> for JsMemory {
    fn check_bounds(
        &self,
        store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        size: u32,
    ) -> Result<(), MemoryAccessError> {
        todo!()
    }
}

impl it_memory_traits::MemoryReadable<DelayedContextLifetime<JsWasmBackend>> for JsMemory {
    fn read_byte(
        &self,
        store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
    ) -> u8 {
        todo!()
    }

    fn read_array<const COUNT: usize>(
        &self,
        store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
    ) -> [u8; COUNT] {
        todo!()
    }

    fn read_vec(
        &self,
        store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        size: u32,
    ) -> Vec<u8> {
        todo!()
    }
}

impl it_memory_traits::MemoryWritable<DelayedContextLifetime<JsWasmBackend>> for JsMemory {
    fn write_byte(
        &self,
        store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        value: u8,
    ) {
        todo!()
    }

    fn write_bytes(
        &self,
        store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        bytes: &[u8],
    ) {
        todo!()
    }
}
