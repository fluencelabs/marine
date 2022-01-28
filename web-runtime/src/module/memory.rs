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

use std::cell::Cell;
use it_traits::{MemoryAccessError, SequentialWriter};
use it_traits::SequentialReader;
use wasmer_it::interpreter::wasm;
use crate::marine_js::WasmMemory;

pub(super) struct WITMemoryView {
    module_name: String,
}

impl WITMemoryView {
    pub fn new(module_name: String) -> Self {
        Self { module_name }
    }

    fn check_bounds(
        &self,
        offset: usize,
        size: usize,
        memory_size: usize,
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
    offset: Cell<usize>,
    data: Vec<u8>,
    memory: WasmMemory,
    start_offset: usize,
}

pub(super) struct JsSequentialWriter {
    offset: usize,
    data: Vec<Cell<u8>>,
    current_offset: Cell<usize>,
    memory: WasmMemory,
}

impl JsSequentialWriter {
    pub fn new(offset: usize, size: usize, memory: WasmMemory) -> Self {
        Self {
            offset,
            data: vec![Cell::new(0u8); size],
            current_offset: Cell::new(offset),
            memory,
        }
    }
}

impl JsSequentialReader {
    pub fn new(offset: usize, size: usize, memory: WasmMemory) -> Self {
        let mut data = vec![0; size];
        memory.get_range(offset, &mut data);
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
        let start = offset - self.start_offset;

        let mut result = [0u8; COUNT];
        result.copy_from_slice(&self.data[start..start + COUNT]);
        self.offset.set(offset + COUNT);

        result
    }
}

impl SequentialWriter for JsSequentialWriter {
    fn start_offset(&self) -> usize {
        self.offset
    }

    fn write_u8(&self, value: u8) {
        let offset = self.current_offset.get();
        self.data[offset].set(value);
        self.current_offset.set(offset + 1);
    }

    fn write_u32(&self, value: u32) {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes);
    }

    fn write_bytes(&self, bytes: &[u8]) {
        let offset = self.current_offset.get();
        let start = offset - self.start_offset();
        for index in 0..bytes.len() {
            self.data[start + index].set(bytes[index]);
        }

        self.current_offset.set(offset + bytes.len());
    }
}

impl Drop for JsSequentialWriter {
    fn drop(&mut self) {
        let data: Vec<u8> = self.data.iter().map(|v| v.get()).collect();
        self.memory.set_range(self.start_offset(), data.as_slice());
    }
}

impl<'v> wasm::structures::MemoryView<'v> for WITMemoryView {
    type SR = JsSequentialReader;
    type SW = JsSequentialWriter;

    fn sequential_writer(
        &'v self,
        offset: usize,
        size: usize,
    ) -> Result<Self::SW, MemoryAccessError> {
        let memory = WasmMemory::new(self.module_name.clone());
        let memory_size = memory.len();

        self.check_bounds(offset, size, memory_size)?;

        Ok(JsSequentialWriter::new(offset, size, memory))
    }

    fn sequential_reader(
        &'v self,
        offset: usize,
        size: usize,
    ) -> Result<Self::SR, MemoryAccessError> {
        let memory = WasmMemory::new(self.module_name.clone());
        let memory_size = memory.len();

        self.check_bounds(offset, size, memory_size)?;

        Ok(JsSequentialReader::new(offset, size, memory))
    }
}

#[derive(Clone)]
pub(super) struct WITMemory {
    module_name: String,
}

impl WITMemory {
    pub fn new(module_name: String) -> Self {
        Self { module_name }
    }
}

impl wasm::structures::Memory<WITMemoryView> for WITMemory {
    fn view(&self) -> WITMemoryView {
        WITMemoryView::new(self.module_name.clone())
    }
}
