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

use wasmer_wasi::WasiVersion;
use wasmer_runtime::ImportObject;

use std::path::PathBuf;
use std::collections::HashMap;

pub struct HostImportDescriptor {
    /// This closure will be invoked for corresponding import.
    pub closure: Box<dyn Fn(Vec<IValue>) -> Option<IValue> + 'static>,

    /// Type of the closure arguments.
    pub argument_types: Vec<IType>,

    /// Types of output of the closure.
    pub output_type: Option<IType>,

    /// If Some, this closure is called with error when errors is encountered while lifting.
    /// If None, panic will occur.
    pub error_handler: Option<Box<dyn Fn(&HostImportError) -> Option<IValue> + 'static>>,
}

pub struct FCEModuleConfig {
    /// Maximum number of Wasm memory pages that loaded module can use.
    /// Each Wasm pages is 65536 bytes long.
    pub mem_pages_count: u32,

    /// Import object that will be used in module instantiation process.
    pub raw_imports: ImportObject,

    /// Imports that will be used in module instantiation process.
    pub imports: HashMap<String, HostImportDescriptor>,

    /// Desired WASI version.
    pub wasi_version: WasiVersion,

    /// Environment variables for loaded modules.
    pub wasi_envs: Vec<Vec<u8>>,

    /// List of available directories for loaded modules.
    pub wasi_preopened_files: Vec<PathBuf>,

    /// Mapping between paths.
    pub wasi_mapped_dirs: Vec<(String, PathBuf)>,
}

impl Default for FCEModuleConfig {
    fn default() -> Self {
        // some reasonable defaults
        Self {
            // 65536*1600 ~ 100 Mb
            mem_pages_count: 1600,
            raw_imports: ImportObject::new(),
            imports: <_>::default(),
            wasi_version: WasiVersion::Latest,
            wasi_envs: vec![],
            wasi_preopened_files: vec![],
            wasi_mapped_dirs: vec![],
        }
    }
}

// TODO: implement debug for FCEModuleConfig

impl FCEModuleConfig {
    #[allow(dead_code)]
    pub fn with_mem_pages_count(mut self, mem_pages_count: u32) -> Self {
        self.mem_pages_count = mem_pages_count;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasi_version(mut self, wasi_version: WasiVersion) -> Self {
        self.wasi_version = wasi_version;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasi_envs(mut self, envs: Vec<Vec<u8>>) -> Self {
        self.wasi_envs = envs;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasi_preopened_files(mut self, preopened_files: Vec<PathBuf>) -> Self {
        self.wasi_preopened_files = preopened_files;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasi_mapped_dirs(mut self, mapped_dirs: Vec<(String, PathBuf)>) -> Self {
        self.wasi_mapped_dirs = mapped_dirs;
        self
    }
}
