use std::marker::PhantomData;
use std::rc::Rc;
use marine_wasm_backend_traits::prelude::*;
use crate::function::JsFunctionStored;

use crate::{JsInstance, JsWasmBackend};

use crate::instance::StoredInstance;
use crate::wasi::WasiContext;

pub struct JsStore {
    pub(crate) inner: Box<JsStoreInner>,
}

#[derive(Default)]
pub(crate) struct JsStoreInner {
    pub(crate) wasi_contexts: Vec<crate::wasi::WasiContext>,
    pub(crate) instances: Vec<StoredInstance>,
    pub(crate) functions: Vec<JsFunctionStored>,

    // when JsFunction::call is called from host, the instance is pushed at the beginning and popped at the end
    // this is used to provide access to the caller instance for the imports
    pub(crate) wasm_call_stack: Vec<JsInstance>,
}

impl JsStoreInner {
    pub(crate) fn store_instance(&mut self, instance: StoredInstance) -> usize {
        let index = self.instances.len();
        self.instances.push(instance);
        index
    }

    pub(crate) fn store_wasi_context(&mut self, context: WasiContext) -> usize {
        let index = self.instances.len();
        self.wasi_contexts.push(context);
        index
    }
}

#[derive(Clone)]
pub struct JsContext<'c> {
    pub(crate) inner: &'c JsStoreInner,
}

impl<'c> JsContext<'c> {
    pub(crate) fn new(inner: &'c JsStoreInner) -> Self {
        Self { inner }
    }

    pub(crate) fn from_raw_ptr(store_inner: *const JsStoreInner) -> Self {
        unsafe {
            Self {
                inner: &*store_inner,
            }
        }
    }
}

pub struct JsContextMut<'c> {
    pub(crate) inner: &'c mut JsStoreInner,
}

impl JsContextMut<'_> {
    pub(crate) fn from_raw_ptr(store_inner: *mut JsStoreInner) -> Self {
        unsafe {
            Self {
                inner: &mut *store_inner,
            }
        }
    }
}

impl<'c> JsContextMut<'c> {
    pub(crate) fn new(inner: &'c mut JsStoreInner) -> Self {
        Self { inner }
    }
}

impl Store<JsWasmBackend> for JsStore {
    fn new(_backend: &JsWasmBackend) -> Self {
        Self {
            inner: Box::new(<_>::default()),
        }
    }
}

impl<'c> Context<JsWasmBackend> for JsContext<'c> {}

impl<'c> ContextMut<JsWasmBackend> for JsContextMut<'c> {}

impl AsContext<JsWasmBackend> for JsStore {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::new(&self.inner)
    }
}

impl AsContextMut<JsWasmBackend> for JsStore {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        JsContextMut::new(&mut self.inner)
    }
}

impl<'c> AsContext<JsWasmBackend> for JsContext<'c> {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::new(&self.inner)
    }
}

impl<'c> AsContext<JsWasmBackend> for JsContextMut<'c> {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::new(self.inner)
    }
}

impl<'c> AsContextMut<JsWasmBackend> for JsContextMut<'c> {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        JsContextMut::new(&mut self.inner)
    }
}