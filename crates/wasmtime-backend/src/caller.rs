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

use crate::StoreState;
use crate::WasmtimeContext;
use crate::WasmtimeContextMut;
use crate::WasmtimeWasmBackend;
use crate::WasmtimeMemory;

use marine_wasm_backend_traits::prelude::*;

use wasmtime::AsContext as WasmtimeAsContext;
use wasmtime::AsContextMut as WasmtimeAsContextMut;

pub struct WasmtimeImportCallContext<'c> {
    pub(crate) inner: wasmtime::Caller<'c, StoreState>,
}

impl<'c> ImportCallContext<WasmtimeWasmBackend> for WasmtimeImportCallContext<'c> {
    fn memory(&mut self, _memory_index: u32) -> Option<WasmtimeMemory> {
        let memory = self
            .inner
            .get_export(STANDARD_MEMORY_EXPORT_NAME)?
            .into_memory()?;

        Some(WasmtimeMemory::new(memory))
    }
}

impl<'c> AsContext<WasmtimeWasmBackend> for WasmtimeImportCallContext<'c> {
    fn as_context(&self) -> WasmtimeContext<'_> {
        WasmtimeContext {
            inner: self.inner.as_context(),
        }
    }
}

impl<'c> AsContextMut<WasmtimeWasmBackend> for WasmtimeImportCallContext<'c> {
    fn as_context_mut(&mut self) -> <WasmtimeWasmBackend as WasmBackend>::ContextMut<'_> {
        WasmtimeContextMut {
            inner: self.inner.as_context_mut(),
        }
    }
}

/// Implements func_getter for given function signature.
/// Later `get_func` variant will be statically chosen based on types.
macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl<'c> FuncGetter<WasmtimeWasmBackend, $args, $rets> for WasmtimeImportCallContext<'c> {
            fn get_func(
                &mut self,
                name: &str,
            ) -> Result<TypedFunc<WasmtimeWasmBackend, $args, $rets>, ResolveError> {
                use futures::FutureExt;
                use std::sync::Arc;

                fn create_func_getter_closure(
                    f: Arc<wasmtime::TypedFunc<$args, $rets>>,
                ) -> impl for<'args, 'ctx2> Fn(
                    &'args mut WasmtimeContextMut<'ctx2>,
                    $args,
                ) -> TypedFuncFuture<'args, $rets>
                       + 'static {
                    move |store: &mut WasmtimeContextMut<'_>,
                          args: $args|
                          -> TypedFuncFuture<'_, $rets> {
                        let f = f.clone();
                        call_typed_func(store, args, f).boxed()
                    }
                }

                async fn call_typed_func<'args, 'ctx2>(
                    store: &'args mut WasmtimeContextMut<'ctx2>,
                    args: $args,
                    f: Arc<wasmtime::TypedFunc<$args, $rets>>,
                ) -> RuntimeResult<$rets> {
                    f.call_async(&mut store.inner, args).await.map_err(|e| {
                        if let Some(_) = e.downcast_ref::<wasmtime::Trap>() {
                            RuntimeError::Trap(e)
                        } else {
                            RuntimeError::Other(e)
                        }
                    })
                }

                let export = self
                    .inner
                    .get_export(name)
                    .ok_or(ResolveError::ExportNotFound(name.to_string()))?;

                match export {
                    wasmtime::Extern::Func(f) => {
                        let f = f
                            .typed(&mut self.inner)
                            .map_err(|e| ResolveError::Other(e))?;
                        let f = Arc::new(f);
                        let closure = create_func_getter_closure(f);

                        Ok(Arc::new(closure))
                    }
                    wasmtime::Extern::Memory(_) => Err(ResolveError::ExportTypeMismatch {
                        expected: "function",
                        actual: "memory",
                    }),
                    _ => Err(ResolveError::ExportTypeMismatch {
                        expected: "function",
                        actual: "neither memory nor function",
                    }),
                }
            }
        }
    };
}

// These signatures are sufficient for marine to work.
impl_func_getter!((i32, i32), i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!(i32, i32);
impl_func_getter!(i32, ());
impl_func_getter!((), i32);
impl_func_getter!((), ());
