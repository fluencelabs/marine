use marine_wasm_backend_traits::prelude::*;
use crate::JsWasmBackend;

pub struct JsWasi {}

impl WasiImplementation<JsWasmBackend> for JsWasi {
    fn register_in_linker(
        store: &mut <JsWasmBackend as WasmBackend>::ContextMut<'_>,
        linker: &mut <JsWasmBackend as WasmBackend>::Imports,
        config: WasiParameters,
    ) -> Result<(), WasiError> {
        todo!()
    }

    fn get_wasi_state<'s>(
        instance: &'s mut <JsWasmBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        todo!()
    }
}
