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

use marine_core::HostImportDescriptor;

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Default, Debug)]
pub struct ConfigContext {
    pub base_path: Option<PathBuf>,
}

pub struct WithContext<'c, T> {
    pub context: &'c ConfigContext,
    pub data: T,
}

impl ConfigContext {
    pub fn wrapped<'c, T>(&'c self, data: T) -> WithContext<'c, T> {
        WithContext {
            context: &self,
            data,
        }
    }
}

/// Info to load a module from filesystem into runtime.
#[derive(Default)]
pub struct ModuleDescriptor {
    pub load_from: Option<PathBuf>,
    pub file_name: String,
    pub import_name: String,
    pub config: MarineModuleConfig,
}

impl ModuleDescriptor {
    pub fn get_path(&self, modules_dir: &Option<PathBuf>) -> Result<PathBuf, MarineError> {
        match &self.load_from {
            None => match modules_dir {
                Some(dir) => Ok(dir.join(Path::new(&self.file_name))),
                None => Err(MarineError::InvalidConfig(format!(
                    r#""modules_dir" field is not defined, but it is required to load module "{}""#,
                    self.import_name
                ))),
            },
            Some(path) => {
                if path.is_file() {
                    Ok(path.clone())
                } else {
                    Ok(path.join(Path::new(&self.file_name)))
                }
            }
        }
    }
}

/// Describes the behaviour of the Marine component.
#[derive(Default)]
pub struct MarineConfig {
    /// Path to a dir where compiled Wasm modules are located.
    pub modules_dir: Option<PathBuf>,

    /// Settings for a module with particular name (not HashMap because the order is matter).
    pub modules_config: Vec<ModuleDescriptor>,

    /// Settings for a module that name's not been found in modules_config.
    pub default_modules_config: Option<MarineModuleConfig>,
}

/// Various settings that could be used to guide Marine how to load a module in a proper way.
#[derive(Default)]
pub struct MarineModuleConfig {
    /// Maximum memory size accessible by a module in Wasm pages (64 Kb).
    pub mem_pages_count: Option<u32>,

    /// Maximum memory size for heap of Wasm module in bytes, if it set, mem_pages_count ignored.
    pub max_heap_size: Option<u64>,

    /// Defines whether Marine should provide a special host log_utf8_string function for this module.
    pub logger_enabled: bool,

    /// Export from host functions that will be accessible on the Wasm side by provided name.
    pub host_imports: HashMap<String, HostImportDescriptor>,

    /// A WASI config.
    pub wasi: Option<MarineWASIConfig>,

    /// Mask used to filter logs, for details see `log_utf8_string`
    pub logging_mask: i32,
}

