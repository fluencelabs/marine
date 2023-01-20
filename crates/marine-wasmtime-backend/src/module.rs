use crate::{WasmtimeImports, WasmtimeInstance, WasmtimeStore, WasmtimeWasmBackend};

use marine_wasm_backend_traits::*;

use multimap::MultiMap;
use crate::utils::inspect_instantiation_error;

pub struct WasmtimeModule {
    pub(crate) custom_sections: MultiMap<String, Vec<u8>>,
    pub(crate) inner: wasmtime::Module,
}

impl Module<WasmtimeWasmBackend> for WasmtimeModule {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]> {
        self.custom_sections
            .get_vec(key)
            .map(|value| value.as_slice())
    }

    fn instantiate(
        &self,
        store: &mut WasmtimeStore,
        imports: &WasmtimeImports,
    ) -> InstantiationResult<<WasmtimeWasmBackend as WasmBackend>::Instance> {
        let instance = imports
            .linker
            .instantiate(&mut store.inner, &self.inner)
            .map_err(inspect_instantiation_error)?; // todo add detail
        Ok(WasmtimeInstance { inner: instance })
    }
}
