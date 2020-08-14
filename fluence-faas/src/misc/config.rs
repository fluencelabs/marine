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

use crate::FaaSError;
use crate::Result;

use serde_derive::Serialize;
use serde_derive::Deserialize;

use std::convert::TryInto;
use std::path::PathBuf;

/*
An example of the config:

modules_dir = "wasm/artifacts/wasm_modules"
service_base_dir = "/Users/user/tmp"

[[module]]
    name = "ipfs_node.wasm"
    mem_pages_count = 100
    logger_enabled = true

    [module.imports]
    mysql = "/usr/bin/mysql"
    ipfs = "/usr/local/bin/ipfs"

    [module.wasi]
    envs = []
    preopened_files = ["/Users/user/tmp"]
    mapped_dirs = ["tmp" = "/Users/user/tmp"]

[default]
    mem_pages_count = 100
    logger_enabled = true

    [default.imports]
    mysql = "/usr/bin/mysql"
    ipfs = "/usr/local/bin/ipfs"

    [default.wasi]
    envs = []
    preopened_files = ["/Users/user/tmp"]
    mapped_dirs = ["tmp" = "/Users/user/tmp"]
 */

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RawModulesConfig {
    pub modules_dir: Option<String>,
    pub service_base_dir: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub module: Vec<RawModuleConfig>,
    pub default: Option<RawDefaultModuleConfig>,
}

impl RawModulesConfig {
    /// Load config from filesystem
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        let bytes = std::fs::read(&path)?;
        toml::from_slice(bytes.as_slice()).map_err(|e| {
            FaaSError::ConfigParseError(format!("Error parsing config {:?}: {:?}", path, e))
        })
    }
}

impl TryInto<ModulesConfig> for RawModulesConfig {
    type Error = FaaSError;

