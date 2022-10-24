pub mod errors;
pub mod exports;
pub mod wasi;
pub mod wtype;

use std::borrow::Cow;

use it_memory_traits::{MemoryView};

pub use errors::*;
pub use exports::*;
pub use wasi::*;
pub use wtype::*;


pub trait WasmBackend: Clone + 'static {
    // general
    type Module: Module<Self>;
    type Instance: Instance<Self>;
    // imports/exports -- subject to improvement
    type ImportObject: ImportObject<Self>;
    type Exports: Exports<Self>;
    type DynamicFunc: DynamicFunc<'static, Self>;
    type MemoryExport: MemoryExport;
    type FunctionExport: FunctionExport;
    type Namespace: Namespace<Self>;
    type ExportContext: for<'c> ExportContext<'c, Self>;
    type ExportedDynFunc: ExportedDynFunc<Self>;

    // interface types
    type WITMemory: Memory<Self> + it_memory_traits::Memory<Self::WITMemoryView> + Clone + 'static;
    type WITMemoryView: MemoryView + 'static;
    // wasi
    type Wasi: WasiImplementation<Self>;

    fn compile(wasm: &[u8]) -> WasmBackendResult<Self::Module>;
}



pub trait Module<WB: WasmBackend> {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]>;
    fn instantiate(
        &self,
        imports: &<WB as WasmBackend>::ImportObject,
    ) -> WasmBackendResult<<WB as WasmBackend>::Instance>;
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
    fn import_object(&self) -> &<WB as WasmBackend>::ImportObject;
    fn memory(&self, memory_index: u32) -> <WB as WasmBackend>::WITMemory;
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

pub trait DynamicFunc<'a, WB: WasmBackend> {
    fn new<'c, F>(sig: FuncSig, func: F) -> Self
    where
        F: Fn(&mut <WB as WasmBackend>::ExportContext, &[WValue]) -> Vec<WValue> + 'static;
}

pub trait InsertFn<WB: WasmBackend, Args, Rets> {
    fn insert_fn<F>(&mut self, name: impl Into<String>, func: F)
    where
        F: 'static + Fn(&mut <WB as WasmBackend>::ExportContext, Args) -> Rets + std::marker::Send;
}

pub trait Namespace<WB: WasmBackend>:
    LikeNamespace<WB>
    + InsertFn<WB, (), ()>
    + InsertFn<WB, (i32,), ()>
    + InsertFn<WB, (i32, i32), ()>
    + InsertFn<WB, (i32, i32, i32), ()>
    + InsertFn<WB, (i32, i32, i32, i32), ()>
{
    fn new() -> Self;

    fn insert(&mut self, name: impl Into<String>, func: <WB as WasmBackend>::DynamicFunc);
}

pub trait LikeNamespace<WB: WasmBackend> {}

pub trait ExportContext<'c, WB: WasmBackend>:
    FuncGetter<'c, (i32, i32), i32>
    + FuncGetter<'c, (i32, i32), ()>
    + FuncGetter<'c, i32, i32>
    + FuncGetter<'c, i32, ()>
    + FuncGetter<'c, (), i32>
    + FuncGetter<'c, (), ()>
{
    fn memory(&self, memory_index: u32) -> <WB as WasmBackend>::WITMemory;
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

pub trait FuncGetter<'c, Args, Rets> {
    unsafe fn get_func(
        &'c mut self,
        name: &str,
    ) -> ResolveResult<Box<dyn FnMut(Args) -> RuntimeResult<Rets> + 'c>>;
}