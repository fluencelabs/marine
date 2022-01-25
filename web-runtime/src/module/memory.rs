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
use it_lilo::read_ty;
use it_traits::{MemoryAccessError, SequentialWriter};
use it_traits::SequentialReader;
use wasmer_it::interpreter::wasm;
use crate::js_log;
use crate::marine_js::WasmMemory;

pub(super) struct WITMemoryView {
    module_name: String,
}

impl WITMemoryView {
    pub fn new(module_name: String) -> Self {
        crate::js_log("WITMemoryView::new called");

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
    #[allow(unused)]
    size: usize,
    memory: WasmMemory,
}

pub(super) struct JsSequentialWriter {
    offset: usize,
    #[allow(unused)]
    size: usize,
    current_offset: Cell<usize>,
    memory: WasmMemory,
}

impl JsSequentialWriter {
    pub fn new(offset: usize, size: usize, memory: WasmMemory) -> Self {
        Self {
            offset,
            size,
            current_offset: Cell::new(offset),
            memory,
        }
    }
}

impl JsSequentialReader {
    pub fn new(offset: usize, size: usize, memory: WasmMemory) -> Self {
        Self {
            offset: Cell::new(offset),
            size,
            memory,
        }
    }
}

impl SequentialReader for JsSequentialReader {
    fn read_bool(&self) -> bool {
        let offset = self.offset.get();
        let result = self.memory.get(offset) != 0;

        self.offset.set(offset + 1);
        result
    }

    read_ty!(read_u8, u8, 1);
    read_ty!(read_i8, i8, 1);
    read_ty!(read_u16, u16, 2);
    read_ty!(read_i16, i16, 2);
    read_ty!(read_u32, u32, 4);
    read_ty!(read_i32, i32, 4);
    read_ty!(read_f32, f32, 4);
    read_ty!(read_u64, u64, 8);
    read_ty!(read_i64, i64, 8);
    read_ty!(read_f64, f64, 8);
}

impl SequentialWriter for JsSequentialWriter {
    fn start_offset(&self) -> usize {
        self.offset
    }

    fn write_u8(&self, value: u8) {
        self.memory.set(self.current_offset.get(), value);
        self.current_offset.set(self.current_offset.get() + 1);
    }

    fn write_u32(&self, value: u32) {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes);
    }

    fn write_bytes(&self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u8(*byte)
        }
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
        js_log("created WITMemory");

        Self { module_name }
    }
}

impl wasm::structures::Memory<WITMemoryView> for WITMemory {
    fn view(&self) -> WITMemoryView {
        crate::js_log("got memory view");
        WITMemoryView::new(self.module_name.clone())
    }
}
