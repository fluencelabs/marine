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
