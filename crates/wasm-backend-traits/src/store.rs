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

/*
impl<T: AsContext<WB>, WB: WasmBackend> AsContext<WB> for &T {
    fn as_context(&self) -> <WB as WasmBackend>::Context<'_> {
        self.as_context()
    }
}

impl<T: AsContext<WB>, WB: WasmBackend> AsContext<WB> for &mut T {
    fn as_context(&self) -> <WB as WasmBackend>::Context<'_> {
    }
}

impl<T: AsContextMut<WB>, WB: WasmBackend> AsContextMut<WB> for &mut T {
    fn as_context_mut(&mut self) -> <WB as WasmBackend>::ContextMut<'_> {
        self.as_context_mut()
    }
}
*/
