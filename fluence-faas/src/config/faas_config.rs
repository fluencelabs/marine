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
use std::path::{PathBuf};

/// Info to load a module from filesystem into runtime.
#[derive(Default)]
pub struct ModuleDescriptor {
    pub file_name: String,
    pub import_name: String,
    pub config: FaaSModuleConfig,
}

/// Describes the behaviour of FluenceFaaS.
#[derive(Default)]
pub struct FaaSConfig {
    /// Path to a dir where compiled Wasm modules are located.
    pub modules_dir: Option<PathBuf>,

    /// Settings for a module with particular name (not HashMap because the order is matter).
    pub modules_config: Vec<ModuleDescriptor>,

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

    /// Mask used to filter logs, for details see `log_utf8_string`
    pub logging_mask: i32,
}

impl FaaSModuleConfig {
    pub fn extend_wasi_envs(&mut self, new_envs: HashMap<Vec<u8>, Vec<u8>>) {
        match &mut self.wasi {
            Some(FaaSWASIConfig { envs, .. }) => envs.extend(new_envs),
            w @ None => {
                *w = Some(FaaSWASIConfig {
                    envs: new_envs,
                    preopened_files: HashSet::new(),
                    mapped_dirs: HashMap::new(),
                })
            }
        };
    }

    pub fn extend_wasi_files(
        &mut self,
        new_preopened_files: HashSet<PathBuf>,
        new_mapped_dirs: HashMap<String, PathBuf>,
    ) {
        match &mut self.wasi {
            Some(FaaSWASIConfig {
                preopened_files,
                mapped_dirs,
                ..
            }) => {
                preopened_files.extend(new_preopened_files);
                mapped_dirs.extend(new_mapped_dirs);
            }
            w @ None => {
                *w = Some(FaaSWASIConfig {
                    envs: HashMap::new(),
                    preopened_files: new_preopened_files,
                    mapped_dirs: new_mapped_dirs,
                })
            }
        };
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

use super::TomlFaaSConfig;
use super::TomlFaaSModuleConfig;
use super::TomlWASIConfig;
use super::TomlFaaSNamedModuleConfig;
use crate::FaaSError;

use std::convert::{TryFrom, TryInto};

impl TryFrom<TomlFaaSConfig> for FaaSConfig {
    type Error = FaaSError;

    fn try_from(toml_config: TomlFaaSConfig) -> Result<Self, Self::Error> {
        let modules_dir = toml_config.modules_dir.map(PathBuf::from);

        let default_modules_config = toml_config
            .default
            .map(|m| m.try_into())
            .transpose()?;

        let modules_config = toml_config
            .module
            .into_iter()
            .map(ModuleDescriptor::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(FaaSConfig {
            modules_dir,
            modules_config,
            default_modules_config,
        })
    }
}

impl TryFrom<TomlFaaSNamedModuleConfig> for ModuleDescriptor {
    type Error = FaaSError;

    fn try_from(config: TomlFaaSNamedModuleConfig) -> Result<Self, Self::Error> {
        Ok(ModuleDescriptor {
            file_name: config.file_name.unwrap_or(format!("{}.wasm", config.name)),
            import_name: config.name,
            config: config.config.try_into()?,
        })
    }
}

impl TryFrom<TomlFaaSModuleConfig> for FaaSModuleConfig {
    type Error = FaaSError;

    fn try_from(toml_config: TomlFaaSModuleConfig) -> Result<Self, Self::Error> {
        let mounted_binaries = toml_config.mounted_binaries.unwrap_or_default();
        let mounted_binaries = mounted_binaries
            .into_iter()
            .map(|(import_func_name, host_cmd)| {
                let host_cmd = host_cmd.try_into::<String>()?;
                Ok((import_func_name, host_cmd))
            })
            .collect::<Result<Vec<_>, Self::Error>>()?;

        let mut host_cli_imports = HashMap::new();
        for (import_name, host_cmd) in mounted_binaries {
            host_cli_imports.insert(
                import_name,
                crate::host_imports::create_host_import(host_cmd),
            );
        }

        let wasi = toml_config.wasi.map(|w| w.try_into()).transpose()?;

        Ok(FaaSModuleConfig {
            mem_pages_count: toml_config.mem_pages_count,
            logger_enabled: toml_config.logger_enabled.unwrap_or(true),
            host_imports: host_cli_imports,
            wasi,
            logging_mask: toml_config.logging_mask.unwrap_or(i32::max_value()),
        })
    }
}

impl TryFrom<TomlWASIConfig> for FaaSWASIConfig {
    type Error = FaaSError;

    fn try_from(toml_config: TomlWASIConfig) -> Result<Self, Self::Error> {
        let to_vec = |elem: (String, toml::Value)| -> Result<(Vec<u8>, Vec<u8>), Self::Error> {
            let to = elem
                .1
                .try_into::<String>()
                .map_err(FaaSError::ParseConfigError)?;
            Ok((elem.0.into_bytes(), to.into_bytes()))
        };

        let to_path = |elem: (String, toml::Value)| -> Result<(String, PathBuf), Self::Error> {
            let to = elem
                .1
                .try_into::<String>()
                .map_err(FaaSError::ParseConfigError)?;
            Ok((elem.0, PathBuf::from(to)))
        };

        let envs = toml_config.envs.unwrap_or_default();
        let envs = envs
            .into_iter()
            .map(to_vec)
            .collect::<Result<HashMap<_, _>, _>>()?;

        let preopened_files = toml_config.preopened_files.unwrap_or_default();
        let preopened_files = preopened_files
            .into_iter()
            .map(PathBuf::from)
            .collect::<HashSet<_>>();

        let mapped_dirs = toml_config.mapped_dirs.unwrap_or_default();
        let mapped_dirs = mapped_dirs
            .into_iter()
            .map(to_path)
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(FaaSWASIConfig {
            envs,
            preopened_files,
            mapped_dirs,
        })
    }
}
