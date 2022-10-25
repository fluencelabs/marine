use crate::errors::*;

use crate::{WasmBackend, WType};
use crate::WValue;
use crate::Export;

use std::borrow::Cow;

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
