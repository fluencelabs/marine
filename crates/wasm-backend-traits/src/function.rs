use crate::{AsContextMut, CallResult, FuncSig, impl_for_each_function_signature, WasmBackend, WasmType, WValue};

pub trait Function<WB: WasmBackend>: Send + Sync {
    fn new<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(&[WValue]) -> Vec<WValue> + Sync + Send + 'static;

    fn new_with_ctx<F>(store: &mut impl AsContextMut<WB>, sig: FuncSig, func: F) -> Self
    where
        F: for<'c> Fn(<WB as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static;

    fn new_typed<Params, Results, Env>(store: &mut impl AsContextMut<WB>, func: impl IntoFunc<WB, Params, Results, Env>) -> Self;

    fn signature<'c>(&self, store: &mut impl AsContextMut<WB>) -> &FuncSig;

    fn call<'c>(
        &self,
        store: &mut impl AsContextMut<WB>, // <- Store or ExportContext. Need to be able to extract wasmtime::StoreContextMut from them. Same for many methods.
        args: &[WValue],
    ) -> CallResult<Vec<WValue>>;
}

pub trait IntoFunc<WB: WasmBackend, Params, Results, Env> {
    fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function;
}

pub struct WithEnv {}
pub struct WithoutEnv {}

#[macro_export]
macro_rules! replace_with {
    ($from:ident -> $to:ident) => {$to};
}


macro_rules! impl_into_func {
    ($num:tt $($args:ident)*) => (paste::paste!{
        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), (), WithoutEnv> for F
        where
            WB: WasmBackend,
            F: Fn($(replace_with!($args -> i32),)*) -> () + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function {
                <WB as WasmBackend>::Function:: [< new_typed_ $num >] (ctx.as_context_mut(), self)
            }
        }

        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), (), WithEnv> for F
        where
            WB: WasmBackend,
            F: Fn(<WB as WasmBackend>::Caller<'_>, $(replace_with!($args -> i32),)*) -> () + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function {
                <WB as WasmBackend>::Function:: [< new_typed_with_env_ $num >] (ctx.as_context_mut(), self)
            }
        }

        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), i32, WithoutEnv> for F
        where
            WB: WasmBackend,
            F: Fn($(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function {
                <WB as WasmBackend>::Function:: [< new_typed_ $num _r >] (ctx.as_context_mut(), self)
            }
        }

        impl<WB, F> IntoFunc<WB, ($(replace_with!($args -> i32),)*), i32, WithEnv> for F
        where
            WB: WasmBackend,
            F: Fn(<WB as WasmBackend>::Caller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static,
        {
            fn into_func(self, ctx: &mut impl AsContextMut<WB>) -> <WB as WasmBackend>::Function {
                <WB as WasmBackend>::Function:: [< new_typed_with_env_ $num _r >] (ctx.as_context_mut(), self)
            }
        }
    });
}

impl_for_each_function_signature!(impl_into_func);

macro_rules! declare_func_construction {
    ($num:tt $($args:ident)*) => (paste::paste!{
        fn [< new_typed_ $num >]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
            where F: Fn($(replace_with!($args -> i32),)*) -> () + Send + Sync + 'static
        {
            let func = move |_: <WB as WasmBackend>::Caller<'_>, $($args,)*| { func($($args,)*)};
            Self:: [< new_typed_with_env_ $num >] (ctx, func)
        }

        fn [< new_typed_with_env_ $num >]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
            where F: Fn(<WB as WasmBackend>::Caller<'_>, $(replace_with!($args -> i32),)*) -> () + Send + Sync + 'static;

        fn [< new_typed_ $num _r>]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
            where F: Fn($(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static
        {
            let func = move |_: <WB as WasmBackend>::Caller<'_>, $($args,)*| -> i32 { func($($args,)*)};
            Self:: [< new_typed_with_env_ $num _r >] (ctx, func)
        }

        fn [< new_typed_with_env_ $num _r>]<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
            where F: Fn(<WB as WasmBackend>::Caller<'_>, $(replace_with!($args -> i32),)*) -> i32 + Send + Sync + 'static;
    });
}

pub trait FuncConstructor<WB: WasmBackend> {
    fn new_typed_with_env_0_test<F>(ctx: <WB as WasmBackend>::ContextMut<'_>, func: F) -> <WB as WasmBackend>::Function
    where F: Fn(<WB as WasmBackend>::Caller<'_>) -> () + Send + Sync + 'static;


    impl_for_each_function_signature!(declare_func_construction);
}



