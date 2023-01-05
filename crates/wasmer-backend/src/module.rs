use multimap::MultiMap;
use crate::{WasmerBackend, WasmerImports, WasmerStore};

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
        todo!()
    }
}
