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

use it_memory_traits::{MemoryAccessError, MemoryView, MemoryReadable, MemoryWritable};
use wasmer_it::interpreter::wasm;

use wasmer_core::memory::Memory;
use wasmer_core::memory::MemoryView as WasmerMemoryView;
use wasmer_core::vm::LocalMemory;

use std::iter::zip;

pub(crate) struct WITMemoryView<'a>(pub(crate) WasmerMemoryView<'a, u8>);

#[derive(Clone)]
pub(crate) struct WITMemory(pub(super) Memory);
impl std::ops::Deref for WITMemory {
    type Target = Memory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> MemoryReadable for WITMemoryView<'s> {
    fn read_byte(&self, offset: u32) -> u8 {
        self.0[offset as usize].get()
    }

    // needed because clippy suggests using an iterator which looks worse
    #[allow(clippy::needless_range_loop)]
    fn read_array<const COUNT: usize>(&self, offset: u32) -> [u8; COUNT] {
        let mut result = [0u8; COUNT];
        for index in 0..COUNT {
            result[index] = self.0[offset as usize + index].get();
        }

        result
    }

    // needed because clippy suggests using an iterator which looks worse
    #[allow(clippy::needless_range_loop)]
    fn read_vec(&self, offset: u32, size: u32) -> Vec<u8> {
        let end = (offset + size) as usize;
        let start = offset as usize;
        self.0[start..end].iter().map(|v| v.get()).collect()
    }
}

impl<'s> MemoryWritable for WITMemoryView<'s> {
    fn write_byte(&self, offset: u32, value: u8) {
        self.0[offset as usize].set(value);
    }

    fn write_bytes(&self, offset: u32, bytes: &[u8]) {
        let offset = offset as usize;
        let pairs = zip(bytes.iter(), self.0[offset..offset + bytes.len()].iter());

        for (src, dst) in pairs {
            dst.set(*src)
        }
    }
}

impl<'s> MemoryView for WITMemoryView<'s> {
    fn check_bounds(&self, offset: u32, size: u32) -> Result<(), MemoryAccessError> {
        let memory_size = self.0.len() as u32;
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

impl<'a> wasm::structures::Memory<WITMemoryView<'a>> for WITMemory {
    fn view(&self) -> WITMemoryView<'a> {
        let LocalMemory { base, .. } = unsafe { *self.0.vm_local_memory() };
        let length = self.0.size().bytes().0 / std::mem::size_of::<u8>();

        unsafe { WITMemoryView(WasmerMemoryView::new(base as _, length as u32)) }
    }
}
