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
use std::ops::Deref;
use wasmer_it::interpreter::wasm;
use wasmer_core::memory::{Memory, MemoryView};
use wasmer_core::vm::LocalMemory;
use crate::module::WasmerSequentialReader;

use crate::module::WasmerSequentialWriter;

use it_tratis::{SequentialReader, SequentialWriter};

pub(crate) struct WITMemoryView<'a>(pub(crate) MemoryView<'a, u8>);

#[derive(Clone)]
pub(crate) struct WITMemory(pub(super) Memory);
impl std::ops::Deref for WITMemory {
    type Target = Memory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl wasm::structures::MemoryView for WITMemoryView<'_> {
    fn sequential_writer<'s>(&'s self, offset: usize, _size: usize) -> Box<dyn SequentialWriter + 's> {
        let view = &self.0;
        let slice = view.deref();
        let writer = WasmerSequentialWriter {
            offset,
            slice,
            current_offset: Cell::new(offset),
        };

        Box::new(writer)
    }

    fn sequential_reader<'s>(&'s self, offset: usize, _size: usize) -> Box<dyn SequentialReader + 's> {
        let view = &self.0;
        let slice: &[Cell<u8>] = view.deref();
        let reader = WasmerSequentialReader {
            offset,
            slice,
            current_offset: Cell::new(offset),
        };

        Box::new(reader)
    }
}

impl<'a> wasm::structures::Memory<WITMemoryView<'a>> for WITMemory {
    fn view(&self) -> WITMemoryView<'a> {
        let LocalMemory { base, .. } = unsafe { *self.0.vm_local_memory() };
        let length = self.0.size().bytes().0 / std::mem::size_of::<u8>();

        unsafe { WITMemoryView(MemoryView::new(base as _, length as u32)) }
    }
}
