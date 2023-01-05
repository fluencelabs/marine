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
            inner: wasmer::Imports::new(),
        }
    }

    fn insert(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <WasmerBackend as WasmBackend>::Function,
    ) {
        todo!()
    }

    fn register<S, I>(&mut self, name: S, namespace: I)
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <WasmerBackend as WasmBackend>::Function)>,
    {
        todo!()
    }
}
