/*
 * Copyright 2023 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::errors::*;

use crate::AsContext;
use crate::WasmBackend;
use crate::WType;

use std::borrow::Cow;
use std::fmt::Formatter;
use std::future::Future;
use std::sync::Arc;
use futures::future::BoxFuture;

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
        store: &impl AsContext<WB>,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <WB as WasmBackend>::HostFunction,
    ) -> Result<(), ImportError>;

    /// Inserts several named functions to the same namespace `module` at once, an equivalent to multiple calls of `insert`.
    /// # Errors:
    ///     An error returned if such combination of `module` and `name` already has an associated function.
    ///
    fn register<S, I>(
        &mut self,
        store: &impl AsContext<WB>,
        name: S,
        namespace: I,
    ) -> Result<(), ImportError>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <WB as WasmBackend>::HostFunction)>;
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

    pub fn params(&self) -> &[WType] {
        &self.params
    }

    pub fn returns(&self) -> &[WType] {
        &self.returns
    }
}

impl std::fmt::Debug for FuncSig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "params: {:?}, returns: {:?}",
            self.params(),
            self.returns
        )
    }
}

pub type TypedFuncFuture<'c, Rets> = BoxFuture<'c, RuntimeResult<Rets>>;

pub type TypedFunc<WB, Args, Rets> = Arc<
    dyn for<'ctx1, 'ctx2> Fn(
            &'ctx1 mut <WB as WasmBackend>::ContextMut<'ctx2>,
            Args,
        ) -> TypedFuncFuture<'ctx1, Rets>
        + Sync
        + Send
        + 'static,
>;

pub trait FuncGetter<WB: WasmBackend, Args, Rets> {
    /// Gets an export function from the calling instance.
    fn get_func(&mut self, name: &str) -> ResolveResult<TypedFunc<WB, Args, Rets>>;
}
