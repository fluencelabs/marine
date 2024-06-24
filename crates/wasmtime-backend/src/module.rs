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

use crate::WasmtimeImports;
use crate::WasmtimeInstance;
use crate::WasmtimeStore;
use crate::WasmtimeWasmBackend;
use crate::utils::inspect_instantiation_error;

use marine_wasm_backend_traits::prelude::*;
use marine_wasm_backend_traits::impl_utils::custom_sections;

use futures::future::BoxFuture;
use futures::FutureExt;
use multimap::MultiMap;

pub struct WasmtimeModule {
    pub(crate) custom_sections: MultiMap<String, Vec<u8>>,
    pub(crate) inner: wasmtime::Module,
}

impl Module<WasmtimeWasmBackend> for WasmtimeModule {
    fn new(store: &mut WasmtimeStore, wasm: &[u8]) -> ModuleCreationResult<Self> {
        let module = wasmtime::Module::new(store.inner.engine(), wasm)
            .map_err(ModuleCreationError::FailedToCompileWasm)?;
        let custom_sections =
            custom_sections(wasm) // TODO: avoid double module parsing
                .map_err(ModuleCreationError::FailedToExtractCustomSections)?;

        Ok(WasmtimeModule {
            custom_sections,
            inner: module,
        })
    }

    fn custom_sections(&self, name: &str) -> &[Vec<u8>] {
        self.custom_sections
            .get_vec(name)
            .map(|value| value.as_slice())
            .unwrap_or_default()
    }

    fn instantiate<'args>(
        &'args self,
        store: &'args mut WasmtimeStore,
        imports: &'args WasmtimeImports,
    ) -> BoxFuture<'args, InstantiationResult<<WasmtimeWasmBackend as WasmBackend>::Instance>> {
        // linker will not call _start, or _initialize unless Linker::module or Linker::module_async is used
        async move {
            let instance = imports
                .linker
                .instantiate_async(&mut store.inner, &self.inner)
                .await
                .map_err(inspect_instantiation_error)?; // TODO add detail
            Ok(WasmtimeInstance { inner: instance })
        }
        .boxed()
    }
}
