use wasmer::Extern;
use crate::{WasmerBackend, WasmerFunction, WasmerMemory, WasmerStore};

use marine_wasm_backend_traits::*;

pub struct WasmerInstance {
    pub(crate) inner: wasmer::Instance,
}

impl Instance<WasmerBackend> for WasmerInstance {
    fn export_iter<'a>(
        &'a self,
        store: &'a mut WasmerStore,
    ) -> Box<dyn Iterator<Item = (String, Export<WasmerBackend>)> + 'a> {
        todo!()
    }

    fn memory(&self, _store: &mut WasmerStore, memory_index: u32) -> WasmerMemory {
        self.inner
            .exports
            .iter()
            .filter_map(|(name, export)| {
                match export {
                    Extern::Memory(memory) => Some(memory),
                    _ => None
                }
            }) // todo is there a way to make it better?
            .nth(memory_index as usize)
            .map(|memory| WasmerMemory { inner: memory.clone() })
            .unwrap() // todo handle error
    }

    fn memory_by_name(
        &self,
        _store: &mut <WasmerBackend as WasmBackend>::Store,
        name: &str,
    ) -> Option<WasmerMemory> {
        self.inner
            .exports
            .get_memory(name)
            .ok()
            .map(|memory| WasmerMemory { inner: memory.clone()})
    }

    fn get_func_no_args_no_rets<'a>(
        &'a self,
        store: &mut <WasmerBackend as WasmBackend>::Store,
        name: &str,
    ) -> ResolveResult<Box<dyn Fn(&mut WasmerStore) -> RuntimeResult<()> + Sync + Send + 'a>> {
        todo!()
    }

    fn get_dyn_func<'a>(
        &'a self,
        store: &mut <WasmerBackend as WasmBackend>::Store,
        name: &str,
    ) -> ResolveResult<<WasmerBackend as WasmBackend>::Function> {
        self.inner
            .exports
            .get_function(name)
            .map_err(|e| ResolveError::Message(format!("wasmer cannot find function {}", e)))
            .map(|func| func.clone().into())
    }
}
