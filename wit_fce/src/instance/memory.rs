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

use wasmer_interface_types::interpreter::wasm;
use wasmer_runtime_core::memory::{Memory, MemoryView};

pub struct WITMemoryView<'a>(pub MemoryView<'a, u8>);
impl<'a> std::ops::Deref for WITMemoryView<'a> {
    type Target = [std::cell::Cell<u8>];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct WITMemory(pub Memory);
impl std::ops::Deref for WITMemory {
    type Target = Memory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl wasm::structures::MemoryView for WITMemoryView<'_> {}

impl<'a> wasm::structures::Memory<WITMemoryView<'a>> for WITMemory {
    fn view(&self) -> WITMemoryView<'a> {
        use wasmer_runtime_core::vm::LocalMemory;

        let LocalMemory { base, .. } = unsafe { *self.0.vm_local_memory() };
        let length = self.0.size().bytes().0 / std::mem::size_of::<u8>();

        unsafe { WITMemoryView(MemoryView::new(base as _, length as u32)) }
    }
}
