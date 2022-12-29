use crate::{AsContextMut, FuncGetter, WasmBackend};

pub trait Caller<WB: WasmBackend>:
    FuncGetter<WB, (i32, i32), i32>
    + FuncGetter<WB, (i32, i32), ()>
    + FuncGetter<WB, i32, i32>
    + FuncGetter<WB, i32, ()>
    + FuncGetter<WB, (), i32>
    + FuncGetter<WB, (), ()>
    + AsContextMut<WB>
{
    fn memory(&mut self, memory_index: u32) -> <WB as WasmBackend>::Memory;
}
