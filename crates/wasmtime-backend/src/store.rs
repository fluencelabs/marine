/*
 * Copyright 2023 Fluence Labs Limited
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

use crate::StoreState;
use crate::WasmtimeWasmBackend;

use marine_wasm_backend_traits::prelude::*;

use wasmtime::ResourceLimiter;
use wasmtime::StoreContext;
use wasmtime::StoreContextMut;
use wasmtime::AsContext as WasmtimeAsContext;
use wasmtime::AsContextMut as WasmtimeAsContextMut;

use std::default::Default;

/// A type that is used to store resources allocated by runtime. It includes memories, functions,
/// tables, globals and so on. More information here: https://webassembly.github.io/spec/core/exec/runtime.html#store.
/// Because of that, most of the methods in API require a handle to store to function.
pub struct WasmtimeStore {
    pub(crate) inner: wasmtime::Store<StoreState>,
}

/// Temporary immutable handle to `Store`, used to interact with stored data.
pub struct WasmtimeContext<'s> {
    pub(crate) inner: wasmtime::StoreContext<'s, StoreState>,
}

/// Temporary mutable handle to `Store`, used to interact with stored data.
pub struct WasmtimeContextMut<'s> {
    pub(crate) inner: wasmtime::StoreContextMut<'s, StoreState>,
}

#[derive(Default)]
pub struct MemoryLimiter {
    remaining_memory: u64,
    allocation_stats: MemoryAllocationStats,
}

impl Store<WasmtimeWasmBackend> for WasmtimeStore {
    fn new(backend: &WasmtimeWasmBackend) -> Self {
        let mut store = wasmtime::Store::new(&backend.engine, <_>::default());
        store.epoch_deadline_async_yield_and_update(1);
        Self { inner: store }
    }

    fn set_total_memory_limit(&mut self, total_memory_limit: u64) {
        let limits = MemoryLimiter::new(total_memory_limit);
        self.inner.data_mut().limits = limits;
        self.inner.limiter(|store_state| &mut store_state.limits);
    }

    fn report_memory_allocation_stats(&self) -> Option<MemoryAllocationStats> {
        Some(self.inner.data().limits.allocation_stats.clone())
    }

    fn clear_allocation_stats(&mut self) {
        self.inner.data_mut().limits.allocation_stats = MemoryAllocationStats::default();
    }
}

impl MemoryLimiter {
    pub(crate) fn new(max_total_memory: u64) -> Self {
        Self {
            remaining_memory: max_total_memory,
            allocation_stats: <_>::default(),
        }
    }

    pub(crate) fn count_allocation_reject(&mut self) {
        self.allocation_stats.allocation_rejects += 1;
    }

    pub(crate) fn try_alloc(&mut self, amount: u64) -> bool {
        if let Some(remaining_memory) = self.remaining_memory.checked_sub(amount) {
            self.remaining_memory = remaining_memory;
            true
        } else {
            self.count_allocation_reject();
            false
        }
    }
}

impl ResourceLimiter for MemoryLimiter {
    fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        let grow_size = (desired - current) as u64;
        Ok(self.try_alloc(grow_size))
    }

    fn table_growing(
        &mut self,
        current: u32,
        desired: u32,
        _maximum: Option<u32>,
    ) -> wasmtime::Result<bool> {
        let grow_size = (desired - current) as usize * std::mem::size_of::<usize>();
        Ok(self.try_alloc(grow_size as u64))
    }
}

impl<'c> Context<WasmtimeWasmBackend> for WasmtimeContext<'c> {}

impl<'c> ContextMut<WasmtimeWasmBackend> for WasmtimeContextMut<'c> {}

impl AsContext<WasmtimeWasmBackend> for WasmtimeStore {
    fn as_context(&self) -> WasmtimeContext<'_> {
        WasmtimeContext {
            inner: self.inner.as_context(),
        }
    }
}

impl AsContextMut<WasmtimeWasmBackend> for WasmtimeStore {
    fn as_context_mut(&mut self) -> WasmtimeContextMut<'_> {
        WasmtimeContextMut {
            inner: self.inner.as_context_mut(),
        }
    }
}

impl<'ctx> AsContext<WasmtimeWasmBackend> for WasmtimeContext<'ctx> {
    fn as_context(&self) -> WasmtimeContext<'_> {
        WasmtimeContext {
            inner: self.inner.as_context(),
        }
    }
}

impl<'ctx> AsContext<WasmtimeWasmBackend> for WasmtimeContextMut<'ctx> {
    fn as_context(&self) -> WasmtimeContext<'_> {
        WasmtimeContext {
            inner: self.inner.as_context(),
        }
    }
}

impl<'ctx> AsContextMut<WasmtimeWasmBackend> for WasmtimeContextMut<'ctx> {
    fn as_context_mut(&mut self) -> WasmtimeContextMut<'_> {
        WasmtimeContextMut {
            inner: self.inner.as_context_mut(),
        }
    }
}

impl wasmtime::AsContext for WasmtimeStore {
    type Data = StoreState;

    fn as_context(&self) -> StoreContext<'_, Self::Data> {
        self.inner.as_context()
    }
}

impl wasmtime::AsContextMut for WasmtimeStore {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, Self::Data> {
        self.inner.as_context_mut()
    }
}

impl wasmtime::AsContext for WasmtimeContext<'_> {
    type Data = StoreState;

    fn as_context(&self) -> StoreContext<'_, Self::Data> {
        self.inner.as_context()
    }
}

impl wasmtime::AsContext for WasmtimeContextMut<'_> {
    type Data = StoreState;

    fn as_context(&self) -> StoreContext<'_, Self::Data> {
        self.inner.as_context()
    }
}

impl wasmtime::AsContextMut for WasmtimeContextMut<'_> {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, Self::Data> {
        self.inner.as_context_mut()
    }
}
