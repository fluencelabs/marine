extern crate core;

use marine_wasm_backend_traits::prelude::*;
use crate::caller::JsCaller;
use crate::function::JsFunction;

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

use web_sys::console;

use crate::store::JsContextMut;
use crate::store::JsStore;
use crate::module::JsModule;
use crate::store::JsContext;
use crate::imports::JsImports;
use crate::instance::JsInstance;
use crate::memory::JsMemory;
use crate::wasi::JsWasi;

#[derive(Default, Clone)]
pub struct JsWasmBackend {}

impl WasmBackend for JsWasmBackend {
    type Store = JsStore;
    type Module = JsModule;
    type Imports = JsImports;
    type Instance = JsInstance;
    type Context<'c> = JsContext<'c>;
    type ContextMut<'c> = JsContextMut<'c>;
    type Caller<'c> = JsCaller<'c>;
    type Function = JsFunction;
    type Memory = JsMemory;
    type MemoryView = JsMemory;
    type Wasi = JsWasi;

    fn new() -> WasmBackendResult<Self> {
        Ok(Self {})
    }
}
