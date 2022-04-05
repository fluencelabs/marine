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
use crate::module::wit_prelude::WITMemoryView;

use crate::call_wasm_func;
use it_lilo::traits::Allocatable;
use it_lilo::traits::AllocatableError;

use it_lilo::traits::DEFAULT_MEMORY_INDEX;
use wasmer_core::vm::Ctx;

pub(crate) struct LoHelper<'c> {
    allocate_func: &'c AllocateFunc,
    ctx: &'c Ctx,
}

impl<'c> LoHelper<'c> {
    pub(crate) fn new(allocate_func: &'c AllocateFunc, ctx: &'c Ctx) -> Self {
        Self { allocate_func, ctx }
    }
}

impl<'s> Allocatable<WITMemoryView<'s>> for LoHelper<'s> {
    fn allocate(
        &self,
        size: u32,
        type_tag: u32,
    ) -> Result<(u32, WITMemoryView<'s>), AllocatableError> {
        let offset = call_wasm_func!(self.allocate_func, size as _, type_tag as _);
        let view = WITMemoryView(self.ctx.memory(DEFAULT_MEMORY_INDEX as u32).view());
        Ok((offset as u32, view))
    }
}
