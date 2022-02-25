use thiserror::Error;

pub struct Value {}

#[derive(Debug, Error)]
pub enum WasmBackendError {
    #[error("Some error")]
    SomeError,
}

pub type WasmBackendResult<T> = Result<T, WasmBackendError>;

pub trait WasmBackend: Clone + 'static {
    type M: Module;

    fn compile(wasm: &[u8]) -> WasmBackendResult<Self::M>;
}

pub trait Module {
    type I: Instance;

    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]>;
    fn instantiate(&self, imports: &wasmer_runtime::ImportObject) -> WasmBackendResult<Self::I>;
}

pub trait Instance {

    fn export_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (String, wasmer_runtime::Export)> + 'a>;
    fn exports(&self) -> &wasmer_core::instance::Exports;
    fn import_object(&self) -> &wasmer_runtime::ImportObject;

    // maybe hide them inside impl
    fn context(&self) -> &wasmer_core::vm::Ctx;
    fn context_mut(&mut self) -> &mut wasmer_core::vm::Ctx;
}
