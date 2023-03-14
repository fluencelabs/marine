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
        func: <WasmtimeWasmBackend as WasmBackend>::Function,
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
