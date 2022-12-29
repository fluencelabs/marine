use wasmer::{AsStoreMut, AsStoreRef};
use crate::WasmerBackend;

use marine_wasm_backend_traits::*;

pub struct WasmerStore {
    pub(crate) inner: wasmer::Store
}

pub struct WasmerContext<'s> {
    inner: wasmer::StoreRef<'s>
}

pub struct WasmerContextMut<'s> {
    inner: wasmer::StoreMut<'s>
}

impl Store<WasmerBackend> for WasmerStore {
    fn new(backend: &WasmerBackend) -> Self {
        Self {
            inner: wasmer::Store::new(&backend.engine)
        }
    }
}

impl Context<WasmerBackend> for WasmerContext<'_> {}

impl ContextMut<WasmerBackend> for WasmerContextMut<'_> {}

impl AsContext<WasmerBackend> for WasmerStore {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref()
        }
    }
}

impl AsContextMut<WasmerBackend> for WasmerStore {
    fn as_context_mut(&mut self) -> WasmerContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut()
        }
    }
}

impl<'c> AsContext<WasmerBackend> for WasmerContext<'c> {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref()
        }
    }
}

impl<'c> AsContext<WasmerBackend> for WasmerContextMut<'c> {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref()
        }
    }
}

impl<'c> AsContextMut<WasmerBackend> for WasmerContextMut<'c> {
    fn as_context_mut(&mut self) -> WasmerContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut()
        }
    }
}