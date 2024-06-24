/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

pub static STANDARD_MEMORY_EXPORT_NAME: &str = "memory";
pub static STANDARD_MEMORY_INDEX: u32 = 0;

use crate::DelayedContextLifetime;
use crate::WasmBackend;

/// Contains Wasm exports necessary for internal usage.
#[derive(Clone)]
pub enum Export<WB: WasmBackend> {
    Memory(<WB as WasmBackend>::Memory),
    Function(<WB as WasmBackend>::ExportFunction),
    Other,
}

// TODO: add read/write/etc methods to the `Memory` trait,
// and then make a generic implementation of interface-types traits
/// A wasm memory handle.
/// As it is only a handle to an object in `Store`, cloning is cheap.
pub trait Memory<WB: WasmBackend>:
    it_memory_traits::Memory<<WB as WasmBackend>::MemoryView, DelayedContextLifetime<WB>>
    + Clone
    + Send
    + Sync
    + 'static
{
    /// Get the size of the allocated memory in bytes.
    fn size(&self, store: &mut <WB as WasmBackend>::ContextMut<'_>) -> usize;
}
