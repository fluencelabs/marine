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

use crate::JsStore;
use crate::JsInstance;
use crate::JsImports;
use crate::JsWasmBackend;
use crate::module_info::ModuleInfo;

use marine_wasm_backend_traits::prelude::*;

use anyhow::anyhow;
use futures::future::BoxFuture;
use futures::FutureExt;
use js_sys::WebAssembly;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;

pub struct JsModule {
    inner: WebAssembly::Module,
    module_info: ModuleInfo,
}

/// Safety: js-backend is expected to run in single-threaded environment,
/// so it is safe to assume that every type is Send + Sync
unsafe impl Send for JsModule {}
unsafe impl Sync for JsModule {}

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

    fn instantiate<'args>(
        &'args self,
        store: &'args mut JsStore,
        imports: &'args JsImports,
    ) -> BoxFuture<'args, InstantiationResult<<JsWasmBackend as WasmBackend>::Instance>> {
        async move {
            let imports_object = imports.build_import_object(store.as_context(), &self.inner);
            let instance =
                WebAssembly::Instance::new(&self.inner, &imports_object).map_err(|e| {
                    InstantiationError::Other(anyhow!("failed to instantiate: {:?}", e))
                })?;

            // adds memory to @wasmer/wasi object
            imports.bind_to_instance(store.as_context(), &instance);

            let stored_instance = JsInstance::new(
                &mut store.as_context_mut(),
                instance,
                self.module_info.clone(),
            );
            Ok(stored_instance)
        }
        .boxed()
    }
}
