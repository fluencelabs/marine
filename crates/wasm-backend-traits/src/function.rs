use crate::{AsContextMut, CallResult, FuncSig, WasmBackend, WValue};

pub trait Function<WB: WasmBackend>: Send + Sync {
    fn new<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static;

    fn new_with_ctx<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(<WB as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static;

    fn signature<'c>(&self, store: &mut impl AsContextMut<WB>) -> &FuncSig;

    fn call<'c>(
        &self,
        store: &mut impl AsContextMut<WB>, // <- Store or ExportContext. Need to be able to extract wasmtime::StoreContextMut from them. Same for many methods.
        args: &[WValue],
    ) -> CallResult<Vec<WValue>>;
}
