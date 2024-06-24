/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::AsContextMut;
use crate::FuncSig;
use crate::impl_for_each_function_signature;
use crate::RuntimeResult;
use crate::WasmBackend;
use crate::WValue;

use futures::future::BoxFuture;

/// A host function ready to be used as an import for instantiating a module.
/// As it is only a handle to an object in `Store`, cloning is cheap.
pub trait HostFunction<WB: WasmBackend>: Send + Sync + Clone {
    /// Creates a new function with dynamic signature.
    /// The signature check is performed at runtime.
    fn new<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> anyhow::Result<Vec<WValue>> + Sync + Send + 'static;

    /// Creates a new function with dynamic signature that needs a context.
    fn new_with_caller<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(
                <WB as WasmBackend>::ImportCallContext<'c>,
                &[WValue],
            ) -> anyhow::Result<Vec<WValue>>
            + Sync
            + Send
            + 'static;

    /// Creates a new function with dynamic signature that needs a context.
    fn new_with_caller_async<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(
                <WB as WasmBackend>::ImportCallContext<'c>,
                &'c [WValue],
            ) -> BoxFuture<'c, anyhow::Result<Vec<WValue>>>
            + Sync
            + Send
            + 'static;

    /// Creates a new function with dynamic signature that needs a context.
    fn new_async<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&'c [WValue]) -> BoxFuture<'c, anyhow::Result<Vec<WValue>>>
            + Sync
            + Send
            + 'static;

    /// Creates a new function with static signature.
    /// Requires less runtime checks when called.
    fn new_typed<Params, Results, Env>(
        store: &mut impl AsContextMut<WB>,
        func: impl IntoFunc<WB, Params, Results, Env>,
    ) -> Self;

    /// Returns the signature of the function.
    /// The signature is constructed each time this function is called, so
    /// it is not recommended to use this function extensively.
    fn signature(&self, store: &mut impl AsContextMut<WB>) -> FuncSig;
}

/// A Wasm function handle, it can be either a function from a host or an export from an `Instance`.
/// As it is only a handle to an object in `Store`, cloning is cheap
pub trait ExportFunction<WB: WasmBackend>: Send + Sync + Clone {
    /// Returns the signature of the function.
    /// The signature is constructed each time this function is called, so
    /// it is not recommended to use this function extensively.
    fn signature(&self, store: &mut impl AsContextMut<WB>) -> FuncSig;

    /// Calls the wasm function.
    /// # Panics:
    ///     If given a store different from the one that stores the function.
    /// # Errors:
    ///     See `RuntimeError` documentation.
    fn call_async<'args>(
        &'args self,
        store: &'args mut impl AsContextMut<WB>,
        args: &'args [WValue],
    ) -> BoxFuture<'args, RuntimeResult<Vec<WValue>>>;
}

/// A helper trait for creating a function with a static signature.
/// Should not be implemented by users.
/// Implemented for all functions that meet the following criteria:
///     * implement Send + Sync + 'static
///     * take or not take ImportCallContext as first parameter
///     * take from 0 to 16 i32 parameters
///     * return () or i32
pub trait IntoFunc<WB: WasmBackend, Params, Results, Env> {
    fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::HostFunction;
}

/// An indicator of using ImportCallContext argument.
pub struct WithEnv {}

/// An indicator of using ImportCallContext argument.
pub struct WithoutEnv {}

#[macro_export]
macro_rules! replace_with {
    ($from:ident -> $to:ident) => {
        $to
    };
}

macro_rules! impl_into_func {
    ($num:tt $($args:ident)*) => (paste::paste!{
        #[allow(non_snake_case)]
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), (), WithoutEnv> for F
        where
            WB: WasmBackend,
            F: Fn($(replace_with!($args -> i32),)*) + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::HostFunction {
                <WB as WasmBackend>::HostFunction:: [< new_typed_ $num >] (ctx.as_context_mut(), self)
            }
        }

        #[allow(non_snake_case)]
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), (), WithEnv> for F
        where
            WB: WasmBackend,
            F: Fn(<WB as WasmBackend>::ImportCallContext<'_>, $(replace_with!($args -> i32),)*) + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::HostFunction {
                <WB as WasmBackend>::HostFunction:: [< new_typed_with_env_ $num >] (ctx.as_context_mut(), self)
            }
        }

        #[allow(non_snake_case)]
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), i32, WithoutEnv> for F
        where
            WB: WasmBackend,
            F: Fn($(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::HostFunction {
                <WB as WasmBackend>::HostFunction:: [< new_typed_ $num _r >] (ctx.as_context_mut(), self)
            }
        }

        #[allow(non_snake_case)]
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), i32, WithEnv> for F
        where
            WB: WasmBackend,
            F: Fn(<WB as WasmBackend>::ImportCallContext<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::HostFunction {
                <WB as WasmBackend>::HostFunction:: [< new_typed_with_env_ $num _r >] (ctx.as_context_mut(), self)
            }
        }
    });
}

impl_for_each_function_signature!(impl_into_func);

macro_rules! declare_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        #[allow(non_snake_case)]
        fn [< new_typed_ $num >]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::HostFunction
            where F: Fn($(replace_with!($args -> i32),)*) + Send + Sync + 'static
        {
            let func = move |_: <WB as WasmBackend>::ImportCallContext<'_>, $($args,)*| { func($($args,)*)};
            Self:: [< new_typed_with_env_ $num >] (ctx, func)
        }

        #[allow(non_snake_case)]
        fn [< new_typed_with_env_ $num >]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::HostFunction
            where F: Fn(<WB as WasmBackend>::ImportCallContext<'_>, $(replace_with!($args -> i32),)*) + Send + Sync + 'static;

        #[allow(non_snake_case)]
        fn [< new_typed_ $num _r>]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::HostFunction
            where F: Fn($(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static
        {
            let func = move |_: <WB as WasmBackend>::ImportCallContext<'_>, $($args,)*| -> i32 { func($($args,)*)};
            Self:: [< new_typed_with_env_ $num _r >] (ctx, func)
        }

        #[allow(non_snake_case)]
        fn [< new_typed_with_env_ $num _r>]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::HostFunction
            where F: Fn(<WB as WasmBackend>::ImportCallContext<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static;
    });
}

pub trait FuncConstructor<WB: WasmBackend> {
    impl_for_each_function_signature!(declare_func_construction);
}
