/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
    // Wasmtime does not have the idea of MemoryView, while Wasmer has.
    // And our interface-types implementation has MemoryView concept
    // So, MemoryView in Wasmtime is just the memory.
    fn view(&self) -> WasmtimeMemory {
        self.clone()
    }
}

impl Memory<WasmtimeWasmBackend> for WasmtimeMemory {
    fn size(&self, store: &mut WasmtimeContextMut<'_>) -> usize {
        self.memory.data_size(store)
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
        let memory_size = self.memory.data_size(&mut store.inner);
        let final_size = offset
            .checked_add(size)
            .ok_or(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size: memory_size as u32,
            })? as usize;

        if memory_size <= final_size {
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
