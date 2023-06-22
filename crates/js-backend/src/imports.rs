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

use crate::JsFunction;
use crate::JsWasmBackend;

use marine_wasm_backend_traits::prelude::*;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Clone)]
pub struct JsImports {
    inner: HashMap<String, HashMap<String, JsFunction>>,

    /// JS backend uses WASI imports directly from JS, so it needs special handling.
    wasi_ctx: Option<usize>,
}

impl JsImports {
    pub(crate) fn build_import_object(
        &self,
        store: impl AsContext<JsWasmBackend>,
        module: &js_sys::WebAssembly::Module,
    ) -> js_sys::Object {
        let import_object = self
            .wasi_ctx
            .map(|idx| store.as_context().inner.wasi_contexts[idx].get_imports(module))
            .unwrap_or_else(js_sys::Object::new);

        for (module_name, namespace) in &self.inner {
            let namespace_obj = js_sys::Object::new();
            for (func_name, func) in namespace {
                js_sys::Reflect::set(
                    &namespace_obj,
                    &func_name.into(),
                    &func.stored(&store.as_context()).js_func,
                )
                .map_err(|e| {
                    web_sys::console::log_1(&e);
                })
                .unwrap(); // Safety: it looks like it fires only if the first argument is not an Object.
            }

            js_sys::Reflect::set(&import_object, &module_name.into(), &namespace_obj)
                .map_err(|e| {
                    web_sys::console::log_1(&e);
                })
                .unwrap(); // Safety: it looks like it fires only if the first argument is not an Object.
        }

        import_object
    }

    pub(crate) fn add_wasi(&mut self, wasi_context_id: usize) {
        self.wasi_ctx = Some(wasi_context_id)
    }

    /// Adds memory to @wasmer/wasi object
    pub(crate) fn bind_to_instance(
        &self,
        store: impl AsContext<JsWasmBackend>,
        instance: &js_sys::WebAssembly::Instance,
    ) {
        if let Some(handle) = &self.wasi_ctx {
            store.as_context().inner.wasi_contexts[*handle].bind_to_instance(instance);
        }
    }

    fn get_namespace(&mut self, module_name: String) -> &mut HashMap<String, JsFunction> {
        self.inner
            .entry(module_name.clone())
            .or_insert(<_>::default())
    }
}

impl Imports<JsWasmBackend> for JsImports {
    fn new(_store: &mut <JsWasmBackend as WasmBackend>::Store) -> Self {
        Self {
            inner: <_>::default(),
            wasi_ctx: None,
        }
    }

    fn insert(
        &mut self,
        _store: &impl AsContext<JsWasmBackend>,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <JsWasmBackend as WasmBackend>::Function,
    ) -> Result<(), ImportError> {
        let module_name = module.into();
        let func_name = name.into();

        let namespace = self.get_namespace(module_name.clone());
        add_to_namespace(namespace, func_name, func, &module_name)
    }

    fn register<S, I>(
        &mut self,
        _store: &impl AsContext<JsWasmBackend>,
        module_name: S,
        functions: I,
    ) -> Result<(), ImportError>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <JsWasmBackend as WasmBackend>::Function)>,
    {
        let module_name = module_name.into();
        let namespace = self.get_namespace(module_name.clone());
        for (func_name, func) in functions {
            add_to_namespace(namespace, func_name, func, &module_name)?;
        }

        Ok(())
    }
}

fn add_to_namespace(
    namespace: &mut HashMap<String, JsFunction>,
    func_name: String,
    func: JsFunction,
    module_name: &str,
) -> Result<(), ImportError> {
    match namespace.entry(func_name) {
        Entry::Occupied(entry) => Err(ImportError::DuplicateImport(
            module_name.to_string(),
            entry.key().clone(),
        )),
        Entry::Vacant(entry) => {
            entry.insert(func);
            Ok(())
        }
    }
}
