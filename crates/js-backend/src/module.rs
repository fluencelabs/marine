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

use crate::JsStore;
use crate::JsInstance;
use crate::JsImports;
use crate::JsWasmBackend;
use crate::module_info::ModuleInfo;

use marine_wasm_backend_traits::prelude::*;

use anyhow::anyhow;
use js_sys::WebAssembly;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;

pub struct JsModule {
    inner: WebAssembly::Module,
    module_info: ModuleInfo,
}

impl Module<JsWasmBackend> for JsModule {
    fn new(_store: &mut JsStore, wasm: &[u8]) -> ModuleCreationResult<Self> {
        let data = Uint8Array::new_with_length(wasm.len() as u32);
        data.copy_from(wasm);
        let data_obj: JsValue = data.into();
        let module = WebAssembly::Module::new(&data_obj).map_err(|e| {
            log::debug!("Module::new failed: {:?}", e);
            ModuleCreationError::FailedToCompileWasm(anyhow!(format!(
                "error compiling module: {:?}",
                e
            )))
        })?;

        // JS WebAssembly module does not provide info about export signatures,
        // so this data is extracted from wasm in control module.
        let module_info = ModuleInfo::from_bytes(wasm)?;

        let module = Self {
            inner: module,
            module_info,
        };

        Ok(module)
    }

    fn custom_sections(&self, name: &str) -> &[Vec<u8>] {
        match self.module_info.custom_sections.get_vec(name) {
            None => &[],
            Some(data) => data,
        }
    }

    fn instantiate(
        &self,
        store: &mut JsStore,
        imports: &JsImports,
    ) -> InstantiationResult<<JsWasmBackend as WasmBackend>::Instance> {
        let imports_object = imports.build_import_object(store.as_context(), &self.inner);

        let instance = WebAssembly::Instance::new(&self.inner, &imports_object)
            .map_err(|e| InstantiationError::Other(anyhow!("failed to instantiate: {:?}", e)))?;

        // adds memory to @wasmer/wasi object
        imports.bind_to_instance(store.as_context(), &instance);

        let stored_instance = JsInstance::new(
            &mut store.as_context_mut(),
            instance,
            self.module_info.clone(),
        );
        Ok(stored_instance)
    }
}
