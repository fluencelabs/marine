use crate::{InstantiationResult, WasmBackend};

pub trait Module<WB: WasmBackend> {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]>;
    fn instantiate(
        &self,
        store: &mut <WB as WasmBackend>::Store,
        imports: &<WB as WasmBackend>::Imports,
    ) -> InstantiationResult<<WB as WasmBackend>::Instance>;
}
