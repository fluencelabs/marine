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

use crate::{WasmerBackend, WasmerContextMut};

use marine_wasm_backend_traits::prelude::*;

use it_memory_traits::{MemoryAccessError};

static MEMORY_ACCESS_EXPECTATION: &str = "User should check memory bounds prior to accessing";

#[derive(Clone)]
pub struct WasmerMemory {
    pub(crate) inner: wasmer::Memory,
}

impl From<wasmer::Memory> for WasmerMemory {
    fn from(memory: wasmer::Memory) -> Self {
        WasmerMemory { inner: memory }
    }
}

impl Memory<WasmerBackend> for WasmerMemory {
    fn size(&self, store: &mut <WasmerBackend as WasmBackend>::ContextMut<'_>) -> usize {
        self.inner.view(store).size().bytes().0
    }
}

impl it_memory_traits::Memory<WasmerMemory, DelayedContextLifetime<WasmerBackend>>
    for WasmerMemory
{
    fn view(&self) -> WasmerMemory {
        self.clone()
    }
}

impl it_memory_traits::MemoryView<DelayedContextLifetime<WasmerBackend>> for WasmerMemory {
    fn check_bounds(
        &self,
        store: &mut WasmerContextMut<'_>,
        offset: u32,
        size: u32,
    ) -> Result<(), MemoryAccessError> {
        let memory_size = self.inner.view(store).size().bytes().0;
        if (offset as usize + size as usize) >= memory_size {
            return Err(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size: memory_size as u32,
            });
        }

        Ok(())
    }
}

impl it_memory_traits::MemoryReadable<DelayedContextLifetime<WasmerBackend>> for WasmerMemory {
    fn read_byte(
        &self,
        store: &mut <DelayedContextLifetime<WasmerBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
    ) -> u8 {
        self.inner
            .view(store)
            .read_u8(offset as u64)
            .expect(MEMORY_ACCESS_EXPECTATION)
    }

    fn read_array<const COUNT: usize>(
        &self,
        store: &mut <DelayedContextLifetime<WasmerBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
    ) -> [u8; COUNT] {
        let mut buf = [0u8; COUNT];
        self.inner
            .view(store)
            .read(offset as u64, &mut buf)
            .expect(MEMORY_ACCESS_EXPECTATION);
        buf
    }

    fn read_vec(
        &self,
        store: &mut <DelayedContextLifetime<WasmerBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        size: u32,
    ) -> Vec<u8> {
        let mut buf = vec![0u8; size as usize];
        self.inner
            .view(store)
            .read(offset as u64, &mut buf)
            .expect(MEMORY_ACCESS_EXPECTATION);
        buf
    }
}

impl it_memory_traits::MemoryWritable<DelayedContextLifetime<WasmerBackend>> for WasmerMemory {
    fn write_byte(
        &self,
        store: &mut <DelayedContextLifetime<WasmerBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        value: u8,
    ) {
        self.inner
            .view(store)
            .write_u8(offset as u64, value)
            .expect(MEMORY_ACCESS_EXPECTATION);
    }

    fn write_bytes(
        &self,
        store: &mut <DelayedContextLifetime<WasmerBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        bytes: &[u8],
    ) {
        self.inner
            .view(store)
            .write(offset as u64, bytes)
            .expect(MEMORY_ACCESS_EXPECTATION);
    }
}
