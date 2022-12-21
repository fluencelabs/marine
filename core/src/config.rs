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

use marine_wasm_backend_traits::{WasmBackend};
use marine_wasm_backend_traits::Namespace;
use marine_wasm_backend_traits::WasiVersion;

use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::HashSet;

// 65536*1600 ~ 100 Mb (Wasm page size is 64 Kb)
const DEFAULT_HEAP_PAGES_COUNT: u32 = 1600;

pub type ErrorHandler = Option<Box<dyn Fn(&HostImportError) -> Option<IValue> + 'static>>;
pub type HostExportedFunc<WB> =
    Box<dyn for<'c> Fn(&mut <WB as WasmBackend>::Caller<'c>, Vec<IValue>) -> Option<IValue> + 'static>;

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
    /// Maximum number of Wasm memory pages that loaded module can use.
    /// Each Wasm page is 65536 bytes long.
    pub max_heap_pages_count: u32,

    /// Import object that will be used in module instantiation process.
    pub raw_imports: <WB as WasmBackend>::Namespace,

    /// Imports from the host side that will be used in module instantiation process.
    pub host_imports: HashMap<String, HostImportDescriptor<WB>>,

    /// Desired WASI version.
    pub wasi_version: WasiVersion,

    /// Environment variables for loaded modules.
    pub wasi_envs: HashMap<Vec<u8>, Vec<u8>>,

    /// List of available directories for loaded modules.
    pub wasi_preopened_files: HashSet<PathBuf>,

    /// Mapping between paths.
    pub wasi_mapped_dirs: HashMap<String, PathBuf>,
}

impl<WB: WasmBackend> Default for MModuleConfig<WB> {
    fn default() -> Self {
        // some reasonable defaults
        Self {
            max_heap_pages_count: DEFAULT_HEAP_PAGES_COUNT,
            raw_imports: <WB as WasmBackend>::Namespace::new(),
            host_imports: HashMap::new(),
            wasi_version: WasiVersion::Latest,
            wasi_envs: HashMap::new(),
            wasi_preopened_files: HashSet::new(),
            wasi_mapped_dirs: HashMap::new(),
        }
    }
}

// TODO: implement debug for MModuleConfig

#[allow(dead_code)]
impl<WB: WasmBackend> MModuleConfig<WB> {
    pub fn with_mem_pages_count(mut self, mem_pages_count: u32) -> Self {
        self.max_heap_pages_count = mem_pages_count;
        self
    }

    pub fn with_wasi_version(mut self, wasi_version: WasiVersion) -> Self {
        self.wasi_version = wasi_version;
        self
    }

    pub fn with_wasi_envs(mut self, envs: HashMap<Vec<u8>, Vec<u8>>) -> Self {
        self.wasi_envs = envs;
        self
    }

    pub fn with_wasi_preopened_files(mut self, preopened_files: HashSet<PathBuf>) -> Self {
        self.wasi_preopened_files = preopened_files;
        self
    }

    pub fn with_wasi_mapped_dirs(mut self, mapped_dirs: HashMap<String, PathBuf>) -> Self {
        self.wasi_mapped_dirs = mapped_dirs;
        self
    }
}
