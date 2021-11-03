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

use super::AllocateFunc;
use crate::call_wasm_func;

use it_lilo::traits::Allocatable;
use it_lilo::traits::AllocatableError;
//use wasmer_core::vm::Ctx;
use crate::marine_js::Ctx;
//use wasmer_core::vm::LocalMemory;

use std::cell::Cell;

pub(crate) struct LoHelper<'c> {
    ctx: &'c Ctx,
    allocate_func: &'c AllocateFunc,
}

impl<'c> LoHelper<'c> {
    pub(crate) fn new(ctx: &'c Ctx, allocate_func: &'c AllocateFunc) -> Self {
        Self { ctx, allocate_func }
    }
}

impl Allocatable for LoHelper<'_> {
    fn allocate(&self, size: u32, type_tag: u32) -> Result<usize, AllocatableError> {
        let offset = call_wasm_func!(self.allocate_func, size as _, type_tag as _);
        Ok(offset as _)
    }

    fn memory_slice(&self, _memory_index: usize) -> Result<&[Cell<u8>], AllocatableError> {
        /*let memory = self.ctx.memory(memory_index as _);

        let LocalMemory { base, .. } = unsafe { *memory.vm_local_memory() };
        let length = memory.size().bytes().0 / std::mem::size_of::<u8>();

        let mut_slice: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(base, length) };
        let cell_slice: &Cell<[u8]> = Cell::from_mut(mut_slice);
        let slice = cell_slice.as_slice_of_cells();

        Ok(slice)*/

        // WEB TODO: rewrite using js functions
        Err(AllocatableError::AllocateCallFailed)
    }
}
