use it_memory_traits::MemoryAccessError;
use marine_wasm_backend_traits::{DelayedContextLifetime, Memory};
use crate::{WasmtimeContextMut, WasmtimeWasmBackend};

static MEMORY_ACCESS_EXPECTATION: &str =
    "api requires checking memory bounds before accessing memory";

#[derive(Clone)]
pub struct WasmtimeMemory {
    memory: wasmtime::Memory,
}

impl WasmtimeMemory {
    pub(crate) fn new(memory: wasmtime::Memory) -> Self {
        Self { memory }
    }
}

impl it_memory_traits::Memory<WasmtimeMemory, DelayedContextLifetime<WasmtimeWasmBackend>>
    for WasmtimeMemory
{
    fn view(&self) -> WasmtimeMemory {
        self.clone()
    }
}

impl Memory<WasmtimeWasmBackend> for WasmtimeMemory {
    fn size(&self, store: &mut WasmtimeContextMut<'_>) -> usize {
        self.memory.data_size(store) as usize
    }
}

impl it_memory_traits::MemoryReadable<DelayedContextLifetime<WasmtimeWasmBackend>>
    for WasmtimeMemory
{
    fn read_byte(&self, store: &mut WasmtimeContextMut<'_>, offset: u32) -> u8 {
        let mut value = [0u8];
        self.memory
            .read(&mut store.inner, offset as usize, &mut value)
            .expect(MEMORY_ACCESS_EXPECTATION);

        value[0]
    }

    fn read_array<const COUNT: usize>(
        &self,
        store: &mut WasmtimeContextMut<'_>,
        offset: u32,
    ) -> [u8; COUNT] {
        let mut value = [0u8; COUNT];
        self.memory
            .read(&mut store.inner, offset as usize, &mut value)
            .expect(MEMORY_ACCESS_EXPECTATION);
        value
    }

    fn read_vec(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, size: u32) -> Vec<u8> {
        let mut value = vec![0u8; size as usize];
        self.memory
            .read(&mut store.inner, offset as usize, &mut value)
            .expect(MEMORY_ACCESS_EXPECTATION);
        value
    }
}

impl it_memory_traits::MemoryWritable<DelayedContextLifetime<WasmtimeWasmBackend>>
    for WasmtimeMemory
{
    fn write_byte(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, value: u8) {
        let buffer = [value];
        self.memory
            .write(&mut store.inner, offset as usize, &buffer)
            .unwrap() // todo handle error
    }

    fn write_bytes(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, bytes: &[u8]) {
        self.memory
            .write(&mut store.inner, offset as usize, bytes)
            .unwrap() // todo handle error
    }
}

impl it_memory_traits::MemoryView<DelayedContextLifetime<WasmtimeWasmBackend>> for WasmtimeMemory {
    fn check_bounds(
        &self,
        store: &mut WasmtimeContextMut<'_>,
        offset: u32,
        size: u32,
    ) -> Result<(), MemoryAccessError> {
        let memory_size = self.memory.data_size(&mut store.inner) as u64;
        if memory_size <= (offset + size) as u64 {
            Err(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size: memory_size as u32, // todo rewrite api when memory64 arrives
            })
        } else {
            Ok(())
        }
    }
}
