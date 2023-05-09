use std::borrow::Cow;
use std::collections::HashMap;
use crate::{JsImports, JsInstance, JsStore, JsWasmBackend};
use crate::module_info::ModuleInfo;

use marine_wasm_backend_traits::prelude::*;

use js_sys::WebAssembly;
use js_sys::Uint8Array;
use anyhow::anyhow;
use walrus::{ExportItem, ImportKind};
use wasm_bindgen::{JsValue, module};
use marine_wasm_backend_traits::impl_utils::MultiMap;
use web_sys::console;
use crate::instance::StoredInstance;

pub struct JsModule {
    inner: WebAssembly::Module,
    module_info: ModuleInfo,
}

impl Module<JsWasmBackend> for JsModule {
    fn new(store: &mut JsStore, wasm: &[u8]) -> ModuleCreationResult<Self> {
        log::debug!("Module::new start");
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
        let module_info = ModuleInfo::from_bytes(wasm);

        let module = Self {
            inner: module,
            module_info,
        };

        log::debug!("Module::new success");
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
        log::debug!("Module::instantiate start");
        let imports_object = imports.build_import_object(store.as_context(), &self.inner);
        let instance = WebAssembly::Instance::new(&self.inner, &imports_object).map_err(|e| {
            web_sys::console::log_1(&e);
            InstantiationError::Other(anyhow!("failed to instantiate"))
        })?;

        // adds memory to @wasmer/wasi object
        imports.bind_to_instance(store.as_context(), &instance);

        let stored_instance = JsInstance::new(
            &mut store.as_context_mut(),
            instance,
            self.module_info.clone(),
        );
        log::debug!("Module::instantiate success");
        Ok(stored_instance)
    }
}
