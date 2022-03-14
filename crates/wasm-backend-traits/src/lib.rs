//pub mod errors;
//pub mod it_memory_traits;
//pub mod wasm_type_list;

//pub use wasm_type_list::WasmTypeList;
use std::borrow::Cow;
use std::fmt::Display;
//use std::fmt::Display;
use std::path::PathBuf;
use thiserror::Error;
use it_memory_traits::{SequentialMemoryView};
//use wasmer_it::IValue;

//use wasmer_core::types::FuncSig;
use wasmer_core::error::CallResult;
use wasmer_core::typed_func::WasmTypeList;
use wasmer_core::types::WasmExternType;
//use wasmer_core::typed_func::WasmTypeList;
//use wasmer_core::types::FuncSig as Wasme;
//pub use tuple_list::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub enum WValue {
    /// The `i32` type.
    I32(i32),
    /// The `i64` type.
    I64(i64),
    /// The `f32` type.
    F32(f32),
    /// The `f64` type.
    F64(f64),
    // /// The `v128` type.
    //V128(u128),
}

impl From<i32> for WValue {
    fn from(value: i32) -> Self {
        WValue::I32(value)
    }
}

impl From<i64> for WValue {
    fn from(value: i64) -> Self {
        WValue::I64(value)
    }
}

impl From<f32> for WValue {
    fn from(value: f32) -> Self {
        WValue::F32(value)
    }
}

impl From<f64> for WValue {
    fn from(value: f64) -> Self {
        WValue::F64(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WType {
    /// The `i32` type.
    I32,
    /// The `i64` type.
    I64,
    /// The `f32` type.
    F32,
    /// The `f64` type.
    F64,
    // /// The `v128` type.
    // V128,
}

impl WValue {
    pub fn to_u128(&self) -> u128 {
        match *self {
            Self::I32(x) => x as u128,
            Self::I64(x) => x as u128,
            Self::F32(x) => f32::to_bits(x) as u128,
            Self::F64(x) => f64::to_bits(x) as u128,
            //Self::V128(x) => x,
        }
    }
}

impl std::fmt::Display for WType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
    type ExportContext: for<'c> ExportContext<'c, Self>;
    type ExportedDynFunc: ExportedDynFunc<Self>;

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
    fn get_func_no_args_no_rets<'a>(
        &'a self,
        name: &str,
    ) -> wasmer_core::error::ResolveResult<
        Box<dyn Fn() -> wasmer_core::error::RuntimeResult<()> + 'a>,
    >;

    fn get_dyn_func<'a>(
        &'a self,
        name: &str,
    ) -> wasmer_core::error::ResolveResult<<WB as WasmBackend>::ExportedDynFunc>;
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
    fn new<'c, F>(sig: FuncSig, func: F) -> Self
    where
        F: Fn(&mut <WB as WasmBackend>::ExportContext, &[WValue]) -> Vec<WValue> + 'static;
}

pub trait Namespace<WB: WasmBackend>: LikeNamespace<WB> {
    fn new() -> Self;

    fn insert(&mut self, name: impl Into<String>, func: <WB as WasmBackend>::DynamicFunc);
}

pub trait LikeNamespace<WB: WasmBackend> {}

pub trait ExportContext<'c, WB: WasmBackend> {
    fn memory(&self, memory_index: u32) -> <WB as WasmBackend>::WITMemory;

    unsafe fn get_export_func_by_name<Args, Rets>(
        &'c mut self,
        name: &str,
    ) -> Result<Box<dyn FnMut(Args) -> Result<Rets, wasmer_runtime::error::RuntimeError> + 'c>, wasmer_runtime::error::ResolveError>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList;
}

pub trait ExportedDynFunc<WB: WasmBackend> {
    fn signature(&self) -> &FuncSig;

    fn call(&self, args: &[WValue]) -> CallResult<Vec<WValue>>;
}

pub struct FuncSig {
    params: Cow<'static, [WType]>,
    returns: Cow<'static, [WType]>,
}

impl FuncSig {
    pub fn new<Params, Returns>(params: Params, returns: Returns) -> Self
    where
        Params: Into<Cow<'static, [WType]>>,
        Returns: Into<Cow<'static, [WType]>>,
    {
        Self {
            params: params.into(),
            returns: returns.into(),
        }
    }

    pub fn params(&self) -> impl Iterator<Item = &WType> {
        self.params.iter()
    }

    pub fn returns(&self) -> impl Iterator<Item = &WType> {
        self.returns.iter()
    }
}
