use std::collections::hash_map::Entry;
use std::collections::HashMap;
use marine_wasm_backend_traits::prelude::*;
use crate::{JsFunction, JsWasmBackend};

use maplit::hashmap;
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct JsImports {
    inner: HashMap<String, HashMap<String, JsFunction>>
}

impl JsImports {
    pub(crate) fn as_js_object(&self) -> js_sys::Object {
        let obj = js_sys::Object::new();
        for (module_name, namespace) in &self.inner {
            let namespace_obj = js_sys::Object::new();
            for (func_name, func) in namespace {
                js_sys::Reflect::set(
                    &namespace_obj,
                    &func_name.into(),
                    &func.inner,
                )
                    .map_err(|e| {
                        web_sys::console::log_1(&e);
                    })
                    .unwrap();
            }

            js_sys::Reflect::set(&obj, &module_name.into(), &namespace_obj)
                .map_err(|e| {
                    web_sys::console::log_1(&e);
                })
                .unwrap();
        }

        obj
    }
}


impl Imports<JsWasmBackend> for JsImports {
    fn new(store: &mut <JsWasmBackend as WasmBackend>::Store) -> Self {
        Self {
            inner: <_>::default()
        }
    }

    fn insert(
        &mut self,
        store: &impl AsContext<JsWasmBackend>,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <JsWasmBackend as WasmBackend>::Function,
    ) -> Result<(), ImportError> {
        // todo refactor without inner match
        let module_name = module.into();
        let func_name = name.into();
        match self.inner.entry(module_name.clone()) {
            Entry::Occupied(mut map) => {
                match map.get_mut().entry(func_name.clone()) {
                    Entry::Occupied(_) => Err(ImportError::DuplicateImport(module_name, func_name)),
                    Entry::Vacant(entry) => {
                        entry.insert(func);
                        Ok(())
                    }
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(hashmap! {
                    func_name => func
                });

                Ok(())
            }
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
            self.insert(store, module_name.clone(), func_name, func)?
        }

        Ok(())
    }
}
