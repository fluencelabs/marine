/*
 * Copyright 2020 Fluence Labs Limited
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

use super::IValue;
use super::IType;
use crate::HostImportError;

use marine_wasm_backend_traits::WasiParameters;
use marine_wasm_backend_traits::WasmBackend;

use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::HashSet;

pub type ErrorHandler =
    Option<Box<dyn Fn(&HostImportError) -> Option<IValue> + Sync + Send + 'static>>;
pub type HostExportedFunc<WB> = Box<
    dyn for<'c> Fn(&mut <WB as WasmBackend>::ImportCallContext<'c>, Vec<IValue>) -> Option<IValue>
        + Sync
        + Send
        + 'static,
>;

pub type RawImportCreator<WB> =
    Box<dyn FnOnce(<WB as WasmBackend>::ContextMut<'_>) -> <WB as WasmBackend>::HostFunction>;

pub struct HostImportDescriptor<WB: WasmBackend> {
    /// This closure will be invoked for corresponding import.
    pub host_exported_func: HostExportedFunc<WB>,

    /// Type of the closure arguments.
    pub argument_types: Vec<IType>,

    /// Types of output of the closure.
    pub output_type: Option<IType>,

    /// If Some, this closure is called with error when errors is encountered while lifting.
    /// If None, panic will occur.
    pub error_handler: ErrorHandler,
}

pub struct MModuleConfig<WB: WasmBackend> {
    /// Import object that will be used in module instantiation process.
    pub raw_imports: HashMap<String, RawImportCreator<WB>>,

    /// Imports from the host side that will be used in module instantiation process.
    pub host_imports: HashMap<String, HostImportDescriptor<WB>>,

    /// WASI parameters: env variables, mapped dirs, preopened files and args
    pub wasi_parameters: WasiParameters,
}

impl<WB: WasmBackend> Default for MModuleConfig<WB> {
    fn default() -> Self {
        // some reasonable defaults
        Self {
            raw_imports: HashMap::new(),
            host_imports: HashMap::new(),
            wasi_parameters: WasiParameters::default(),
        }
    }
}

// TODO: implement debug for MModuleConfig

#[allow(dead_code)]
impl<WB: WasmBackend> MModuleConfig<WB> {
    pub fn with_wasi_envs(mut self, envs: HashMap<String, String>) -> Self {
        self.wasi_parameters.envs = envs;
        self
    }

    pub fn with_wasi_preopened_files(mut self, preopened_files: HashSet<PathBuf>) -> Self {
        self.wasi_parameters.preopened_files = preopened_files;
        self
    }

    pub fn with_wasi_mapped_dirs(mut self, mapped_dirs: HashMap<String, PathBuf>) -> Self {
        self.wasi_parameters.mapped_dirs = mapped_dirs;
        self
    }
}

pub struct MarineCoreConfig {
    pub(crate) total_memory_limit: u64,
}

impl Default for MarineCoreConfig {
    fn default() -> Self {
        Self {
            total_memory_limit: INFINITE_MEMORY_LIMIT,
        }
    }
}

pub const INFINITE_MEMORY_LIMIT: u64 = u64::MAX;

#[derive(Default, Debug)]
pub struct MarineCoreConfigBuilder {
    total_memory_limit: Option<u64>,
}

impl MarineCoreConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total_memory_limit(mut self, total_memory_limit: u64) -> Self {
        self.total_memory_limit = Some(total_memory_limit);
        self
    }

    pub fn build(self) -> MarineCoreConfig {
        MarineCoreConfig {
            total_memory_limit: self.total_memory_limit.unwrap_or(INFINITE_MEMORY_LIMIT),
        }
    }
}
