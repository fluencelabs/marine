use crate::WasmBackend;

pub trait Store<WB: WasmBackend>: AsContextMut<WB> {
    fn new(backend: &WB) -> Self;
}

pub trait Context<WB: WasmBackend>: AsContext<WB> {}

pub trait ContextMut<WB: WasmBackend>: AsContextMut<WB> {}

pub trait AsContext<WB: WasmBackend> {
    fn as_context(&self) -> <WB as WasmBackend>::Context<'_>;
}

pub trait AsContextMut<WB: WasmBackend>: AsContext<WB> {
    fn as_context_mut(&mut self) -> <WB as WasmBackend>::ContextMut<'_>;
}
