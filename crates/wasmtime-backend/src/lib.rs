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

mod caller;
mod store;
mod utils;
mod module;
mod instance;
mod wasi;
mod function;
mod imports;
mod memory;

use store::*;
use caller::*;
use module::*;
use instance::*;
use wasi::*;
use function::*;
use memory::*;
use imports::*;
use utils::*;

use marine_wasm_backend_traits::prelude::*;

use wasmtime_wasi::WasiCtx;

const MB: usize = 1024 * 1024;

/// Default amount of stack space available for executing WebAssembly code.
pub const DEFAULT_WASM_STACK_SIZE: usize = 2 * MB;

#[derive(Clone)]
pub struct WasmtimeWasmBackend {
    engine: wasmtime::Engine,
}

impl WasmBackend for WasmtimeWasmBackend {
    type Store = WasmtimeStore;
    type Module = WasmtimeModule;
    type Imports = WasmtimeImports;
    type Instance = WasmtimeInstance;
    type Context<'c> = WasmtimeContext<'c>;
    type ContextMut<'c> = WasmtimeContextMut<'c>;
    type ImportCallContext<'c> = WasmtimeImportCallContext<'c>;
    type HostFunction = WasmtimeFunction;
    type ExportFunction = WasmtimeFunction;
    type Memory = WasmtimeMemory;
    type MemoryView = WasmtimeMemory;
    type Wasi = WasmtimeWasi;

    fn new_async() -> WasmBackendResult<Self> {
        Self::new(WasmtimeConfig::new())
    }
}

impl WasmtimeWasmBackend {
    pub fn increment_epoch(&self) {
        self.engine.increment_epoch()
    }

    pub fn new(config: WasmtimeConfig) -> WasmBackendResult<Self> {
        let engine =
            wasmtime::Engine::new(&config.config).map_err(WasmBackendError::InitializationError)?;

        Ok(Self { engine })
    }
}

#[derive(Default)]
pub struct StoreState {
    wasi: Vec<WasiCtx>, // wasmtime store does not release memory until drop, so do we
    limits: MemoryLimiter,
}

#[derive(Clone)]
pub struct WasmtimeConfig {
    config: wasmtime::Config,
}

impl Default for WasmtimeConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmtimeConfig {
    pub fn new() -> Self {
        let mut config = wasmtime::Config::default();
        config
            .async_support(true)
            .debug_info(true)
            .max_wasm_stack(DEFAULT_WASM_STACK_SIZE)
            .epoch_interruption(true)
            .wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);

        Self { config }
    }

    /// Constructs wasmtime config directly from wasmtime config.
    /// It forcefully enables async support, because the backend does not work with sync configs.
    pub fn from_raw(mut config: wasmtime::Config) -> Self {
        config.async_support(true);
        Self { config }
    }

    /// Configures whether DWARF debug information will be emitted during
    /// compilation.
    ///
    /// By default this option is `true`.
    pub fn debug_info(&mut self, enable: bool) -> &mut Self {
        self.config.debug_info(enable);
        self
    }

    /// Enables the epoch interruption mechanism. See Wasmtime docs for detailed explanation.
    ///
    /// By default this option is `true`.
    pub fn epoch_interruption(&mut self, enable: bool) -> &mut Self {
        self.config.epoch_interruption(enable);
        self
    }

    /// Configures the maximum amount of stack space available for
    /// executing WebAssembly code.
    ///
    /// By default this option is 2 MiB.
    pub fn max_wasm_stack(&mut self, size: usize) -> &mut Self {
        self.config.max_wasm_stack(size);
        self
    }

    /// Configures the size of the stacks used for asynchronous execution.
    ///
    /// This setting configures the size of the stacks that are allocated for
    /// asynchronous execution. The value cannot be less than `max_wasm_stack`.
    ///
    /// By default this option is 2 MiB.
    pub fn async_wasm_stack(&mut self, size: usize) -> &mut Self {
        self.config.async_stack_size(size);
        self
    }

    /// Configures whether the errors from the VM should collect the wasm backtrace and parse debug info.
    ///
    /// By default this option is `true`.
    pub fn wasm_backtrace(&mut self, enable: bool) -> &mut Self {
        self.config
            .wasm_backtrace(enable)
            .wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        self
    }
}
