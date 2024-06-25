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

use crate::AsContextMut;
use crate::Export;
use crate::ResolveResult;
use crate::WasmBackend;

/// A handle to an instantiated Wasm module. Cloning is cheap.
pub trait Instance<WB: WasmBackend>: Clone {
    /// Returns an `Iterator` to all exports of this instance.
    fn export_iter<'a>(
        &'a self,
        store: <WB as WasmBackend>::ContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<WB>)> + 'a>;

    /// Returns nth exported memory, None if there is no nth memory.
    /// No guaranties is known for memory order, but almost always a module has only one memory,
    /// hence the only valid value for `memory_index` is 0.
    fn get_nth_memory(
        &self,
        store: &mut impl AsContextMut<WB>,
        memory_index: u32, // TODO: refactor memory indexing with enums
    ) -> Option<<WB as WasmBackend>::Memory>;

    /// Returns a memory export with given name.
    /// # Errors:
    ///     Returns an error if there is no export with such name, or it is not a memory.
    fn get_memory(
        &self,
        store: &mut impl AsContextMut<WB>,
        memory_name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::Memory>;

    /// Returns an exported function with the given name.
    /// # Errors:
    ///     Returns an error if there is no export with such name, or it is not a function.
    fn get_function(
        &self,
        store: &mut impl AsContextMut<WB>,
        name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::ExportFunction>;
}
