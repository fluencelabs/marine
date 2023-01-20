use marine_wasm_backend_traits::*;
use crate::{StoreState, WasmtimeFunction, WasmtimeStore, WasmtimeWasmBackend};

// imports
#[derive(Clone)]
pub struct WasmtimeImports {
    pub(crate) linker: wasmtime::Linker<StoreState>,
}

impl Imports<WasmtimeWasmBackend> for WasmtimeImports {
    fn new(store: &mut WasmtimeStore) -> Self {
        Self {
            linker: wasmtime::Linker::new(store.inner.engine()),
        }
    }

    fn insert(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <WasmtimeWasmBackend as WasmBackend>::Function,
    ) -> Result<(), ImportError> {
        let module = module.into();
        let name = name.into();
        self.linker
            .define(&module, &name, func.inner)
            .map_err(|_| ImportError::DuplicateImport(module, name))
            .map(|_| ())
    }

    fn register<S, I>(&mut self, name: S, namespace: I) -> Result<(), ImportError>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, WasmtimeFunction)>,
    {
        let module: String = name.into();
        for (name, func) in namespace {
            self.insert(&module, name, func)?;
        }

        Ok(())
    }
}
