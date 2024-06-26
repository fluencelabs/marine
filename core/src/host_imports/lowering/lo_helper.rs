/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
    fn allocate<'this, 'store: 'this, 'ctx: 'this>(
        &'this mut self,
        store: &'store mut <WB as WasmBackend>::ContextMut<'ctx>,
        size: u32,
        type_tag: u32,
    ) -> BoxFuture<'this, Result<(u32, MV), AllocatableError>> {
        async move {
            let offset = (self.allocate_func)(store, (size as _, type_tag as _))
                .await
                .map_err(|e| AllocatableError::AllocateCallFailed {
                    reason: anyhow::anyhow!(e),
                })?;
            Ok((offset as u32, self.memory.view()))
        }
        .boxed()
    }
}
