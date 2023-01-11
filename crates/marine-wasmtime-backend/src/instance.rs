use crate::{
    StoreState, val_type_to_wtype, WasmtimeFunction, WasmtimeFunctionExport,
    WasmtimeMemory, WasmtimeStore, WasmtimeWasmBackend,
};

use marine_wasm_backend_traits::*;

pub struct WasmtimeInstance {
    pub(crate) inner: wasmtime::Instance,
}

impl Instance<WasmtimeWasmBackend> for WasmtimeInstance {
    fn export_iter<'a>(
        &'a self,
        store: &'a mut WasmtimeStore,
    ) -> Box<dyn Iterator<Item = (&'a String, Export<WasmtimeWasmBackend>)> + 'a>
    {
        let iter = self
            .inner
            .exports(&mut store.inner)
            .map(|export| {
                let name = export.name();
                let export = match export.into_extern() {
                    wasmtime::Extern::Memory(memory) => Export::Memory(WasmtimeMemory { memory }),
                    wasmtime::Extern::Func(func) => Export::Function(WasmtimeFunction { func, signature: () }),
                    _ => Export::Other,
                };
                (name, export)
            });

        Box::new(iter)
    }

    fn memory(
        &self,
        store: &mut WasmtimeStore,
        memory_index: u32,
    ) -> <WasmtimeWasmBackend as WasmBackend>::Memory {
        let memory = self
            .inner
            .exports(&mut store.inner)
            .filter_map(wasmtime::Export::into_memory)
            .nth(memory_index as usize)
            .unwrap(); // todo change api to handle error

        WasmtimeMemory::new(memory)
    }

    fn memory_by_name(
        &self,
        store: &mut WasmtimeStore,
        memory_name: &str,
    ) -> Option<<WasmtimeWasmBackend as WasmBackend>::Memory> {
        let memory = self.inner.get_memory(&mut store.inner, memory_name);

        memory.map(WasmtimeMemory::new)
    }
/*
    fn get_func_no_args_no_rets<'a>(
        &'a self,
        store: &mut WasmtimeStore,
        name: &str,
    ) -> ResolveResult<Box<dyn Fn(&mut WasmtimeStore) -> RuntimeResult<()> + Sync + Send + 'a>>
    {
        let func = match self.inner.get_func(&mut store.inner, name) {
            None => return Err(ResolveError::Message(format!("no such function {}", name))),
            Some(func) => func,
        };

        let typed = func.typed::<(), (), _>(&store.inner).unwrap(); // todo handle error
        Ok(Box::new(move |store: &mut WasmtimeStore| {
            Ok(typed.call(&mut store.inner, ()).unwrap()) //todo handle error
        }))
    }*/

    fn get_function<'a>(
        &'a self,
        store: &mut WasmtimeStore,
        name: &str,
    ) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::Function> {
        let func = self.inner.get_func(&mut store.inner, name).unwrap(); // todo handle None
        let ty = func.ty(&store.inner);
        let params = ty
            .params()
            .map(|ty| {
                val_type_to_wtype(&ty).unwrap() // todo handle error
            })
            .collect::<Vec<_>>();
        let rets = ty
            .results()
            .map(|ty| {
                val_type_to_wtype(&ty).unwrap() // todo handle error
            })
            .collect::<Vec<_>>();

        let sig = FuncSig::new(params, rets);
        Ok(WasmtimeFunction {
            func,
            signature: sig,
        })
    }
}
