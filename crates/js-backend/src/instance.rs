use marine_wasm_backend_traits::prelude::*;
use crate::JsWasmBackend;

#[derive(Clone)]
pub struct JsInstance {}

impl Instance<JsWasmBackend> for JsInstance {
    fn export_iter<'a>(
        &'a self,
        store: <JsWasmBackend as WasmBackend>::ContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<JsWasmBackend>)> + 'a> {
        todo!()
    }

    fn get_nth_memory(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        memory_index: u32,
    ) -> Option<<JsWasmBackend as WasmBackend>::Memory> {
        todo!()
    }

    fn get_memory(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        memory_name: &str,
    ) -> ResolveResult<<JsWasmBackend as WasmBackend>::Memory> {
        todo!()
    }

    fn get_function(
        &self,
        store: &mut impl AsContextMut<JsWasmBackend>,
        name: &str,
    ) -> ResolveResult<<JsWasmBackend as WasmBackend>::Function> {
        todo!()
    }
}
