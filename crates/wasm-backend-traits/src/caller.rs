use crate::{AsContextMut, FuncGetter, WasmBackend};

/// `Caller` is a structure that is used to pass context to imports.
/// It serves as a handle to `Store`, and also provides access to `Memory` and export functions
/// from the caller instance, if there is one.
pub trait Caller<WB: WasmBackend>:
    FuncGetter<WB, (i32, i32), i32>
    + FuncGetter<WB, (i32, i32), ()>
    + FuncGetter<WB, i32, i32>
    + FuncGetter<WB, i32, ()>
    + FuncGetter<WB, (), i32>
    + FuncGetter<WB, (), ()>
    + AsContextMut<WB>
{
    /// Gets the `Memory` from the caller instance.
    /// Returns `None` if function was called directly from host.
    fn memory(&mut self, memory_index: u32) -> Option<<WB as WasmBackend>::Memory>;
}
