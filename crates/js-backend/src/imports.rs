/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::HostImportFunction;
use crate::JsWasmBackend;
use crate::store::WasiContextHandle;

use marine_wasm_backend_traits::prelude::*;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Clone)]
pub struct JsImports {
    inner: HashMap<String, HashMap<String, HostImportFunction>>,

    /// JS backend uses WASI imports directly from JS, so it needs special handling.
    wasi_ctx: Option<WasiContextHandle>,
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
            .unwrap_or_default();

        for (module_name, namespace) in &self.inner {
            let namespace_obj = js_sys::Object::new();
            for (func_name, func) in namespace {
                js_sys::Reflect::set(
                    &namespace_obj,
                    &func_name.into(),
                    &func.stored(&store.as_context()).js_function,
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

    pub(crate) fn add_wasi(&mut self, wasi_context_id: WasiContextHandle) {
        self.wasi_ctx = Some(wasi_context_id)
    }

    /// Adds memory to @wasmer/wasi object
    pub(crate) fn bind_to_instance(
        &self,
        store: impl AsContext<JsWasmBackend>,
        instance: &js_sys::WebAssembly::Instance,
    ) {
        if let Some(handle) = self.wasi_ctx {
            store.as_context().inner.wasi_contexts[handle].bind_to_instance(instance);
        }
    }

    fn get_namespace(&mut self, module_name: String) -> &mut HashMap<String, HostImportFunction> {
        self.inner.entry(module_name).or_default()
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
        func: <JsWasmBackend as WasmBackend>::HostFunction,
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
        I: IntoIterator<Item = (String, <JsWasmBackend as WasmBackend>::HostFunction)>,
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
    namespace: &mut HashMap<String, HostImportFunction>,
    func_name: String,
    func: HostImportFunction,
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
