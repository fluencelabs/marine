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
    ) -> ImportResult<()> {
        // todo check for existence
        self.inner.define(&module.into(), &name.into(), func.inner);
        Ok(())
    }

    fn register<S, I>(&mut self, name: S, namespace: I) -> ImportResult<()>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <WasmerBackend as WasmBackend>::Function)>,
    {
        // todo check for existence
        let namespace = namespace
            .into_iter()
            .map(|(name, func)| (name, Extern::Function(func.inner)));

        self.inner.register_namespace(&name.into(), namespace);

        Ok(())
    }
}
/*
impl InsertFn<WasmerBackend, (i32, i32), i32> for WasmerImports {
    fn insert_fn<F>(func: F)
        where F: Fn(&mut <WasmerBackend as WasmBackend>::Caller<'_>, (i32, i32)) -> i32 + Sync + Send + 'static {

    }
}*/