impl MarineModuleConfig {
    pub fn extend_wasi_envs(&mut self, new_envs: HashMap<Vec<u8>, Vec<u8>>) {
        match &mut self.wasi {
            Some(MarineWASIConfig { envs, .. }) => envs.extend(new_envs),
            w @ None => {
                *w = Some(MarineWASIConfig {
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
            Some(MarineWASIConfig {
                preopened_files,
                mapped_dirs,
                ..
            }) => {
                preopened_files.extend(new_preopened_files);
                mapped_dirs.extend(new_mapped_dirs);
            }
            w @ None => {
                *w = Some(MarineWASIConfig {
                    envs: HashMap::new(),
                    preopened_files: new_preopened_files,
                    mapped_dirs: new_mapped_dirs,
                })
            }
        };
    }
}

#[derive(Debug, Clone, Default)]
pub struct MarineWASIConfig {
    /// A list of environment variables available for this module.
    pub envs: HashMap<Vec<u8>, Vec<u8>>,

    /// A list of files available for this module.
    /// A loaded module could have access only to files from this list.
    pub preopened_files: HashSet<PathBuf>,

    /// Mapping from a usually short to full file name.
    pub mapped_dirs: HashMap<String, PathBuf>,
}

use super::TomlMarineConfig;
use super::TomlMarineModuleConfig;
use super::TomlWASIConfig;
use super::TomlMarineNamedModuleConfig;
use crate::MarineError;
use crate::MarineResult;
use crate::config::as_relative_to_base;

use std::convert::{TryFrom, TryInto};

impl TryFrom<TomlMarineConfig> for MarineConfig {
    type Error = MarineError;

    fn try_from(toml_config: TomlMarineConfig) -> Result<Self, Self::Error> {
        let base_path = PathBuf::from(toml_config.base_path);
        let context = ConfigContext {
            base_path: Some(base_path),
        };
        // unwrap is safe because it was set to Some in the line above;

        let modules_dir = toml_config
            .modules_dir
            .map(|dir| as_relative_to_base(context.base_path.as_ref(), &dir))
            .transpose()?;

        let default_modules_config = toml_config
            .default
            .map(|m| context.wrapped(m).try_into())
            .transpose()?;

        let modules_config = toml_config
            .module
            .into_iter()
            .map(|toml_module| ModuleDescriptor::try_from(context.wrapped(toml_module)))
            .collect::<MarineResult<Vec<_>>>()?;

        Ok(MarineConfig {
            modules_dir,
            modules_config,
            default_modules_config,
        })
    }
}

impl<'c> TryFrom<WithContext<'c, TomlMarineNamedModuleConfig>> for ModuleDescriptor {
    type Error = MarineError;

    fn try_from(config: WithContext<'c, TomlMarineNamedModuleConfig>) -> Result<Self, Self::Error> {
        let WithContext {
            context,
            data: config,
        } = config;

        let file_name = config.file_name.unwrap_or(format!("{}.wasm", config.name));
        let load_from = config
            .load_from
            .map(|path| as_relative_to_base(context.base_path.as_ref(), &path))
            .transpose()?;

        Ok(ModuleDescriptor {
            load_from,
            file_name,
            import_name: config.name,
            config: context.wrapped(config.config).try_into()?,
        })
    }
}

impl<'c> TryFrom<WithContext<'c, TomlMarineModuleConfig>> for MarineModuleConfig {
    type Error = MarineError;

    fn try_from(toml_config: WithContext<'c, TomlMarineModuleConfig>) -> Result<Self, Self::Error> {
        let WithContext {
            context,
            data: toml_config,
        } = toml_config;

        let mounted_binaries = toml_config.mounted_binaries.unwrap_or_default();
        let mounted_binaries = mounted_binaries
            .into_iter()
            .map(|(import_func_name, host_cmd)| {
                let host_cmd = host_cmd.try_into::<PathBuf>()?;
                Ok((import_func_name, host_cmd))
            })
            .collect::<Result<Vec<_>, Self::Error>>()?;

        let max_heap_size = toml_config.max_heap_size.map(|v| v.as_u64());
        let mut host_cli_imports = HashMap::new();
        for (import_name, host_cmd) in mounted_binaries {
            let host_cmd = as_relative_to_base(context.base_path.as_ref(), &host_cmd)?;
            host_cli_imports.insert(
                import_name,
                crate::host_imports::create_mounted_binary_import(host_cmd),
            );
        }

        let wasi = toml_config
            .wasi
            .map(|w| context.wrapped(w).try_into())
            .transpose()?;

        Ok(MarineModuleConfig {
            mem_pages_count: toml_config.mem_pages_count,
            max_heap_size,
            logger_enabled: toml_config.logger_enabled.unwrap_or(true),
            host_imports: host_cli_imports,
            wasi,
            logging_mask: toml_config.logging_mask.unwrap_or(i32::max_value()),
        })
    }
}

impl<'c> TryFrom<WithContext<'c, TomlWASIConfig>> for MarineWASIConfig {
    type Error = MarineError;

    fn try_from(toml_config: WithContext<'c, TomlWASIConfig>) -> Result<Self, Self::Error> {
        let WithContext {
            context,
            data: toml_config,
        } = toml_config;

        let to_vec = |elem: (String, toml::Value)| -> Result<(Vec<u8>, Vec<u8>), Self::Error> {
            let to = elem
                .1
                .try_into::<String>()
                .map_err(MarineError::ParseConfigError)?;
            Ok((elem.0.into_bytes(), to.into_bytes()))
        };

        let to_path = |elem: (String, toml::Value)| -> Result<(String, PathBuf), Self::Error> {
            let to = elem
                .1
                .try_into::<PathBuf>()
                .map_err(MarineError::ParseConfigError)?;
            let to = as_relative_to_base(context.base_path.as_ref(), &to)?;
            Ok((elem.0, to))
        };

        let envs = toml_config.envs.unwrap_or_default();
        let envs = envs
            .into_iter()
            .map(to_vec)
            .collect::<Result<HashMap<_, _>, _>>()?;

        let preopened_files = toml_config.preopened_files.unwrap_or_default();
        let preopened_files = preopened_files
            .into_iter()
            .map(|path| as_relative_to_base(context.base_path.as_ref(), &path))
            .collect::<Result<HashSet<_>, _>>()?;

        let mapped_dirs = toml_config.mapped_dirs.unwrap_or_default();
        let mapped_dirs = mapped_dirs
            .into_iter()
            .map(to_path)
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(MarineWASIConfig {
            envs,
            preopened_files,
            mapped_dirs,
        })
    }
}
