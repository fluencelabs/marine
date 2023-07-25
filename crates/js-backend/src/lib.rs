/*
 * Copyright 2022 Fluence Labs Limited
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

    fn new() -> WasmBackendResult<Self> {
        Ok(Self {})
    }
}
