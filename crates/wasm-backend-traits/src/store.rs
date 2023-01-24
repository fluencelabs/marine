use crate::WasmBackend;

/// `Store` is an object that stores modules, instances, functions memories and so on.
/// `Store` is grow-only: once something added, it will not be removed until Store is destroyed.
/// Some of the implementations can limit allocated resources.
/// For example, Wasmtime cannot have more than 10000 instances in one `Store`.
///
/// Most of the functions in interface require a handle to `Store` to work
pub trait Store<WB: WasmBackend>: AsContextMut<WB> {
    fn new(backend: &WB) -> Self;
}

/// A temporary immutable handle to store
pub trait Context<WB: WasmBackend>: AsContext<WB> {}

/// A temporary mutable handle to store
pub trait ContextMut<WB: WasmBackend>: AsContextMut<WB> {}

pub trait AsContext<WB: WasmBackend> {
    fn as_context(&self) -> <WB as WasmBackend>::Context<'_>;
}

pub trait AsContextMut<WB: WasmBackend>: AsContext<WB> {
    fn as_context_mut(&mut self) -> <WB as WasmBackend>::ContextMut<'_>;
}
