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
use std::sync::Arc;

pub type ErrorHandler =
    Option<Box<dyn Fn(&HostImportError) -> Option<IValue> + Sync + Send + 'static>>;
pub type HostExportedFunc<WB> = Box<
    dyn for<'c> Fn(&mut <WB as WasmBackend>::ImportCallContext<'c>, Vec<IValue>) -> Option<IValue>
        + Sync
        + Send
        + 'static,
>;

pub type RawImportCreator<WB> =
    Arc<dyn Fn(<WB as WasmBackend>::ContextMut<'_>) -> <WB as WasmBackend>::HostFunction + Send>;

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

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum HostAPIVersion {
    V0,
    V1,
    V2,
    V3,
}

impl HostAPIVersion {
    pub fn namespace(&self) -> &'static str {
        // TODO: create a common place for these consts to use in both marine and marine-rs-sdk to use in both marine and marine-rs-sdk
        match self {
            Self::V0 => "host",
            Self::V1 => "__marine_host_api_v1",
            Self::V2 => "__marine_host_api_v2",
            Self::V3 => "__marine_host_api_v3",
        }
    }
}

pub struct MModuleConfig<WB: WasmBackend> {
    /// Import object that will be used in module instantiation process.
    pub raw_imports: HashMap<HostAPIVersion, HashMap<String, RawImportCreator<WB>>>,

    /// Imports from the host side that will be used in module instantiation process.
    pub host_imports: HashMap<HostAPIVersion, HashMap<String, HostImportDescriptor<WB>>>,

    /// WASI parameters: env variables, mapped dirs, and args
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

    pub fn with_wasi_mapped_dirs(mut self, mapped_dirs: HashMap<String, PathBuf>) -> Self {
        self.wasi_parameters.mapped_dirs = mapped_dirs;
        self
    }
}

pub struct MarineCoreConfig<WB: WasmBackend> {
    pub(crate) total_memory_limit: u64,
    pub(crate) wasm_backend: WB,
}

pub const INFINITE_MEMORY_LIMIT: u64 = u64::MAX;

impl<WB: WasmBackend> MarineCoreConfig<WB> {
    pub fn new(wasm_backend: WB, total_memory_limit: Option<u64>) -> Self {
        Self {
            total_memory_limit: total_memory_limit.unwrap_or(INFINITE_MEMORY_LIMIT),
            wasm_backend,
        }
    }
}
