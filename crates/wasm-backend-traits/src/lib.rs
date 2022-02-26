use std::fmt::Display;
use std::path::PathBuf;
use thiserror::Error;

pub struct Value {}

#[derive(Debug, Error)]
pub enum WasmBackendError {
    #[error("Some error")]
    SomeError,
}

pub type WasmBackendResult<T> = Result<T, WasmBackendError>;

pub trait WasmBackend: Clone + 'static {
    type IO: ImportObject<Self>;
    type E: Export;
    type M: Module<Self>;
    type I: Instance<Self>;
    type Wasi: WasiImplementation<Self>;

    fn compile(wasm: &[u8]) -> WasmBackendResult<Self::M>;
}

pub trait Module<WB: WasmBackend> {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]>;
    fn instantiate(
        &self,
        imports: &<WB as WasmBackend>::IO,
    ) -> WasmBackendResult<<WB as WasmBackend>::I>;
}

pub trait Instance<WB: WasmBackend> {
    fn export_iter<'a>(&'a self)
        -> Box<dyn Iterator<Item = (String, wasmer_runtime::Export)> + 'a>;
    fn exports(&self) -> &wasmer_core::instance::Exports;
    fn import_object(&self) -> &<WB as WasmBackend>::IO;

    // maybe hide them inside impl
    fn context(&self) -> &wasmer_core::vm::Ctx;
    fn context_mut(&mut self) -> &mut wasmer_core::vm::Ctx;
}

pub trait Export {}

pub trait ImportObject<WB: WasmBackend>:
    Clone + Extend<(String, String, <WB as WasmBackend>::E)>
{
    fn new() -> Self;
    fn extend_with_self(&mut self, other: Self);

    fn register<S, N>(
        &mut self,
        name: S,
        namespace: N,
    ) -> Option<Box<dyn wasmer_runtime::LikeNamespace>>
    where
        S: Into<String>,
        N: wasmer_runtime::LikeNamespace + Send + 'static;

    fn maybe_with_namespace<Func, InnerRet>(&self, namespace: &str, f: Func) -> Option<InnerRet>
    where
        Func: FnOnce(&(dyn wasmer_runtime::LikeNamespace + Send)) -> Option<InnerRet>,
        InnerRet: Sized;
}

pub trait WasiImplementation<WB: WasmBackend> {
    fn generate_import_object_for_version(
        version: wasmer_wasi::WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<Vec<u8>>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<<WB as WasmBackend>::IO, String>;
}
