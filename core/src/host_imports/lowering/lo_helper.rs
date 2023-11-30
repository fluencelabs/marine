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

use marine_wasm_backend_traits::DelayedContextLifetime;
use marine_wasm_backend_traits::WasmBackend;

use it_lilo::traits::Allocatable;
use it_lilo::traits::AllocatableError;
use it_memory_traits::MemoryView;
use it_memory_traits::Memory;

use futures::future::BoxFuture;
use futures::FutureExt;

use std::marker::PhantomData;

pub(crate) struct LoHelper<
    WB: WasmBackend,
    MV: MemoryView<DelayedContextLifetime<WB>>,
    M: Memory<MV, DelayedContextLifetime<WB>>,
> {
    allocate_func: AllocateFunc<WB>,
    memory: M,
    _memory_view_phantom: PhantomData<MV>,
}

impl<
        WB: WasmBackend,
        MV: MemoryView<DelayedContextLifetime<WB>>,
        M: Memory<MV, DelayedContextLifetime<WB>>,
    > LoHelper<WB, MV, M>
{
    pub(crate) fn new(allocate_func: AllocateFunc<WB>, memory: M) -> Self {
        Self {
            allocate_func,
            memory,
            _memory_view_phantom: PhantomData,
        }
    }
}

impl<
        WB: WasmBackend,
        MV: MemoryView<DelayedContextLifetime<WB>>,
        M: Memory<MV, DelayedContextLifetime<WB>>,
    > Allocatable<MV, DelayedContextLifetime<WB>> for LoHelper<WB, MV, M>
{
    fn allocate<'this, 'ctx1: 'this, 'ctx2: 'ctx1>(
        &'this mut self,
        store: &'ctx1 mut <WB as WasmBackend>::ContextMut<'ctx2>,
        size: u32,
        type_tag: u32,
    ) -> BoxFuture<'this, Result<(u32, MV), AllocatableError>> {
        async move {
            let offset = (self.allocate_func)(store, (size as _, type_tag as _))
                .await
                .unwrap();
            Ok((offset as u32, self.memory.view()))
        }
        .boxed()
    }
}
