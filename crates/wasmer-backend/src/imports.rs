use crate::WasmerBackend;

use marine_wasm_backend_traits::*;

#[derive(Clone)]
pub struct WasmerImports {
    inner: wasmer::Imports,
}

pub struct WasmerNamespace {}

impl Imports<WasmerBackend> for WasmerImports {
    fn new(store: &mut <WasmerBackend as WasmBackend>::Store) -> Self {
        Self {
            inner: wasmer::Imports::new()
        }
    }

    fn register<S>(
        &mut self,
        name: S,
        namespace: <WasmerBackend as WasmBackend>::Namespace,
    ) -> Option<Box<dyn LikeNamespace<WasmerBackend>>>
    where
        S: Into<String>,
    {
        self.inner.define()
    }
}
