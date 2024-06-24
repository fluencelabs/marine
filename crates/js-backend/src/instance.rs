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

use crate::JsMemory;
use crate::WasmExportFunction;
use crate::JsContextMut;
use crate::JsWasmBackend;
use crate::module_info;
use crate::module_info::ModuleInfo;
use crate::store::InstanceHandle;

use marine_wasm_backend_traits::prelude::*;

use js_sys::WebAssembly;
use js_sys::Object as JsObject;

use std::collections::HashMap;

#[derive(Clone)]
pub struct JsInstance {
    store_handle: InstanceHandle,
}

impl JsInstance {
    pub(crate) fn new(
        ctx: &mut JsContextMut<'_>,
        js_instance: WebAssembly::Instance,
        module_info: ModuleInfo,
    ) -> Self {
        let stored_instance = StoredInstance {
            inner: js_instance,
            exports: HashMap::default(),
        };

        let store_handle = ctx.inner.store_instance(stored_instance);
        let instance = Self::from_store_handle(store_handle);
        let js_exports = instance
            .stored_instance(ctx.as_context_mut())
            .inner
            .exports();
        let exports = Self::build_export_map(
            instance.clone(),
            ctx.as_context_mut(),
            module_info.exports.iter(),
            js_exports,
        );
        instance.stored_instance(ctx.as_context_mut()).exports = exports;

        instance
    }

    pub(crate) fn from_store_handle(store_handle: InstanceHandle) -> Self {
        Self { store_handle }
    }

    fn stored_instance<'store>(&self, ctx: JsContextMut<'store>) -> &'store mut StoredInstance {
        &mut ctx.inner.instances[self.store_handle]
    }

    fn build_export_map<'names, 'store>(
        instance: JsInstance,
        mut ctx: JsContextMut<'store>,
        module_exports: impl Iterator<Item = (&'names String, &'names crate::module_info::Export)>,
        js_exports: JsObject,
    ) -> HashMap<String, Export<JsWasmBackend>> {
        module_exports
            .map(|(name, export)| {
                // Safety: all used names are results of wasm imports parsing,
                // so there will always be an import for the name and the type will be correct
                let js_export = js_sys::Reflect::get(js_exports.as_ref(), &name.into()).unwrap();
                let export: Export<JsWasmBackend> = match export {
                    module_info::Export::Function(signature) => {
                        Export::Function(WasmExportFunction::new_stored(
                            &mut ctx,
                            instance.clone(),
                            js_export.into(),
                            signature.clone(),
                        ))
                    }
                    module_info::Export::Memory => Export::Memory(JsMemory::new(js_export.into())),
                    module_info::Export::Table => Export::Other,
                    module_info::Export::Global => Export::Other,
                };

                (name.clone(), export)
            })
            .collect::<HashMap<String, Export<JsWasmBackend>>>()
    }
}

/// Allocated instance resources.
pub(crate) struct StoredInstance {
    #[allow(unused)] // Keep the instance, so it wont get dropped
    pub(crate) inner: WebAssembly::Instance,
    pub(crate) exports: HashMap<String, Export<JsWasmBackend>>,
}

impl Instance<JsWasmBackend> for JsInstance {
    fn export_iter<'a>(
        &'a self,
        store: <JsWasmBackend as WasmBackend>::ContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<JsWasmBackend>)> + 'a> {
        let stored_instance = self.stored_instance(store);

        let iter = stored_instance
            .exports
            .iter()
            .map(|(name, export)| (name.as_str(), export.clone()));

        Box::new(iter)
    }

    fn get_nth_memory(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        memory_index: u32,
    ) -> Option<<JsWasmBackend as WasmBackend>::Memory> {
        let stored_instance = self.stored_instance(store.as_context_mut());
        stored_instance
            .exports
            .iter()
            .filter_map(|(_, export)| match export {
                Export::Memory(memory) => Some(memory.clone()),
                _ => None,
            })
            .nth(memory_index as usize)
    }

    fn get_memory(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        memory_name: &str,
    ) -> ResolveResult<<JsWasmBackend as WasmBackend>::Memory> {
        log::trace!(
            "Instance::get_memory, instance_id: {:?}, memory_name: {}",
            self.store_handle,
            memory_name
        );
        let stored_instance = self.stored_instance(store.as_context_mut());
        let export = stored_instance
            .exports
            .get(memory_name)
            .ok_or_else(|| ResolveError::ExportNotFound(memory_name.to_string()))?;

        match export {
            Export::Memory(memory) => Ok(memory.clone()),
            Export::Function(_) => Err(ResolveError::ExportTypeMismatch {
                expected: "memory",
                actual: "function",
            }),
            Export::Other => Err(ResolveError::ExportTypeMismatch {
                expected: "memory",
                actual: "other (funcref or externref)",
            }),
        }
    }

    fn get_function(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        name: &str,
    ) -> ResolveResult<<JsWasmBackend as WasmBackend>::ExportFunction> {
        let stored_instance = self.stored_instance(store.as_context_mut());
        let export = stored_instance
            .exports
            .get(name)
            .ok_or_else(|| ResolveError::ExportNotFound(name.to_string()))?;

        match export {
            Export::Function(func) => Ok(func.clone()),
            Export::Memory(_) => Err(ResolveError::ExportTypeMismatch {
                expected: "function",
                actual: "memory",
            }),
            Export::Other => Err(ResolveError::ExportTypeMismatch {
                expected: "function",
                actual: "other(funcref or externref)",
            }),
        }
    }
}
