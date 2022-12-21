use crate::errors::*;

use crate::{ContextMut, WasmBackend};
use crate::FuncSig;
use crate::WValue;

pub enum Export<M: MemoryExport, F: FunctionExport> {
    Memory(M),
    Function(F),
    Other,
}

pub trait ExportedDynFunc<WB: WasmBackend> {
    fn signature<'c>(&self, store: <WB as WasmBackend>::ContextMut<'c>) -> &FuncSig;

    fn call<'c>(
        &self,
        store: <WB as WasmBackend>::ContextMut<'c>, // <- Store or ExportContext. Need to be able to extract wasmtime::StoreContextMut from them. Same for many methods.
        args: &[WValue],
    ) -> CallResult<Vec<WValue>>;
}

pub trait MemoryExport {}

pub trait FunctionExport {}

pub trait Memory<WB: WasmBackend> {
    fn new(export: <WB as WasmBackend>::MemoryExport) -> Self;

    fn size(&self) -> usize;
}
