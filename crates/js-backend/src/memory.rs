/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::JsWasmBackend;

use it_memory_traits::MemoryAccessError;
use marine_wasm_backend_traits::prelude::*;

use js_sys::WebAssembly;
use wasm_bindgen::JsCast;

static MEMORY_ACCESS_CONTRACT: &str =
    "user is expected to check memory bounds before accessing memory";

#[derive(Clone)]
pub struct JsMemory {
    pub(crate) inner: WebAssembly::Memory,
}

impl JsMemory {
    pub(crate) fn new(mem: WebAssembly::Memory) -> Self {
        Self { inner: mem }
    }
}

/// Safety: js-backend is expected to run in single-threaded environment,
/// so it is safe to assume that every type is Send + Sync
unsafe impl Send for JsMemory {}
unsafe impl Sync for JsMemory {}

impl JsMemory {
    fn array_buffer(&self) -> js_sys::ArrayBuffer {
        self.inner.buffer().unchecked_into::<js_sys::ArrayBuffer>()
    }

    fn uint8_array(&self) -> js_sys::Uint8Array {
        let buffer = self.array_buffer();
        js_sys::Uint8Array::new(&buffer)
    }
}

impl Memory<JsWasmBackend> for JsMemory {
    fn size(&self, _store: &mut <JsWasmBackend as WasmBackend>::ContextMut<'_>) -> usize {
        self.array_buffer().byte_length() as usize
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
        let memory_size = self.size(store);
        let end = offset
            .checked_add(size)
            .ok_or(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size: memory_size as u32,
            })?;

        if end as usize >= memory_size {
            return Err(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size: memory_size as u32,
            });
        }

        Ok(())
    }
}

impl it_memory_traits::MemoryReadable<DelayedContextLifetime<JsWasmBackend>> for JsMemory {
    fn read_byte(
        &self,
        _store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
    ) -> u8 {
        self.uint8_array().get_index(offset)
    }

    fn read_array<const COUNT: usize>(
        &self,
        _store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
    ) -> [u8; COUNT] {
        let mut result = [0u8; COUNT];
        let end = offset
            .checked_add(COUNT as u32)
            .expect(MEMORY_ACCESS_CONTRACT);
        self.uint8_array()
            .subarray(offset, end)
            .copy_to(result.as_mut_slice());
        result
    }

    fn read_vec(
        &self,
        _store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        size: u32,
    ) -> Vec<u8> {
        let mut result = vec![0u8; size as usize];
        let end = offset.checked_add(size).expect(MEMORY_ACCESS_CONTRACT);
        self.uint8_array()
            .subarray(offset, end)
            .copy_to(result.as_mut_slice());
        result
    }
}

impl it_memory_traits::MemoryWritable<DelayedContextLifetime<JsWasmBackend>> for JsMemory {
    fn write_byte(
        &self,
        _store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        value: u8,
    ) {
        self.uint8_array().set_index(offset, value);
    }

    fn write_bytes(
        &self,
        _store: &mut <DelayedContextLifetime<JsWasmBackend> as it_memory_traits::Store>::ActualStore<
            '_,
        >,
        offset: u32,
        bytes: &[u8],
    ) {
        let end = offset
            .checked_add(bytes.len() as u32)
            .expect(MEMORY_ACCESS_CONTRACT);
        self.uint8_array().subarray(offset, end).copy_from(bytes);
    }
}
