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

use crate::StoreState;
use crate::WasmtimeContext;
use crate::WasmtimeContextMut;
use crate::WasmtimeWasmBackend;
use crate::WasmtimeMemory;

use marine_wasm_backend_traits::prelude::*;

use wasmtime::AsContext as WasmtimeAsContext;
use wasmtime::AsContextMut as WasmtimeAsContextMut;

pub struct WasmtimeCaller<'c> {
    pub(crate) inner: wasmtime::Caller<'c, StoreState>,
}

impl<'c> Caller<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn memory(&mut self, _memory_index: u32) -> Option<WasmtimeMemory> {
        let memory = self
            .inner
            .get_export(STANDARD_MEMORY_EXPORT_NAME)?
            .into_memory()?;

        Some(WasmtimeMemory::new(memory))
    }
}

impl<'c> AsContext<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn as_context(&self) -> WasmtimeContext<'_> {
        WasmtimeContext {
            inner: self.inner.as_context(),
        }
    }
}

impl<'c> AsContextMut<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
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
        impl<'c> FuncGetter<WasmtimeWasmBackend, $args, $rets> for WasmtimeCaller<'c> {
            fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(&mut WasmtimeContextMut<'_>, $args) -> Result<$rets, RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let export = self
                    .inner
                    .get_export(name)
                    .ok_or(ResolveError::ExportNotFound(name.to_string()))?;

                match export {
                    wasmtime::Extern::Func(f) => {
                        let f = f
                            .typed(&mut self.inner)
                            .map_err(|e| ResolveError::Other(e))?;

                        let closure = move |store: &mut WasmtimeContextMut<'_>, args| {
                            f.call(&mut store.inner, args).map_err(|e| {
                                if let Some(_) = e.downcast_ref::<wasmtime::Trap>() {
                                    RuntimeError::Trap(e)
                                } else {
                                    RuntimeError::Other(e)
                                }
                            })
                        };

                        Ok(Box::new(closure))
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
