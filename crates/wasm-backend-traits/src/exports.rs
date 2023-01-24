use crate::{DelayedContextLifetime, WasmBackend};

/// A general export representaion. Now only `Memory` and `Function` are supported values.
pub enum Export<WB: WasmBackend> {
    Memory(<WB as WasmBackend>::Memory),
    Function(<WB as WasmBackend>::Function),
    Other,
}

// TODO: add read/write/etc methods to the `Memory` trait,
// and then make a generic implementation of interface-types traits
/// A wasm memory handle. As it is only a handle to an object in `Store`, cloning is cheap.
pub trait Memory<WB: WasmBackend>:
    it_memory_traits::Memory<<WB as WasmBackend>::MemoryView, DelayedContextLifetime<WB>>
    + Clone
    + Send
    + Sync
    + 'static
{
    /// Get the size of the allocated memory in bytes.
    fn size(&self, store: &mut <WB as WasmBackend>::ContextMut<'_>) -> usize;
}
