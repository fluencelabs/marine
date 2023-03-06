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

use marine_wasm_backend_traits::prelude::*;
use marine_wasm_backend_traits::impl_utils::custom_sections;

use anyhow::anyhow;

mod module;
mod store;
mod instance;
mod function;
mod imports;
mod utils;
mod memory;
mod caller;
mod wasi;

use module::*;
use store::*;
use instance::*;
use function::*;
use imports::*;
use memory::*;
use caller::*;
use wasi::*;

pub(crate) use utils::*;

#[derive(Clone, Default)]
pub struct WasmerBackend {}

impl WasmBackend for WasmerBackend {
    type Module = WasmerModule;
    type Instance = WasmerInstance;
    type Store = WasmerStore;
    type Context<'c> = WasmerContext<'c>;
    type ContextMut<'c> = WasmerContextMut<'c>;
    type Caller<'c> = WasmerCaller<'c>;
    type Imports = WasmerImports;
    type Function = WasmerFunction;
    type Memory = WasmerMemory;
    type MemoryView = WasmerMemory;
    type Wasi = WasmerWasi;

    fn compile(store: &mut Self::Store, wasm: &[u8]) -> CompilationResult<Self::Module> {
        wasmer::Module::new(store.inner.engine(), wasm)
            .map_err(|e| {
                CompilationError::Other(anyhow!("Wasmer module compilation failed: {}", e))
                // TODO make detailed
            })
            .and_then(|module| {
                let custom_sections = custom_sections(wasm).map_err(|e| {
                    // TODO: avoid double module parsing
                    CompilationError::Other(anyhow!("{}", e)) // TODO make detailed
                })?;
                Ok(WasmerModule {
                    inner: module,
                    custom_sections,
                })
            })
    }

    fn new() -> Self {
        <_>::default()
    }
}