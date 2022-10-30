use crate::errors::*;

use crate::{WasmBackend, WType};
use crate::WValue;
use crate::Export;

use std::borrow::Cow;

pub trait ImportObject<WB: WasmBackend>: Clone {
    fn new(store: &mut <WB as WasmBackend>::Store) -> Self;

    fn register<S>(
        &mut self,
        name: S,
        namespace: <WB as WasmBackend>::Namespace,
    ) -> Option<Box<dyn LikeNamespace<WB>>>
        where
            S: Into<String>;
}

pub trait DynamicFunc<'a, WB: WasmBackend> {
    fn new<'c, F>(sig: FuncSig, func: F) -> Self
        where
            F: Fn(&mut dyn ExportContext<WB>, &[WValue]) -> Vec<WValue> + 'static;
}

pub trait InsertFn<WB: WasmBackend, Args, Rets> {
    fn insert_fn<F>(&mut self, name: impl Into<String>, func: F)
        where
            F: 'static + Fn(&mut dyn ExportContext<WB>, Args) -> Rets + std::marker::Send;
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
FuncGetter<(i32, i32), i32>
+ FuncGetter< (i32, i32), ()>
+ FuncGetter< i32, i32>
+ FuncGetter< i32, ()>
+ FuncGetter<(), i32>
+ FuncGetter< (), ()>
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

pub trait FuncGetter<Args, Rets> {
    unsafe fn get_func<'c>(
        &'c mut self,
        name: &str,
    ) -> ResolveResult<Box<dyn FnMut(Args) -> RuntimeResult<Rets> + 'c>>;
}
