use std::marker::PhantomData;
use marine_wasm_backend_traits::prelude::*;

use crate::JsWasmBackend;

#[derive(Default, Clone)]
pub struct JsStore {}

#[derive(Default, Clone)]
pub struct JsContext<'c> {
    _data: PhantomData<&'c i32>,
}

impl Store<JsWasmBackend> for JsStore {
    fn new(backend: &JsWasmBackend) -> Self {
        log::debug!("JsStore created");
        Self {}
    }
}

impl<'c> Context<JsWasmBackend> for JsContext<'c> {}

impl<'c> ContextMut<JsWasmBackend> for JsContext<'c> {}

impl AsContext<JsWasmBackend> for JsStore {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::default()
    }
}

impl AsContextMut<JsWasmBackend> for JsStore {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        JsContext::default()
    }
}

impl<'c> AsContext<JsWasmBackend> for JsContext<'c> {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::default()
    }
}

impl<'c> AsContextMut<JsWasmBackend> for JsContext<'c> {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        JsContext::default()
    }
}
