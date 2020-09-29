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

use fce::HostImportDescriptor;

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

/// Describes the behaviour of FluenceFaaS.
#[derive(Default)]
pub struct FaaSConfig {
    /// Path to a dir where compiled Wasm modules are located.
    pub modules_dir: Option<String>,

    /// Settings for a module with particular name.
    pub modules_config: HashMap<String, FaaSModuleConfig>,

    /// Settings for a module that name's not been found in modules_config.
    pub default_modules_config: Option<FaaSModuleConfig>,
}

/// Various settings that could be used to guide FCE how to load a module in a proper way.
#[derive(Default)]
pub struct FaaSModuleConfig {
    /// Maximum memory size accessible by a module in Wasm pages (64 Kb).
    pub mem_pages_count: Option<u32>,

    /// Defines whether FaaS should provide a special host log_utf8_string function for this module.
    pub logger_enabled: bool,

    /// Export from host functions that will be accessible on the Wasm side by provided name.
    pub host_imports: HashMap<String, HostImportDescriptor>,

    /// A WASI config.
    pub wasi: Option<FaaSWASIConfig>,
}

impl FaaSModuleConfig {
    pub fn extend_wasi_envs(mut self, new_envs: HashMap<Vec<u8>, Vec<u8>>) -> Self {
        match &mut self.wasi {
            Some(FaaSWASIConfig { envs, .. }) => envs.extend(new_envs),
            w @ None => {
                *w = Some(FaaSWASIConfig {
                    envs: new_envs,
                    preopened_files: HashSet::new(),
                    mapped_dirs: HashMap::new(),
                })
            }
        }

        self
    }

    #[rustfmt::skip]
    pub fn extend_wasi_files(
        mut self,
        new_preopened_files: HashSet<PathBuf>,
        new_mapped_dirs: HashMap<String, PathBuf>,
    ) -> Self {
        match &mut self.wasi {
            Some(FaaSWASIConfig {
                     preopened_files,
                     mapped_dirs,
                     ..
                 }) => {
                    preopened_files.extend(new_preopened_files);
                    mapped_dirs.extend(new_mapped_dirs);
            },
            w @ None => {
                *w = Some(FaaSWASIConfig {
                    envs: HashMap::new(),
                    preopened_files: new_preopened_files,
                    mapped_dirs: new_mapped_dirs,
                })
            }
        }

        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct FaaSWASIConfig {
    /// A list of environment variables available for this module.
    pub envs: HashMap<Vec<u8>, Vec<u8>>,

    /// A list of files available for this module.
    /// A loaded module could have access only to files from this list.
    pub preopened_files: HashSet<PathBuf>,

    /// Mapping from a usually short to full file name.
    pub mapped_dirs: HashMap<String, PathBuf>,
}
