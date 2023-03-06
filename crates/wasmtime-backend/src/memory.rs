/*
 * Copyright 2023 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::WasmtimeContextMut;
use crate::WasmtimeWasmBackend;

use marine_wasm_backend_traits::DelayedContextLifetime;
use marine_wasm_backend_traits::Memory;

use it_memory_traits::MemoryAccessError;

static MEMORY_ACCESS_CONTRACT: &str = "api requires checking memory bounds before accessing memory";

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
            .expect(MEMORY_ACCESS_CONTRACT);

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
            .expect(MEMORY_ACCESS_CONTRACT);
        value
    }

    fn read_vec(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, size: u32) -> Vec<u8> {
        let mut value = vec![0u8; size as usize];
        self.memory
            .read(&mut store.inner, offset as usize, &mut value)
            .expect(MEMORY_ACCESS_CONTRACT);
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
            .expect(MEMORY_ACCESS_CONTRACT);
    }

    fn write_bytes(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, bytes: &[u8]) {
        self.memory
            .write(&mut store.inner, offset as usize, bytes)
            .expect(MEMORY_ACCESS_CONTRACT);
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
        if memory_size <= (offset as u64) + (size as u64) {
            Err(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size: memory_size as u32, // TODO rewrite api when memory64 arrives
            })
        } else {
            Ok(())
        }
    }
}
