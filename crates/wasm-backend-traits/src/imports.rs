use crate::errors::*;

use crate::{AsContextMut, WasmBackend, WType};
use crate::WValue;
use crate::Export;

use std::borrow::Cow;

pub trait Imports<WB: WasmBackend>: Clone {
    fn new(store: &mut <WB as WasmBackend>::Store) -> Self;

    fn insert(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <WB as WasmBackend>::Function,
    );

    fn register<S, I>(&mut self, name: S, namespace: I)
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <WB as WasmBackend>::Function)>;
}
/*
pub trait InsertFn<WB: WasmBackend, Args, Rets> {
    fn insert_fn<F>(&mut self, name: impl Into<String>, func: F)
    where
        F: Fn(&mut <WB as WasmBackend>::Caller<'_>, Args) -> Rets + Sync + Send + 'static;
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

    fn insert(&mut self, name: impl Into<String>, func: <WB as WasmBackend>::Function);
}

pub trait LikeNamespace<WB: WasmBackend> {}
*/
#[derive(Clone)]
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

pub trait FuncGetter<WB: WasmBackend, Args, Rets> {
    unsafe fn get_func(
        &mut self,
        name: &str,
    ) -> ResolveResult<
        Box<
            dyn FnMut(&mut <WB as WasmBackend>::ContextMut<'_>, Args) -> RuntimeResult<Rets>
                + Sync
                + Send
                + 'static,
        >,
    >;
}
