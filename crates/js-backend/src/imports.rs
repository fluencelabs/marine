use std::collections::hash_map::Entry;
use std::collections::HashMap;

use marine_wasm_backend_traits::prelude::*;

use crate::JsFunction;
use crate::JsWasmBackend;

#[derive(Clone)]
pub struct JsImports {
    inner: HashMap<String, HashMap<String, JsFunction>>,
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
                .unwrap(); // TODO: research when it can return error. So far there is no info in documentation about return value.
            }

            js_sys::Reflect::set(&import_object, &module_name.into(), &namespace_obj)
                .map_err(|e| {
                    web_sys::console::log_1(&e);
                })
                .unwrap(); // TODO: research when it can return error. So far there is no info in documentation about return value.
        }

        import_object
    }

    pub(crate) fn add_wasi(&mut self, wasi_context_id: usize) {
        self.wasi_ctx = Some(wasi_context_id)
    }

    pub(crate) fn bind_to_instance(
        &self,
        store: impl AsContext<JsWasmBackend>,
        instance: &js_sys::WebAssembly::Instance,
    ) {
        if let Some(handle) = &self.wasi_ctx {
            store.as_context().inner.wasi_contexts[*handle].bind_to_instance(instance);
        }
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

        let add_func = |namespace: &mut HashMap<String, JsFunction>| -> Result<(), ImportError> {
            if let Entry::Vacant(entry) = namespace.entry(func_name.clone()) {
                entry.insert(func);
                Ok(())
            } else {
                Err(ImportError::DuplicateImport(module_name.clone(), func_name))
            }
        };

        match self.inner.entry(module_name.clone()) {
            Entry::Occupied(mut entry) => add_func(entry.get_mut()),
            Entry::Vacant(entry) => add_func(entry.insert(HashMap::new())),
        }
    }

    fn register<S, I>(
        &mut self,
        store: &impl AsContext<JsWasmBackend>,
        name: S,
        namespace: I,
    ) -> Result<(), ImportError>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <JsWasmBackend as WasmBackend>::Function)>,
    {
        let module_name = name.into();
        for (func_name, func) in namespace {
            // TODO: maybe rewrite without extensive cloning
            self.insert(store, module_name.clone(), func_name, func)?
        }

        Ok(())
    }
}
