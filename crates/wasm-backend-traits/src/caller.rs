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

use crate::AsContextMut;
use crate::FuncGetter;
use crate::WasmBackend;

/// `ImportCallContext` is a structure used to pass context to imports.
/// It serves as a handle to `Store`, and also provides access to `Memory` and export functions
/// from the caller instance, if there is one.
pub trait ImportCallContext<WB: WasmBackend>:
    FuncGetter<WB, (i32, i32), i32>
    + FuncGetter<WB, (i32, i32), ()>
    + FuncGetter<WB, i32, i32>
    + FuncGetter<WB, i32, ()>
    + FuncGetter<WB, (), i32>
    + FuncGetter<WB, (), ()>
    + AsContextMut<WB>
    + Sync
{
    /// Gets the `Memory` from the caller instance.
    /// Returns `None` if function was called directly from host.
    fn memory(&mut self, memory_index: u32) -> Option<<WB as WasmBackend>::Memory>;
}
