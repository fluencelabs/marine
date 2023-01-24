use crate::errors::*;

use crate::{WasmBackend, WType};

use std::borrow::Cow;

/// A "Linker" object, that is used to match functions with module imports during instantiation.
/// Cloning is a cheap operation for this object. All clones refer to the same data in store.
pub trait Imports<WB: WasmBackend>: Clone {
    /// Creates a new empty object.
    fn new(store: &mut <WB as WasmBackend>::Store) -> Self;

    /// Inserts a function with name `name` to the namespace `module`.
    /// # Errors:
    ///     An error returned if such combination of `module` and `name` already has an associated function.
    fn insert(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <WB as WasmBackend>::Function,
    ) -> Result<(), ImportError>;

    /// Inserts many named functions to the samne namespace `module`. Is equivalent to multiple calls to `insert`.
    /// # Errors:
    ///     An error returned if such combination of `module` and `name` already has an associated function.
    ///
    fn register<S, I>(&mut self, name: S, namespace: I) -> Result<(), ImportError>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <WB as WasmBackend>::Function)>;
}

/// A type representing function signature.
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
    /// Gets an export function from the calling instance.
    fn get_func(
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
