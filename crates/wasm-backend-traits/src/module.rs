use crate::{Store, WasmBackend, WasmBackendResult};

pub trait Module<WB: WasmBackend> {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]>;
    fn instantiate(
        &self,
        store: &mut <WB as WasmBackend>::Store,
        imports: &<WB as WasmBackend>::Imports,
    ) -> WasmBackendResult<<WB as WasmBackend>::Instance>;
}
