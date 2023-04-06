use marine_wasm_backend_traits::prelude::*;
use crate::JsWasmBackend;

#[derive(Clone)]
pub struct JsImports {}

impl Imports<JsWasmBackend> for JsImports {
    fn new(store: &mut <JsWasmBackend as WasmBackend>::Store) -> Self {
        Self {}
    }

    fn insert(
        &mut self,
        store: &impl AsContext<JsWasmBackend>,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <JsWasmBackend as WasmBackend>::Function,
    ) -> Result<(), ImportError> {
        todo!()
    }

    fn register<S, I>(
        &mut self,
        store: &impl AsContext<JsWasmBackend>,
        name: S,
        namespace: I,
    ) -> Result<(), ImportError>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <JsWasmBackend as WasmBackend>::Function)>,
    {
        todo!()
    }
}