    fn try_into(self) -> Result<ModulesConfig> {
        from_raw_modules_config(self)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RawModuleConfig {
    pub name: String,
    pub mem_pages_count: Option<u32>,
    pub logger_enabled: Option<bool>,
    pub imports: Option<toml::value::Table>,
    pub wasi: Option<RawWASIConfig>,
}

impl TryInto<ModuleConfig> for RawModuleConfig {
    type Error = FaaSError;

    fn try_into(self) -> Result<ModuleConfig> {
        from_raw_module_config(self).map(|(_, module_config)| module_config)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RawDefaultModuleConfig {
    pub mem_pages_count: Option<u32>,
    pub logger_enabled: Option<bool>,
    pub imports: Option<toml::value::Table>,
    pub wasi: Option<RawWASIConfig>,
}

impl RawModuleConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            mem_pages_count: None,
            logger_enabled: None,
            imports: None,
            wasi: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RawWASIConfig {
    pub envs: Option<Vec<String>>,
    pub preopened_files: Option<Vec<String>>,
    pub mapped_dirs: Option<toml::value::Table>,
}

/// Describes behaviour of all modules from a node.
#[derive(Debug, Clone, Default)]
pub struct ModulesConfig {
    /// Used for preparing filesystem on the service initialization stage.
    pub service_base_dir: Option<String>,

    /// Path to a dir where compiled Wasm modules are located.
    pub modules_dir: Option<String>,

    /// Settings for a module with particular name.
    pub modules_config: Vec<(String, ModuleConfig)>,

    /// Settings for a module that name's not been found in modules_config.
    pub default_modules_config: Option<ModuleConfig>,
}

/// Various settings that could be used to guide FCE how to load a module in a proper way.
#[derive(Debug, Clone, Default)]
pub struct ModuleConfig {
    /// Maximum memory size accessible by a module in Wasm pages (64 Kb).
    pub mem_pages_count: Option<u32>,

    /// Defines whether FaaS should provide a special host log_utf8_string function for this module.
    pub logger_enabled: bool,

    /// A list of CLI host imports that should be provided for this module.
    pub imports: Option<Vec<(String, String)>>,

    /// A WASI config.
    pub wasi: Option<WASIConfig>,
}

impl ModuleConfig {
    pub fn extend_wasi_envs(mut self, new_envs: Vec<Vec<u8>>) -> Self {
        match &mut self.wasi {
            Some(WASIConfig {
                envs: Some(envs), ..
            }) => envs.extend(new_envs),
            Some(w @ WASIConfig { envs: None, .. }) => w.envs = Some(new_envs),
            w @ None => {
                *w = Some(WASIConfig {
                    envs: Some(new_envs),
                    preopened_files: None,
                    mapped_dirs: None,
                })
            }
        }

        self
    }

    #[rustfmt::skip]
    pub fn extend_wasi_files(
        mut self,
        new_preopened_files: Vec<String>,
        new_mapped_dirs: Vec<(String, String)>,
    ) -> Self {
        match &mut self.wasi {
            Some(WASIConfig {
                preopened_files,
                mapped_dirs,
                ..
            }) => {
                match preopened_files {
                    Some(files) => files.extend(new_preopened_files),
                    f @ None => *f = Some(new_preopened_files),
                };
                match mapped_dirs {
                    Some(dirs) => dirs.extend(new_mapped_dirs),
                    d @ None => *d = Some(new_mapped_dirs),
                };
            },
            w @ None => {
                *w = Some(WASIConfig {
                    envs: None,
                    preopened_files: Some(new_preopened_files),
                    mapped_dirs: Some(new_mapped_dirs),
                })
            }
        }

        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct WASIConfig {
    /// A list of environment variables available for this module.
    pub envs: Option<Vec<Vec<u8>>>,

    /// A list of files available for this module.
    /// A loaded module could have access only to files from this list.
    pub preopened_files: Option<Vec<String>>,

    /// Mapping from a usually short to full file name.
    pub mapped_dirs: Option<Vec<(String, String)>>,
}

/// Prepare config after parsing it from TOML.
fn from_raw_modules_config(config: RawModulesConfig) -> Result<ModulesConfig> {
    let service_base_dir = config.service_base_dir;
    let modules_config = config
        .module
        .into_iter()
        .map(from_raw_module_config)
        .collect::<Result<Vec<_>>>()?;

    let default_modules_config = config
        .default
        .map(from_raw_default_module_config)
        .transpose()?;

    Ok(ModulesConfig {
        service_base_dir,
        modules_dir: config.modules_dir,
        modules_config,
        default_modules_config,
    })
}

/// Parse config from TOML.
pub(crate) fn load_config(config_file_path: std::path::PathBuf) -> Result<RawModulesConfig> {
    let file_content = std::fs::read(config_file_path)?;
    Ok(toml::from_slice(&file_content)?)
}

fn from_raw_module_config(config: RawModuleConfig) -> Result<(String, ModuleConfig)> {
    let imports = config.imports.map(parse_imports).transpose()?;
    let wasi = config.wasi.map(from_raw_wasi_config);
    Ok((
        config.name,
        ModuleConfig {
            mem_pages_count: config.mem_pages_count,
            logger_enabled: config.logger_enabled.unwrap_or_default(),
            imports,
            wasi,
        },
    ))
}

fn from_raw_default_module_config(config: RawDefaultModuleConfig) -> Result<ModuleConfig> {
    let imports = config.imports.map(parse_imports).transpose()?;
    let wasi = config.wasi.map(from_raw_wasi_config);
    Ok(ModuleConfig {
        mem_pages_count: config.mem_pages_count,
        logger_enabled: config.logger_enabled.unwrap_or_default(),
        imports,
        wasi,
    })
}

fn from_raw_wasi_config(wasi: RawWASIConfig) -> WASIConfig {
    let envs = wasi
        .envs
        .map(|env| env.into_iter().map(|e| e.into_bytes()).collect::<Vec<_>>());

    let mapped_dirs = wasi.mapped_dirs.map(|mapped_dir| {
        mapped_dir
            .into_iter()
            .map(|(from, to)| (from, to.try_into::<String>().unwrap()))
            .collect::<Vec<_>>()
    });

    WASIConfig {
        envs,
        preopened_files: wasi.preopened_files,
        mapped_dirs,
    }
}

fn parse_imports(imports: toml::value::Table) -> Result<Vec<(String, String)>> {
    imports
        .into_iter()
        .map(|(import_func_name, host_cmd)| {
            let host_cmd = host_cmd.try_into::<String>()?;
            Ok((import_func_name, host_cmd))
        })
        .collect::<Result<Vec<_>>>()
}
