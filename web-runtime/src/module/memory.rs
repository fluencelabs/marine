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

use wasmer_it::interpreter::wasm;
use wasmer_it::interpreter::wasm::structures::MemSlice2;
use crate::js_log;
use crate::marine_js::WasmMemory;
//use wasmer_core::memory::{Memory, MemoryView};

// WEB TODO: implement with js interface

pub(super) struct WITMemoryView<'a> {
    slice: MemSlice2<'a>,
}

impl<'a> WITMemoryView<'a> {
    pub fn new(memory: &'a WasmMemory) -> Self {
        crate::js_log("WITMemoryView::new called");

         Self {
            slice: MemSlice2 {
                slice_ref: memory
            }
        }
    }
}


impl<'a> std::ops::Deref for WITMemoryView<'a> {
    type Target = MemSlice2<'a>;

    fn deref(&self) -> &Self::Target {
        crate::js_log("got slice from WITMemoryView");
        &self.slice
    }
}

impl wasm::structures::MemoryView for WITMemoryView<'static> {}

const MEMORY_CONTAINTER: WasmMemory = WasmMemory {
    module_name: "greeting",
};

#[derive(Clone)]
pub(super) struct WITMemory {
}

impl WITMemory {
    pub fn new(_module_name: String) -> Self {
        js_log("created WITMemory");

        Self {}
    }
}
impl wasm::structures::Memory<WITMemoryView<'static>> for WITMemory {
    fn view(&self) -> WITMemoryView<'static> {
        crate::js_log("got memory view");
        WITMemoryView::new(&MEMORY_CONTAINTER)
    }
}
