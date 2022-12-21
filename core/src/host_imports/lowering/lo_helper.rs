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

use std::marker::PhantomData;
use super::AllocateFunc;

use crate::call_wasm_func;
use it_lilo::traits::Allocatable;
use it_lilo::traits::AllocatableError;
use it_memory_traits::MemoryView;
use it_memory_traits::Memory;

pub(crate) struct LoHelper<'c, MV: MemoryView, M: Memory<MV>> {
    allocate_func: &'c AllocateFunc,
    memory: M,
    _memory_view_phantom: PhantomData<MV>,
}

impl<'c, MV: MemoryView, M: Memory<MV>> LoHelper<'c, MV, M> {
    pub(crate) fn new(allocate_func: &'c AllocateFunc, memory: M) -> Self {
        Self {
            allocate_func,
            memory,
            _memory_view_phantom: <_>::default(),
        }
    }
}

impl<'s, MV: MemoryView, M: Memory<MV>, Store: it_memory_traits::Store> Allocatable<MV, Store>
    for LoHelper<'s, MV, M>
{
    fn allocate(
        &mut self,
        _store: &mut <Store as it_memory_traits::Store>::ActualStore<'_>,
        size: u32,
        type_tag: u32,
    ) -> Result<(u32, MV), AllocatableError> {
        let offset = call_wasm_func!(self.allocate_func, size as _, type_tag as _);
        Ok((offset as u32, self.memory.view()))
    }
}
