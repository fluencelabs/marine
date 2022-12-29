use crate::WasmerBackend;

use marine_wasm_backend_traits::*;

#[derive(Clone)]
pub struct WasmerMemory {
    pub(crate) inner: wasmer::Memory
}

pub struct WasmerMemoryView<'c> {
    pub(crate) inner: wasmer::MemoryView<'c>,
}


impl Memory<WasmerBackend> for WasmerMemory {
    fn size(&self, store: &mut <WasmerBackend as WasmBackend>::ContextMut<'_>) -> usize {
        self.inner.view(store).size().bytes().0
    }
}

impl it_memory_traits::Memory<>