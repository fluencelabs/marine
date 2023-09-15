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
pub(crate) mod js_imports;

use crate::JsContextMut;
use crate::JsWasmBackend;

use marine_wasm_backend_traits::prelude::*;

use anyhow::anyhow;
use wasm_bindgen::JsValue;

use std::collections::HashMap;

pub struct JsWasi {}

impl WasiImplementation<JsWasmBackend> for JsWasi {
    fn register_in_linker(
        store: &mut JsContextMut<'_>,
        linker: &mut <JsWasmBackend as WasmBackend>::Imports,
        config: WasiParameters,
    ) -> Result<(), WasiError> {
        let context_index = store
            .inner
            .store_wasi_context(WasiContext::new(config.envs)?);
        linker.add_wasi(context_index);

        Ok(())
    }

    fn get_wasi_state<'s>(
        _instance: &'s mut <JsWasmBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        Box::new(JsWasiState {})
    }
}

pub struct JsWasiState {}

impl WasiState for JsWasiState {
    fn envs(&self) -> &[Vec<u8>] {
        &[]
    }
}

pub(crate) struct WasiContext {
    wasi_impl: JsValue,
}

impl WasiContext {
    pub(crate) fn new(envs: HashMap<String, String>) -> Result<Self, WasiError> {
        let envs_js = serde_wasm_bindgen::to_value(&envs)
            .map_err(|e| WasiError::EngineWasiError(anyhow!(e.to_string())))?;

        Ok(Self {
            wasi_impl: js_imports::create_wasi(envs_js),
        })
    }

    pub(crate) fn get_imports(&self, module: &js_sys::WebAssembly::Module) -> js_sys::Object {
        js_imports::generate_wasi_imports(module, &self.wasi_impl).into()
    }

    pub(crate) fn bind_to_instance(&self, instance: &js_sys::WebAssembly::Instance) {
        js_imports::bind_to_instance(&self.wasi_impl, instance)
    }
}
