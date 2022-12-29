use crate::errors::*;

use crate::{ContextMut, DelayedContextLifetime, WasmBackend};
use crate::FuncSig;
use crate::WValue;

pub enum Export<WB: WasmBackend> {
    Memory(<WB as WasmBackend>::Memory),
    Function(<WB as WasmBackend>::Memory),
    Other,
}

pub trait MemoryExport {}

pub trait FunctionExport {}

pub trait Memory<WB: WasmBackend>:
    it_memory_traits::Memory<<WB as WasmBackend>::MemoryView, DelayedContextLifetime<WB>>
    + Clone
    + Send
    + Sync
    + 'static
{
    fn size(&self, store: &mut <WB as WasmBackend>::ContextMut<'_>) -> usize;
}
