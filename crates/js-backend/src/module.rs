use crate::{JsImports, JsStore, JsWasmBackend};

use marine_wasm_backend_traits::prelude::*;

use js_sys::WebAssembly;
use js_sys::Uint8Array;
use anyhow::anyhow;
use marine_wasm_backend_traits::impl_utils::MultiMap;

pub struct JsModule {
    inner: WebAssembly::Module,
    custom_sections: MultiMap<String, Vec<u8>>,
}

impl Module<JsWasmBackend> for JsModule {
    fn new(store: &mut JsStore, wasm: &[u8]) -> ModuleCreationResult<Self> {
        let js_wasm_bytes = unsafe { Uint8Array::view(wasm) };
        let module = WebAssembly::Module::new(&js_wasm_bytes.into()).map_err(|e| {
            ModuleCreationError::FailedToCompileWasm(anyhow!(format!(
                "error compiling module: {:?}",
                e
            )))
        })?;
        let custom_sections = marine_wasm_backend_traits::impl_utils::custom_sections(wasm)
            .map_err(|e| ModuleCreationError::FailedToExtractCustomSections(e))?;
        let module = Self {
            inner: module,
            custom_sections,
        };

        Ok(module)
    }

    fn custom_sections(&self, name: &str) -> &[Vec<u8>] {
        match self.custom_sections.get_vec(name) {
            None => &[],
            Some(data) => data,
        }
    }

    fn instantiate(
        &self,
        store: &mut JsStore,
        imports: &JsImports,
    ) -> InstantiationResult<<JsWasmBackend as WasmBackend>::Instance> {
        todo!()
    }
}
