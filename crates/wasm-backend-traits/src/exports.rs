use crate::errors::*;

use crate::WasmBackend;
use crate::FuncSig;
use crate::WValue;

pub trait Exports<WB: WasmBackend> {
    fn get_func_no_args_no_rets<'a>(
        &'a self,
        name: &str,
    ) -> ResolveResult<Box<dyn Fn() -> RuntimeResult<()> + 'a>>;

    fn get_dyn_func<'a>(
        &'a self,
        name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::ExportedDynFunc>;
}

pub enum Export<M: MemoryExport, F: FunctionExport> {
    Memory(M),
    Function(F),
    Other,
}

pub type ExportShort<WB: WasmBackend> = Export<<WB as WasmBackend>::MemoryExport, <WB as WasmBackend>::FunctionExport>;

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
