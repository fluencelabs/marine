use multimap::MultiMap;
use crate::{WasmerBackend, WasmerImports, WasmerInstance, WasmerStore};

use marine_wasm_backend_traits::*;

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
    ) -> WasmBackendResult<<WasmerBackend as WasmBackend>::Instance> {
        wasmer::Instance::new(&mut store.inner, &self.inner, &imports.inner)
            .map_err(|e| WasmBackendError::InstantiationError(format!("{}", e)))
            .map(|instance| WasmerInstance { inner: instance })
    }
}
