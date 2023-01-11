use crate::{sig_to_fn_ty, StoreState, val_type_to_wtype, WasmtimeFunction, WasmtimeFunctionExport, WasmtimeMemory, WasmtimeStore, WasmtimeWasmBackend};

use marine_wasm_backend_traits::*;
use crate::utils::fn_ty_to_sig;

pub struct WasmtimeInstance {
    pub(crate) inner: wasmtime::Instance,
}

impl Instance<WasmtimeWasmBackend> for WasmtimeInstance {
    fn export_iter<'a>(
        &'a self,
        store: &'a mut impl AsContextMut<WasmtimeWasmBackend>,
    ) -> Box<dyn Iterator<Item = (&'a String, Export<WasmtimeWasmBackend>)> + 'a>
    {
        let iter = self
            .inner
            .exports(&mut store.as_context_mut())
            .map(|export| {
                let name = export.name();
                let export = match export.into_extern() {
                    wasmtime::Extern::Memory(memory) => Export::Memory(WasmtimeMemory::new(memory)),
                    wasmtime::Extern::Func(func) => {
                        let ty = func.ty(&store.as_context());
                        let sig = fn_ty_to_sig(&ty);
                        Export::Function(WasmtimeFunction { inner: func, signature: sig })
                    },
                    _ => Export::Other,
                };
                (name, export)
            });

        Box::new(iter)
    }

    fn memory(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        memory_index: u32,
    ) -> <WasmtimeWasmBackend as WasmBackend>::Memory {
        let memory = self
            .inner
            .exports(&mut store.as_context_mut().inner)
            .filter_map(wasmtime::Export::into_memory)
            .nth(memory_index as usize)
            .unwrap(); // todo change api to handle error

        WasmtimeMemory::new(memory)
    }

    fn memory_by_name(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        memory_name: &str,
    ) -> Option<<WasmtimeWasmBackend as WasmBackend>::Memory> {
        let memory = self.inner.get_memory(&mut store.as_context_mut().inner, memory_name);

        memory.map(WasmtimeMemory::new)
    }

    fn get_function<'a>(
        &'a self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        name: &str,
    ) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::Function> {
        let func = self.inner.get_func(&mut store.as_context_mut().inner, name).unwrap(); // todo handle None
        let ty = func.ty(&store.as_context().inner);
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
            inner: func,
            signature: sig,
        })
    }
}
