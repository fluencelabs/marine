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

use crate::WasmtimeImports;
use crate::WasmtimeInstance;
use crate::WasmtimeStore;
use crate::WasmtimeWasmBackend;
use crate::utils::inspect_instantiation_error;

use marine_wasm_backend_traits::prelude::*;
use marine_wasm_backend_traits::impl_utils::custom_sections;

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

    fn custom_sections(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.custom_sections
            .get_vec(name)
            .map(|value| value.as_slice())
    }

    fn instantiate(
        &self,
        store: &mut WasmtimeStore,
        imports: &WasmtimeImports,
    ) -> InstantiationResult<<WasmtimeWasmBackend as WasmBackend>::Instance> {
        // linker will not call _start, or _initialize unless Linker::module or Linker::module_async is used
        let instance = imports
            .linker
            .instantiate(&mut store.inner, &self.inner)
            .map_err(inspect_instantiation_error)?; // TODO add detail
        Ok(WasmtimeInstance { inner: instance })
    }
}
