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

use crate::WasmtimeContextMut;
use crate::WasmtimeFunction;
use crate::WasmtimeMemory;
use crate::WasmtimeWasmBackend;

use marine_wasm_backend_traits::prelude::*;

#[derive(Clone)]
pub struct WasmtimeInstance {
    pub(crate) inner: wasmtime::Instance,
}

impl Instance<WasmtimeWasmBackend> for WasmtimeInstance {
    fn export_iter<'a>(
        &'a self,
        store: WasmtimeContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<WasmtimeWasmBackend>)> + 'a> {
        let exports = self.inner.exports(store.inner).map(|export| {
            let name = export.name();
            let export = match export.into_extern() {
                wasmtime::Extern::Memory(memory) => Export::Memory(WasmtimeMemory::new(memory)),
                wasmtime::Extern::Func(func) => Export::Function(WasmtimeFunction { inner: func }),
                _ => Export::Other,
            };
            (name, export)
        });
        Box::new(exports)
    }

    fn get_nth_memory(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        memory_index: u32,
    ) -> Option<<WasmtimeWasmBackend as WasmBackend>::Memory> {
        self.inner
            .exports(&mut store.as_context_mut().inner)
            .filter_map(wasmtime::Export::into_memory)
            .nth(memory_index as usize)
            .map(WasmtimeMemory::new)
    }

    fn get_memory(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        memory_name: &str,
    ) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::Memory> {
        self.inner
            .get_export(&mut store.as_context_mut().inner, memory_name)
            .ok_or_else(|| ResolveError::ExportNotFound(memory_name.to_string()))
            .and_then(|e| {
                e.into_memory().ok_or(ResolveError::ExportTypeMismatch {
                    expected: "memory",
                    actual: "other",
                })
            })
            .map(WasmtimeMemory::new)
    }

    fn get_function(
        &self,
        store: &mut impl AsContextMut<WasmtimeWasmBackend>,
        name: &str,
    ) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::ExportFunction> {
        let func = self
            .inner
            .get_export(&mut store.as_context_mut().inner, name)
            .ok_or_else(|| ResolveError::ExportNotFound(name.to_owned()))
            .and_then(|e| {
                e.into_func().ok_or(ResolveError::ExportTypeMismatch {
                    expected: "function",
                    actual: "other",
                })
            })?;

        Ok(WasmtimeFunction { inner: func })
    }
}
