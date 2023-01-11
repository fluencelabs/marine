use wasmer::Extern;
use crate::WasmerBackend;

use marine_wasm_backend_traits::*;

#[derive(Clone)]
pub struct WasmerImports {
    pub(crate) inner: wasmer::Imports,
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
        self.inner.define(&module.into(), &name.into(), func.inner);
    }

    fn register<S, I>(&mut self, name: S, namespace: I)
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <WasmerBackend as WasmBackend>::Function)>,
    {
        let namespace = namespace
            .into_iter()
            .map(|(name, func)| (name, Extern::Function(func.inner)));

        self.inner.register_namespace(&name.into(),namespace);
    }
}
/*
impl InsertFn<WasmerBackend, (i32, i32), i32> for WasmerImports {
    fn insert_fn<F>(func: F)
        where F: Fn(&mut <WasmerBackend as WasmBackend>::Caller<'_>, (i32, i32)) -> i32 + Sync + Send + 'static {

    }
}*/