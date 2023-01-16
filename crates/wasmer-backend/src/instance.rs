use wasmer::{AsStoreMut, Extern};
use crate::{
    func_sig_to_function_type, function_type_to_func_sig, wasmer_ty_to_generic_ty, WasmerBackend,
    WasmerContextMut, WasmerFunction, WasmerMemory, WasmerStore,
};

use marine_wasm_backend_traits::*;

pub struct WasmerInstance {
    pub(crate) inner: wasmer::Instance,
}

impl Instance<WasmerBackend> for WasmerInstance {
    fn export_iter<'a>(
        &'a self,
        mut store: WasmerContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<WasmerBackend>)> + 'a> {
        let iter =
            self.inner
                .exports
                .iter()
                .map(move |(name, export): (&String, &wasmer::Extern)| {
                    let export = match export {
                        wasmer::Extern::Function(function) => Export::Function(
                            WasmerFunction::from_func(&mut store, function.clone()),
                        ),
                        wasmer::Extern::Memory(memory) => Export::Memory(memory.clone().into()),
                        _ => Export::Other,
                    };

                    (name.as_str(), export)
                });
        Box::new(iter)
    }

    fn memory(
        &self,
        _store: &mut impl AsContextMut<WasmerBackend>,
        memory_index: u32,
    ) -> WasmerMemory {
        self.inner
            .exports
            .iter()
            .filter_map(|(name, export)| match export {
                Extern::Memory(memory) => Some(memory),
                _ => None,
            }) // todo is there a way to make it better?
            .nth(memory_index as usize)
            .map(|memory| WasmerMemory {
                inner: memory.clone(),
            })
            .unwrap() // todo handle error
    }

    fn memory_by_name(
        &self,
        _store: &mut impl AsContextMut<WasmerBackend>,
        name: &str,
    ) -> Option<WasmerMemory> {
        self.inner
            .exports
            .get_memory(name)
            .ok()
            .map(|memory| WasmerMemory {
                inner: memory.clone(),
            })
    }

    fn get_function(
        &self,
        store: &mut impl AsContextMut<WasmerBackend>,
        name: &str,
    ) -> ResolveResult<<WasmerBackend as WasmBackend>::Function> {
        self.inner
            .exports
            .get_function(name)
            .map_err(|e| ResolveError::Message(format!("wasmer cannot find function {}", e)))
            .map(|func| {
                let ty = func.ty(&store.as_context());

                WasmerFunction {
                    sig: function_type_to_func_sig(&ty),
                    inner: func.clone(),
                }
            })
    }
}
