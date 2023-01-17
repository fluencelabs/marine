use wasmer::{AsStoreMut, AsStoreRef, Engine, EngineBuilder, StoreMut, StoreRef};
use crate::WasmerBackend;

use marine_wasm_backend_traits::*;

pub struct WasmerStore {
    pub(crate) inner: wasmer::Store,
    pub(crate) env: wasmer::FunctionEnv<StoreState>,
}

pub struct StoreState {
    pub(crate) current_memory: Option<wasmer::Memory>,
}

pub struct WasmerContext<'s> {
    pub(crate) inner: wasmer::StoreRef<'s>,
    pub(crate) env: wasmer::FunctionEnv<StoreState>,
}

pub struct WasmerContextMut<'s> {
    pub(crate) inner: wasmer::StoreMut<'s>,
    pub(crate) env: wasmer::FunctionEnv<StoreState>,
}

impl Store<WasmerBackend> for WasmerStore {
    fn new(_backend: &WasmerBackend) -> Self {
        let mut store = wasmer::Store::default();
        let env = wasmer::FunctionEnv::new(&mut store, StoreState { current_memory: None});
        Self {
            inner: store,
            env,
        }
    }
}

impl Context<WasmerBackend> for WasmerContext<'_> {}

impl ContextMut<WasmerBackend> for WasmerContextMut<'_> {}

impl AsContext<WasmerBackend> for WasmerStore {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
            env: self.env.clone()
        }
    }
}

impl AsContextMut<WasmerBackend> for WasmerStore {
    fn as_context_mut(&mut self) -> WasmerContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut(),
            env: self.env.clone()
        }
    }
}

impl<'c> AsContext<WasmerBackend> for WasmerContext<'c> {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
            env: self.env.clone()
        }
    }
}

impl<'c> AsContext<WasmerBackend> for WasmerContextMut<'c> {
    fn as_context(&self) -> WasmerContext<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
            env: self.env.clone()
        }
    }
}

impl<'c> AsContextMut<WasmerBackend> for WasmerContextMut<'c> {
    fn as_context_mut(&mut self) -> WasmerContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut(),
            env: self.env.clone()
        }
    }
}

impl<'c> AsContext<WasmerBackend> for &mut WasmerContextMut<'c> {
    fn as_context(&self) -> <WasmerBackend as WasmBackend>::Context<'_> {
        WasmerContext {
            inner: self.inner.as_store_ref(),
            env: self.env.clone()
        }
    }
}

impl<'c> AsContextMut<WasmerBackend> for &mut WasmerContextMut<'c> {
    fn as_context_mut(&mut self) -> <WasmerBackend as WasmBackend>::ContextMut<'_> {
        WasmerContextMut {
            inner: self.inner.as_store_mut(),
            env: self.env.clone()
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
