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
use std::cell::Cell;
use std::marker::PhantomData;
//use wasmer_core::memory::{Memory, MemoryView};

// WEB TODO: implement with js interface

pub(super) struct WITMemoryView<'a> {
    data3: PhantomData<&'a i32>,
    data: Vec<Cell<u8>>,
}

impl<'a> std::ops::Deref for WITMemoryView<'a> {
    type Target = [std::cell::Cell<u8>];

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

#[derive(Clone)]
pub(super) struct WITMemory {
    memory: i32,
}

impl wasm::structures::MemoryView for WITMemoryView<'_> {}

impl<'a> wasm::structures::Memory<WITMemoryView<'a>> for WITMemory {
    fn view(&self) -> WITMemoryView<'a> {
        WITMemoryView {
            data: Vec::new(),
            data3: <_>::default()
        }
    }
}
