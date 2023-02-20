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

use crate::{WasmerBackend, WasmerImports, WasmerInstance, WasmerStore};

use marine_wasm_backend_traits::*;

use anyhow::anyhow;
use multimap::MultiMap;

pub struct WasmerModule {
    pub(crate) inner: wasmer::Module,
    pub(crate) custom_sections: MultiMap<String, Vec<u8>>,
}

impl Module<WasmerBackend> for WasmerModule {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]> {
        self.custom_sections
            .get_vec(key)
            .map(|value| value.as_slice())
    }
    fn instantiate(
        &self,
        store: &mut WasmerStore,
        imports: &WasmerImports,
    ) -> InstantiationResult<<WasmerBackend as WasmBackend>::Instance> {
        wasmer::Instance::new(&mut store.inner, &self.inner, &imports.inner)
            .map_err(|e| InstantiationError::Other(anyhow!(e))) // TODO make detailed
            .map(|instance| WasmerInstance { inner: instance })
    }
}
