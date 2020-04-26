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

use wasmer_wasi::WasiVersion;

use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct WASIConfig {
    /// Desired WASI version.
    pub version: WasiVersion,

    /// Environment variables for loaded modules.
    pub envs: Vec<Vec<u8>>,

    /// List of available directories for loaded modules.
    pub preopened_files: Vec<PathBuf>,

    /// Mapping between paths.
    pub mapped_dirs: Vec<(String, PathBuf)>,
}

impl Default for WASIConfig {
    fn default() -> Self {
        Self {
            version: WasiVersion::Latest,
            envs: vec![],
            preopened_files: vec![],
            mapped_dirs: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// Maximum number of Wasm memory pages that loaded module can use.
    /// Each Wasm pages is 65536 bytes long.
    pub mem_pages_count: u32,

    /// If true, registers the logger Wasm module with name 'logger'.
    /// This functionality is just for debugging, and this module will be disabled in future.
    pub logger_enabled: bool,

    /// The name of the main module handler function.
    pub invoke_fn_name: String,

    /// The name of function that should be called for allocation memory. This function
    /// is used for passing array of bytes to the main module.
    pub allocate_fn_name: String,

    /// The name of function that should be called for deallocation of
    /// previously allocated memory by allocateFunction.
    pub deallocate_fn_name: String,

    /// Config for WASI subsystem initialization. None means that module should be loaded
    /// without WASI.
    pub wasi_config: WASIConfig,
}

impl Default for Config {
    fn default() -> Self {
        // some reasonable defaults
        Self {
            // 65536*1600 ~ 100 Mb
            mem_pages_count: 1600,
            invoke_fn_name: "invoke".to_string(),
            allocate_fn_name: "allocate".to_string(),
            deallocate_fn_name: "deallocate".to_string(),
            logger_enabled: true,
            wasi_config: WASIConfig::default(),
        }
    }
}

impl Config {
    #[allow(dead_code)]
    pub fn with_mem_pages_count(mut self, mem_pages_count: u32) -> Self {
        self.mem_pages_count = mem_pages_count;
        self
    }

    #[allow(dead_code)]
    pub fn with_invoke_fn_name(mut self, invoke_fn_name: String) -> Self {
        self.invoke_fn_name = invoke_fn_name;
        self
    }

    #[allow(dead_code)]
    pub fn with_allocate_fn_name(mut self, allocate_fn_name: String) -> Self {
        self.allocate_fn_name = allocate_fn_name;
        self
    }

    #[allow(dead_code)]
    pub fn with_deallocate_fn_name(mut self, deallocate_fn_name: String) -> Self {
        self.deallocate_fn_name = deallocate_fn_name;
        self
    }

    #[allow(dead_code)]
    pub fn with_logger_enable(mut self, logger_enable: bool) -> Self {
        self.logger_enabled = logger_enable;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasi_version(mut self, wasi_version: WasiVersion) -> Self {
        self.wasi_config.version = wasi_version;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasi_envs(mut self, envs: Vec<Vec<u8>>) -> Self {
        self.wasi_config.envs = envs;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasi_preopened_files(mut self, preopened_files: Vec<PathBuf>) -> Self {
        self.wasi_config.preopened_files = preopened_files;
        self
    }

    #[allow(dead_code)]
    pub fn with_wasi_mapped_dirs(mut self, mapped_dirs: Vec<(String, PathBuf)>) -> Self {
        self.wasi_config.mapped_dirs = mapped_dirs;
        self
    }
}
