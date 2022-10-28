use crate::errors::*;

use crate::WasmBackend;
use crate::FuncSig;
use crate::WValue;

pub enum Export<M: MemoryExport, F: FunctionExport> {
    Memory(M),
    Function(F),
    Other,
}

pub trait ExportedDynFunc<WB: WasmBackend> {
    fn signature(&self) -> &FuncSig;

    fn call(&self, args: &[WValue]) -> CallResult<Vec<WValue>>;
}

pub trait MemoryExport {}

pub trait FunctionExport {}

pub trait Memory<WB: WasmBackend> {
    fn new(export: <WB as WasmBackend>::MemoryExport) -> Self;

    fn size(&self) -> usize;
}
