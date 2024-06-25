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

mod store;
mod module;
mod imports;
mod instance;
mod caller;
mod function;
mod memory;
mod wasi;
mod module_info;
mod js_conversions;

mod single_shot_async_executor;

use crate::store::JsContextMut;
use crate::store::JsStore;
use crate::module::JsModule;
use crate::store::JsContext;
use crate::imports::JsImports;
use crate::instance::JsInstance;
use crate::memory::JsMemory;
use crate::wasi::JsWasi;
use crate::caller::JsImportCallContext;
use crate::function::HostImportFunction;
use crate::function::WasmExportFunction;

use marine_wasm_backend_traits::prelude::*;

#[derive(Default, Clone)]
pub struct JsWasmBackend {}

impl WasmBackend for JsWasmBackend {
    type Store = JsStore;
    type Module = JsModule;
    type Imports = JsImports;
    type Instance = JsInstance;
    type Context<'c> = JsContext<'c>;
    type ContextMut<'c> = JsContextMut<'c>;
    type ImportCallContext<'c> = JsImportCallContext;
    type HostFunction = HostImportFunction;
    type ExportFunction = WasmExportFunction;
    type Memory = JsMemory;
    type MemoryView = JsMemory;
    type Wasi = JsWasi;

    fn new_async() -> WasmBackendResult<Self> {
        Ok(Self {})
    }
}
