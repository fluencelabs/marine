use anyhow::anyhow;
use wasmtime::AsContextMut as WasmtimeAsContextMut;
use crate::{
    sig_to_fn_ty, StoreState, val_type_to_wtype, WasmtimeContextMut, WasmtimeFunction,
    WasmtimeMemory, WasmtimeStore, WasmtimeWasmBackend,
};

use marine_wasm_backend_traits::*;
use marine_wasm_backend_traits::WasmBackendError;
use crate::utils::fn_ty_to_sig;

pub struct WasmtimeInstance {
    pub(crate) inner: wasmtime::Instance,
    //pub(crate) exports: Vec<String, Export<WasmtimeWasmBackend>>
}

impl Instance<WasmtimeWasmBackend> for WasmtimeInstance {
    fn export_iter<'a>(
        &'a self,
        store: WasmtimeContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<WasmtimeWasmBackend>)> + 'a> {
        let exports = self.inner.exports(store.inner).map(|export| {
            let name = export.name();
            let export = match export.into_extern() {
                wasmtime::Extern::Memory(memory) => Export::Memory(WasmtimeMemory::new(memory)),
                wasmtime::Extern::Func(func) => Export::Function(WasmtimeFunction { inner: func }),
                _ => Export::Other,
            };
            (name, export)
        });
        Box::new(exports)
    }

    fn get_nth_memory(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        memory_index: u32,
    ) -> Option<<WasmtimeWasmBackend as WasmBackend>::Memory> {
        self.inner
            .exports(&mut store.as_context_mut().inner)
            .filter_map(wasmtime::Export::into_memory)
            .nth(memory_index as usize)
            .map(WasmtimeMemory::new)
    }

    fn get_memory(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        memory_name: &str,
    ) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::Memory> {
        self.inner
            .get_export(&mut store.as_context_mut().inner, memory_name)
            .ok_or(ResolveError::ExportNotFound(memory_name.to_string()))
            .and_then(|e| {
                e.into_memory().ok_or(ResolveError::ExportTypeMismatch(
                    "memory".to_string(),
                    "other".to_string(),
                ))
            })
            .map(WasmtimeMemory::new)
    }

    fn get_function<'a>(
        &'a self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        name: &str,
    ) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::Function> {
        let func = self
            .inner
            .get_export(&mut store.as_context_mut().inner, name)
            .ok_or(ResolveError::ExportNotFound(name.to_owned()))
            .and_then(|e| {
                e.into_func().ok_or(ResolveError::ExportTypeMismatch(
                    "function".to_string(),
                    "other".to_string(),
                ))
            })?;

        let ty = func.ty(&store.as_context().inner);
        let params = ty
            .params()
            .map(|ty| val_type_to_wtype(&ty))
            .collect::<Vec<_>>();
        let rets = ty
            .results()
            .map(|ty| val_type_to_wtype(&ty))
            .collect::<Vec<_>>();

        let sig = FuncSig::new(params, rets);
        Ok(WasmtimeFunction { inner: func })
    }
}
