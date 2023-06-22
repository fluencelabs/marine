/*
 * Copyright 2023 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::function::StoredFunction;
use crate::JsInstance;
use crate::JsWasmBackend;
use crate::instance::StoredInstance;
use crate::wasi::WasiContext;

use marine_wasm_backend_traits::prelude::*;

pub struct JsStore {
    pub(crate) inner: Box<JsStoreInner>,
}

#[derive(Default)]
pub(crate) struct JsStoreInner {
    pub(crate) wasi_contexts: Vec<WasiContext>,
    pub(crate) instances: Vec<StoredInstance>,
    pub(crate) functions: Vec<StoredFunction>,

    /// Imports provided to the ImportObject do not know the instance they will be bound to,
    /// so they need to get the instance handle somehow during the call.
    /// When JsFunction::call is called from host, the corresponding instance is pushed to stack
    /// at the start of the call, and removed at the end of the call.
    /// This way imports can get the caller instance from the Store.
    pub(crate) wasm_call_stack: Vec<JsInstance>,
}

impl JsStoreInner {
    pub(crate) fn store_instance(&mut self, instance: StoredInstance) -> usize {
        let index = self.instances.len();
        self.instances.push(instance);
        index
    }

    pub(crate) fn store_wasi_context(&mut self, context: WasiContext) -> usize {
        let index = self.wasi_contexts.len();
        self.wasi_contexts.push(context);
        index
    }

    pub(crate) fn store_function(&mut self, function: StoredFunction) -> usize {
        let index = self.functions.len();
        self.functions.push(function);
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

    /// Safety: wasm backend traits require that Store outlives everything created using it,
    /// so this function should be called only when Store is alive.
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
            inner: <_>::default(),
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
        JsContext::new(self.inner)
    }
}

impl<'c> AsContext<JsWasmBackend> for JsContextMut<'c> {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::new(self.inner)
    }
}

impl<'c> AsContextMut<JsWasmBackend> for JsContextMut<'c> {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        JsContextMut::new(self.inner)
    }
}
