use crate::{JsFunction, JsMemory, JsWasmBackend};
use crate::module_info;
use crate::module_info::ModuleInfo;

use js_sys::WebAssembly;
use marine_wasm_backend_traits::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct JsInstance {
    pub(crate) inner: WebAssembly::Instance,
    pub(crate) module_info: ModuleInfo,
}

impl Instance<JsWasmBackend> for JsInstance {
    fn export_iter<'a>(
        &'a self,
        store: <JsWasmBackend as WasmBackend>::ContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<JsWasmBackend>)> + 'a> {
        log::debug!("Instance::export_iter success");
        let js_exports = self.inner.exports();
        let iter = self
            .module_info
            .exports
            .iter()
            .map(move |(name, export)| {
                let export: Export<JsWasmBackend> = match export {
                    module_info::Export::Function(sig) => {
                        let func = js_sys::Reflect::get(js_exports.as_ref(), &name.into())
                            .unwrap();

                        Export::Function(JsFunction::from_js(sig.clone(), func))
                    }
                    module_info::Export::Memory => {
                        let memory = js_sys::Reflect::get(js_exports.as_ref(), &name.into())
                            .unwrap();

                        Export::Memory(JsMemory::try_from_js(memory).unwrap())
                    }
                    module_info::Export::Table => Export::Other,
                    module_info::Export::Global => Export::Other,
                };

                (name.as_str(), export)
            });

        Box::new(iter)
    }

    fn get_nth_memory(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        _memory_index: u32,
    ) -> Option<<JsWasmBackend as WasmBackend>::Memory> {
        // TODO use index
        // TODO handle errors
        log::debug!("Instance::get_nth_memory start");
        let exports = self.inner.exports();
        let mem = js_sys::Reflect::get(exports.as_ref(), &"memory".into()).unwrap()
            .dyn_into::<WebAssembly::Memory>()
            .expect("memory export wasn't a `WebAssembly.Memory`");
        log::debug!("Instance::get_nth_memory success");
        Some(JsMemory {
            inner: mem,
        })
    }

    fn get_memory(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        memory_name: &str,
    ) -> ResolveResult<<JsWasmBackend as WasmBackend>::Memory> {
        // TODO handle errors
        log::debug!("Instance::get_memory start");
        let exports = self.inner.exports();
        let memory = js_sys::Reflect::get(exports.as_ref(), &memory_name.into()).unwrap();
        let memory = JsMemory::try_from_js(memory).unwrap();

        log::debug!("Instance::get_memory success");
        Ok(memory)
    }

    fn get_function(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        name: &str,
    ) -> ResolveResult<<JsWasmBackend as WasmBackend>::Function> {
        log::debug!("Instance::get_function start");
        // TODO handle errors
        let exports = self.inner.exports();
        let func = js_sys::Reflect::get(exports.as_ref(), &name.into()).unwrap();
        let sig= self.module_info
            .exports
            .get(name)
            .and_then(|export|
                if let crate::module_info::Export::Function(sig) = export {
                    Some(sig.clone())
                } else {
                    None
                }
            )
            .unwrap();

        log::debug!("Instance::get_function success");

        Ok(JsFunction::from_js(sig, func))
    }
}
