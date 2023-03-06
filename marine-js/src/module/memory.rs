/*
 * Copyright 2020 Fluence Labs Limited
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

use crate::marine_js::JsWasmMemoryProxy;

use it_memory_traits::MemoryAccessError;
use it_memory_traits::MemoryView;
use it_memory_traits::MemoryWritable;
use it_memory_traits::MemoryReadable;
use wasmer_it::interpreter::wasm;

use std::rc::Rc;
use crate::module::wit_store::WITStore;

pub(super) struct WITMemoryView {
    memory: JsWasmMemoryProxy,
}

impl WITMemoryView {
    pub fn new(module_name: Rc<String>) -> Self {
        Self {
            memory: JsWasmMemoryProxy::new(module_name),
        }
    }
}

impl MemoryWritable<WITStore> for WITMemoryView {
    fn write_byte(&self, _store: &mut (), offset: u32, value: u8) {
        self.memory.set(offset, value);
    }

    fn write_bytes(&self, _store: &mut (), offset: u32, bytes: &[u8]) {
        self.memory.set_range(offset, bytes);
    }
}

impl MemoryReadable<WITStore> for WITMemoryView {
    fn read_byte(&self, _store: &mut (), offset: u32) -> u8 {
        self.memory.get(offset)
    }

    fn read_array<const COUNT: usize>(&self, _store: &mut (), offset: u32) -> [u8; COUNT] {
        let mut result = [0u8; COUNT];
        let data = self.memory.get_range(offset, COUNT as u32);
        result.copy_from_slice(&data[..COUNT]);
        result
    }

    fn read_vec(&self, _store: &mut (), offset: u32, size: u32) -> Vec<u8> {
        self.memory.get_range(offset, size)
    }
}

impl MemoryView<WITStore> for WITMemoryView {
    fn check_bounds(
        &self,
        _store: &mut (),
        offset: u32,
        size: u32,
    ) -> Result<(), MemoryAccessError> {
        let memory_size = self.memory.len();
        if offset + size >= memory_size {
            Err(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size,
            })
        } else {
            Ok(())
        }
    }
}

#[derive(Clone)]
pub(super) struct WITMemory {
    module_name: Rc<String>,
}

impl WITMemory {
    pub fn new(module_name: Rc<String>) -> Self {
        Self { module_name }
    }
}

impl wasm::structures::Memory<WITMemoryView, WITStore> for WITMemory {
    fn view(&self) -> WITMemoryView {
        WITMemoryView::new(self.module_name.clone())
    }
}
