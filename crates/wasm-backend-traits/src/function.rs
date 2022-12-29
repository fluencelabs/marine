use crate::{CallResult, FuncSig, WasmBackend, WValue};

pub trait Function<WB: WasmBackend>: Send + Sync {
    fn signature<'c>(&self, store: <WB as WasmBackend>::ContextMut<'c>) -> &FuncSig;

    fn call<'c>(
        &self,
        store: <WB as WasmBackend>::ContextMut<'c>, // <- Store or ExportContext. Need to be able to extract wasmtime::StoreContextMut from them. Same for many methods.
        args: &[WValue],
    ) -> CallResult<Vec<WValue>>;
}