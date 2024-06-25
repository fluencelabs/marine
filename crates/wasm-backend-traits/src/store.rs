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

use crate::WasmBackend;

/// `Store` is an object that stores modules, instances, functions memories and so on.
/// `Store` is grow-only: once something added, it will not be removed until Store is destroyed.
/// Some of the implementations can limit allocated resources.
/// For example, Wasmtime cannot have more than 10000 instances in one `Store`.
///
/// Most of the functions in this crate require a handle to `Store` to work.
pub trait Store<WB: WasmBackend>: AsContextMut<WB> {
    fn new(backend: &WB) -> Self;

    // TODO: create general/backend-specific core config when new parameters are needed
    fn set_total_memory_limit(&mut self, total_memory_limit: u64);

    fn report_memory_allocation_stats(&self) -> Option<MemoryAllocationStats>;

    fn clear_allocation_stats(&mut self);
}

/// A temporary immutable handle to store
pub trait Context<WB: WasmBackend>: AsContext<WB> + Send {}

/// A temporary mutable handle to store
pub trait ContextMut<WB: WasmBackend>: AsContextMut<WB> + Send {}

pub trait AsContext<WB: WasmBackend>: Send {
    fn as_context(&self) -> <WB as WasmBackend>::Context<'_>;
}

pub trait AsContextMut<WB: WasmBackend>: AsContext<WB> {
    fn as_context_mut(&mut self) -> <WB as WasmBackend>::ContextMut<'_>;
}

#[derive(Default, Clone, Debug)]
pub struct MemoryAllocationStats {
    pub allocation_rejects: u32,
}
