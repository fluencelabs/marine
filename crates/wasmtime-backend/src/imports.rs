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

use crate::StoreState;
use crate::WasmtimeFunction;
use crate::WasmtimeStore;
use crate::WasmtimeWasmBackend;

use marine_wasm_backend_traits::prelude::*;

#[derive(Clone)]
pub struct WasmtimeImports {
    pub(crate) linker: wasmtime::Linker<StoreState>,
}

impl Imports<WasmtimeWasmBackend> for WasmtimeImports {
    fn new(store: &mut WasmtimeStore) -> Self {
        Self {
            linker: wasmtime::Linker::new(store.inner.engine()),
        }
    }

    fn insert(
        &mut self,
        store: &impl AsContext<WasmtimeWasmBackend>,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <WasmtimeWasmBackend as WasmBackend>::HostFunction,
    ) -> Result<(), ImportError> {
        let module = module.into();
        let name = name.into();
        self.linker
            .define(store.as_context(), &module, &name, func.inner)
            .map_err(|_| ImportError::DuplicateImport(module, name))
            .map(|_| ())
    }

    fn register<S, I>(
        &mut self,
        store: &impl AsContext<WasmtimeWasmBackend>,
        name: S,
        namespace: I,
    ) -> Result<(), ImportError>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, WasmtimeFunction)>,
    {
        let module: String = name.into();
        for (name, func) in namespace {
            self.insert(store, &module, name, func)?;
        }

        Ok(())
    }
}
