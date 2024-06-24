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

use crate::ModuleCreationResult;
use crate::InstantiationResult;
use crate::WasmBackend;

use futures::future::BoxFuture;

/// A handle to compiled wasm module.
pub trait Module<WB: WasmBackend>: Sized {
    /// Compiles a wasm bytes into a module and extracts custom sections.
    fn new(store: &mut <WB as WasmBackend>::Store, wasm: &[u8]) -> ModuleCreationResult<Self>;

    /// Returns custom sections corresponding to `name`, empty slice if there is no sections.
    fn custom_sections(&self, name: &str) -> &[Vec<u8>];

    /// Instantiates module by allocating memory, VM state and linking imports with ones from `import` argument.
    /// Does not call `_start` or `_initialize` functions.
    ///
    /// # Panics:
    ///
    ///     If the `Store` given is not the same with `Store` used to create `Imports` and this object.
    fn instantiate<'args>(
        &'args self,
        store: &'args mut <WB as WasmBackend>::Store,
        imports: &'args <WB as WasmBackend>::Imports,
    ) -> BoxFuture<'args, InstantiationResult<<WB as WasmBackend>::Instance>>;
}
