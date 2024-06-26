/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::errors::*;

use crate::AsContext;
use crate::WasmBackend;
use crate::WType;

use futures::future::BoxFuture;

use std::borrow::Cow;
use std::fmt::Formatter;
use std::sync::Arc;

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
