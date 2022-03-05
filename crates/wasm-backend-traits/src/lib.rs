//pub mod errors;
//pub mod it_memory_traits;

//use std::fmt::Display;
use std::path::PathBuf;
use thiserror::Error;
use wasmer_core::types::FuncSig;
use it_memory_traits::{SequentialMemoryView};

pub struct Value {}

#[derive(Debug, Error)]
pub enum WasmBackendError {
    #[error("Some error")]
    SomeError,
}

pub type WasmBackendResult<T> = Result<T, WasmBackendError>;

pub trait WasmBackend: Clone + 'static {
    type IO: ImportObject<Self>;
    type Exports: Exports<Self>;
    type MemoryExport: MemoryExport;
    type WITMemory: Memory<Self> + it_memory_traits::Memory<Self::WITMemoryView> + Clone + 'static;
    //type SR: SequentialReader;
    //type SW: SequentialWriter;
    type DynamicFunc: DynamicFunc<'static, Self>;
    type WITMemoryView: for<'a> SequentialMemoryView<'a /* SR = Self::SR, SW = Self::SW*/> + 'static;
    type FunctionExport: FunctionExport;
    type M: Module<Self>;
    type I: Instance<Self>;
    type Wasi: WasiImplementation<Self>;
    type Namespace: Namespace<Self>;
    type ExportContext: ExportContext<Self>;

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
    fn export_iter<'a>(
        &'a self,
    ) -> Box<
        dyn Iterator<
                Item = (
                    String,
                    Export<<WB as WasmBackend>::MemoryExport, <WB as WasmBackend>::FunctionExport>,
                ),
            > + 'a,
    >;
    fn exports(&self) -> &<WB as WasmBackend>::Exports;
    fn import_object(&self) -> &<WB as WasmBackend>::IO;
    fn memory(&self, memory_index: u32) -> <WB as WasmBackend>::WITMemory;
}

pub trait Exports<WB: WasmBackend> {
    fn get<'a, T: wasmer_core::export::Exportable<'a>>(
        &'a self,
        name: &str,
    ) -> wasmer_core::error::ResolveResult<T>;

    fn get_func_no_args<'a, Rets: wasmer_core::typed_func::WasmTypeList + 'a>(
        &'a self,
        name: &str,
    ) -> wasmer_core::error::ResolveResult<Box<dyn Fn() -> wasmer_core::error::RuntimeResult<Rets> + 'a>>;
}

pub enum Export<M: MemoryExport, F: FunctionExport> {
    Memory(M),
    Function(F),
    Other,
}

pub trait ImportObject<WB: WasmBackend>:
    Clone
    + Extend<(
        String,
        String,
        Export<<WB as WasmBackend>::MemoryExport, <WB as WasmBackend>::FunctionExport>,
    )>
{
    fn new() -> Self;
    fn extend_with_self(&mut self, other: Self);

    fn register<S>(
        &mut self,
        name: S,
        namespace: <WB as WasmBackend>::Namespace,
    ) -> Option<Box<dyn LikeNamespace<WB>>>
    where
        S: Into<String>;

    fn get_memory_env(
        &self,
    ) -> Option<Export<<WB as WasmBackend>::MemoryExport, <WB as WasmBackend>::FunctionExport>>;
}

pub trait WasiImplementation<WB: WasmBackend> {
    fn generate_import_object_for_version(
        version: wasmer_wasi::WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<Vec<u8>>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<<WB as WasmBackend>::IO, String>;

    fn get_wasi_state(instance: &mut <WB as WasmBackend>::I) -> &wasmer_wasi::state::WasiState;
}

pub trait MemoryExport {}

pub trait FunctionExport {}

pub trait Memory<WB: WasmBackend> {
    fn new(export: <WB as WasmBackend>::MemoryExport) -> Self;

    fn size(&self) -> usize;
}

pub trait DynamicFunc<'a, WB: WasmBackend> {
    fn new<'c, F>(sig: std::sync::Arc<FuncSig>, func: F) -> Self
    where
        F: Fn(
                &mut <WB as WasmBackend>::ExportContext,
                &[wasmer_core::types::Value],
            ) -> Vec<wasmer_core::types::Value>
            + 'static;
}

pub trait Namespace<WB: WasmBackend>: LikeNamespace<WB> {
    fn new() -> Self;

    fn insert(&mut self, name: impl Into<String>, func: <WB as WasmBackend>::DynamicFunc);
}

pub trait LikeNamespace<WB: WasmBackend> {}

pub trait ExportContext<WB: WasmBackend> {
    fn memory(&self, memory_index: u32) -> <WB as WasmBackend>::WITMemory;

    unsafe fn get_export_func_by_name<'a, Args, Rets>(
        &mut self,
        name: &str,
    ) -> Result<wasmer_runtime::Func<'a, Args, Rets>, wasmer_runtime::error::ResolveError>
    where
        Args: wasmer_core::typed_func::WasmTypeList,
        Rets: wasmer_core::typed_func::WasmTypeList;
}
