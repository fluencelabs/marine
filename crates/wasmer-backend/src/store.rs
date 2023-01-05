use wasmer::{AsStoreMut, AsStoreRef, Engine, EngineBuilder, StoreMut, StoreRef};
use crate::WasmerBackend;

use marine_wasm_backend_traits::*;

pub struct WasmerStore {
    pub(crate) inner: wasmer::Store,
}

pub struct WasmerContext<'s> {
    inner: wasmer::StoreRef<'s>,
}

pub struct WasmerContextMut<'s> {
    pub(crate) inner: wasmer::StoreMut<'s>,
}

impl Store<WasmerBackend> for WasmerStore {
    fn new(_backend: &WasmerBackend) -> Self {
        Self {
            inner: wasmer::Store::default(),
        }
    }
}

impl Context<WasmerBackend> for WasmerContext<'_> {}

impl ContextMut<WasmerBackend> for WasmerContextMut<'_> {}

impl AsContext<WasmerBackend> for WasmerStore {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
        }
    }
}

impl AsContextMut<WasmerBackend> for WasmerStore {
    fn as_context_mut(&mut self) -> WasmerContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut(),
        }
    }
}

impl<'c> AsContext<WasmerBackend> for WasmerContext<'c> {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
        }
    }
}

impl<'c> AsContext<WasmerBackend> for WasmerContextMut<'c> {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
        }
    }
}

impl<'c> AsContextMut<WasmerBackend> for WasmerContextMut<'c> {
    fn as_context_mut(&mut self) -> WasmerContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut(),
        }
    }
}

impl AsStoreRef for WasmerStore {
    fn as_store_ref(&self) -> StoreRef<'_> {
        self.inner.as_store_ref()
    }
}

impl<'c> AsStoreRef for WasmerContext<'c> {
    fn as_store_ref(&self) -> StoreRef<'_> {
        self.inner.as_store_ref()
    }
}

impl<'c> AsStoreRef for WasmerContextMut<'c> {
    fn as_store_ref(&self) -> StoreRef<'_> {
        self.inner.as_store_ref()
    }
}
