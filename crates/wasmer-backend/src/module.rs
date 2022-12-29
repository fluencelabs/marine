use crate::{WasmerBackend, WasmerImports, WasmerStore};

use marine_wasm_backend_traits::*;

pub struct WasmerModule {
    pub(crate) inner: wasmer::Module,
}

impl Module<WasmerBackend> for WasmerModule {
    fn custom_sections(&self, key: &str) -> Vec<&[u8]> {
        self.inner
            .custom_sections(key)
            .map(|section| section.as_ref())
            .collect::<Vec<&[u8]>>()
    }

    fn instantiate(
        &self,
        store: &mut WasmerStore,
        imports: &WasmerImports,
    ) -> WasmBackendResult<<WasmerBackend as WasmBackend>::Instance> {
        todo!()
    }
}
