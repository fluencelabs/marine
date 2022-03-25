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

use it_memory_traits::{MemoryAccessError, SequentialWriter};
use it_memory_traits::SequentialReader;
use wasmer_it::interpreter::wasm;

use std::cell::Cell;
use std::rc::Rc;

pub(super) struct WITMemoryView {
    module_name: Rc<String>,
}

impl WITMemoryView {
    pub fn new(module_name: Rc<String>) -> Self {
        Self { module_name }
    }

    fn check_bounds(
        &self,
        offset: u32,
        size: u32,
        memory_size: u32,
    ) -> Result<(), MemoryAccessError> {
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

pub(super) struct JsSequentialReader {
    offset: Cell<u32>,
    data: Vec<u8>,
    memory: JsWasmMemoryProxy,
    start_offset: u32,
}

pub(super) struct JsSequentialWriter {
    offset: u32,
    current_offset: Cell<u32>,
    memory: JsWasmMemoryProxy,
}

impl JsSequentialWriter {
    pub fn new(offset: u32, memory: JsWasmMemoryProxy) -> Self {
        Self {
            offset,
            current_offset: Cell::new(offset),
            memory,
        }
    }
}

impl JsSequentialReader {
    pub fn new(offset: u32, size: u32, memory: JsWasmMemoryProxy) -> Self {
        let data = memory.get_range(offset, size);
        Self {
            offset: Cell::new(offset),
            data,
            memory,
            start_offset: offset,
        }
    }
}

impl SequentialReader for JsSequentialReader {
    fn read_byte(&self) -> u8 {
        let offset = self.offset.get();
        let result = self.memory.get(offset);

        self.offset.set(offset + 1);
        result
    }

    fn read_bytes<const COUNT: usize>(&self) -> [u8; COUNT] {
        let offset = self.offset.get();
        let start = (offset - self.start_offset) as usize;

        let mut result = [0u8; COUNT];
        result.copy_from_slice(&self.data[start..start + COUNT]);
        self.offset.set(offset + COUNT as u32);

        result
    }
}

impl SequentialWriter for JsSequentialWriter {
    fn start_offset(&self) -> u32 {
        self.offset
    }

    fn write_u8(&self, value: u8) {
        let offset = self.current_offset.get();
        self.memory.set(offset, value);
        self.current_offset.set(offset + 1);
    }

    fn write_u32(&self, value: u32) {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes);
    }

    fn write_bytes(&self, bytes: &[u8]) {
        let offset = self.current_offset.get();
        self.memory.set_range(offset, bytes);
        self.current_offset.set(offset + bytes.len() as u32);
    }
}

impl<'v> wasm::structures::SequentialMemoryView<'v> for WITMemoryView {
    type SR = JsSequentialReader;
    type SW = JsSequentialWriter;

    fn sequential_writer(&'v self, offset: u32, size: u32) -> Result<Self::SW, MemoryAccessError> {
        let memory = JsWasmMemoryProxy::new(self.module_name.clone());
        let memory_size = memory.len();

        self.check_bounds(offset, size, memory_size)?;

        Ok(JsSequentialWriter::new(offset, memory))
    }

    fn sequential_reader(&'v self, offset: u32, size: u32) -> Result<Self::SR, MemoryAccessError> {
        let memory = JsWasmMemoryProxy::new(self.module_name.clone());
        let memory_size = memory.len();

        self.check_bounds(offset, size, memory_size)?;

        Ok(JsSequentialReader::new(offset, size, memory))
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

impl wasm::structures::Memory<WITMemoryView> for WITMemory {
    fn view(&self) -> WITMemoryView {
        WITMemoryView::new(self.module_name.clone())
    }
}
