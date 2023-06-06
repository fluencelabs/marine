use std::marker::PhantomData;
use marine_wasm_backend_traits::prelude::*;
use marine_wasm_backend_traits::impl_for_each_function_signature;
use marine_wasm_backend_traits::replace_with;

use crate::{JsInstance, JsWasmBackend};
use crate::JsContext;
use crate::JsContextMut;

pub struct JsCaller<'c> {
    pub(crate) store_inner: *mut crate::store::JsStoreInner,
    pub(crate) caller_instance: Option<JsInstance>,
    pub(crate) _data: PhantomData<&'c i32>,
}

impl<'c> Caller<JsWasmBackend> for JsCaller<'c> {
    fn memory(&mut self, memory_index: u32) -> Option<<JsWasmBackend as WasmBackend>::Memory> {
        self.caller_instance
            .clone()
            .and_then(|instance| instance.get_nth_memory(&mut self.as_context_mut(), 0))
    }
}

impl<'c> AsContext<JsWasmBackend> for JsCaller<'c> {
    fn as_context(&self) -> <JsWasmBackend as WasmBackend>::Context<'_> {
        JsContext::from_raw_ptr(self.store_inner)
    }
}

impl<'c> AsContextMut<JsWasmBackend> for JsCaller<'c> {
    fn as_context_mut(&mut self) -> <JsWasmBackend as WasmBackend>::ContextMut<'_> {
        JsContextMut::from_raw_ptr(self.store_inner)
    }
}

/// Implements func_getter for given function signature.
/// Later `get_func` variant will be statically chosen based on types.
/*
macro_rules! impl_func_getter {
    ($($arg:ty,)*, $rets:ty) => {
        impl<'c> FuncGetter<JsWasmBackend, $args, $rets> for JsCaller<'c> {
            fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(&mut JsContextMut<'_>, $args) -> Result<$rets, RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let store = JsContextMut::from_raw_ptr(self.store_inner)
                let func = self
                    .caller_instance
                    .ok_or(|| ResolveError::NotFound)?
                    .get_function(name)?;

                let func = move |args: $args| -> Result<$rets, RuntimeError> {

                    let results = func.call(&args)?
                    Ok(results)

                }


            }
        }
    };
}*/

/// Generates a function that accepts a Fn with $num template parameters and turns it into WasmtimeFunction.
/// Needed to allow users to pass almost any function to `Function::new_typed` without worrying about signature.
macro_rules! impl_func_getter_2 {
    ($num:tt $($args:ident)*) => (paste::paste!{
        impl<'c> FuncGetter<JsWasmBackend, ($(replace_with!($args -> i32)),*), ()> for JsCaller<'c> {
            fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(&mut JsContextMut<'_>, ($(replace_with!($args -> i32)),*)) -> Result<(), RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let mut store = JsContextMut::from_raw_ptr(self.store_inner);
                let func = self
                    .caller_instance
                    .as_ref()
                    .ok_or_else(|| ResolveError::ExportNotFound(name.to_string()))?
                    .get_function(&mut store, name)?;

                let func = move |store: &mut JsContextMut<'_>, ($($args),*)| -> Result<(), RuntimeError> {
                    let args: [WValue; $num] = [$(Into::<WValue>::into($args)),*];
                    func.call(store, &args)?;
                    Ok(())
                };

                Ok(Box::new(func))
            }
        }

        impl<'c> FuncGetter<JsWasmBackend, ($(replace_with!($args -> i32)),*), i32> for JsCaller<'c> {
            fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(&mut JsContextMut<'_>, ($(replace_with!($args -> i32)),*)) -> Result<i32, RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let mut store = JsContextMut::from_raw_ptr(self.store_inner);
                let func = self
                    .caller_instance
                    .as_ref()
                    .ok_or_else(|| ResolveError::ExportNotFound(name.to_string()))?
                    .get_function(&mut store, name)?;

                let func = move |store: &mut JsContextMut<'_>, ($($args),*)| -> Result<i32, RuntimeError> {
                    let args: [WValue; $num] = [$(Into::<WValue>::into($args)),*];
                    let res = func.call(store, &args)?;
                    Ok(res[0].to_i32())
                };

                Ok(Box::new(func))
            }
        }
    });
}

impl_for_each_function_signature!(impl_func_getter_2);
/*

// These signatures are sufficient for marine to work.
impl_func_getter!((i32, i32), i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!(i32, i32);
impl_func_getter!(i32, ());
impl_func_getter!((), i32);
impl_func_getter!((), ());
*/