/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use marine_wasm_backend_traits::WasmBackend;
use marine_core::generic::HostImportDescriptor;
use marine_core::HostAPIVersion;

use std::collections::HashMap;
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
    pub fn wrapped<T>(&self, data: T) -> WithContext<'_, T> {
        WithContext {
            context: self,
            data,
        }
    }
}

/// Info to load a module from filesystem into runtime.
#[derive(Default)]
pub struct ModuleDescriptor<WB: WasmBackend> {
    pub load_from: Option<PathBuf>,
    pub file_name: String,
    pub import_name: String,
    pub config: MarineModuleConfig<WB>,
}

impl<WB: WasmBackend> ModuleDescriptor<WB> {
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
pub struct MarineConfig<WB: WasmBackend> {
    /// Path to a dir where compiled Wasm modules are located.
    pub modules_dir: Option<PathBuf>,

    /// Total memory available for the service (in bytes)
    pub total_memory_limit: Option<u64>,

    /// Settings for a module with particular name (not HashMap because the order is matter).
    pub modules_config: Vec<ModuleDescriptor<WB>>,

    /// Settings for a module that name's not been found in modules_config.
    pub default_modules_config: Option<MarineModuleConfig<WB>>,
}

// Manual implementation because #[derive(Default)] does not allow direct usage of non-Default wasm backend.
impl<WB: WasmBackend> Default for MarineConfig<WB> {
    fn default() -> Self {
        Self {
            modules_dir: <_>::default(),
            total_memory_limit: <_>::default(),
            modules_config: <_>::default(),
            default_modules_config: <_>::default(),
        }
    }
}

/// Various settings that could be used to guide Marine how to load a module in a proper way.
#[derive(Default)]
pub struct MarineModuleConfig<WB: WasmBackend> {
    /// Defines whether Marine should provide a special host log_utf8_string function for this module.
    pub logger_enabled: bool,

    /// Export from host functions that will be accessible on the Wasm side by provided name.
    /// The imports are provided separately for each marine host api version
    pub host_imports: HashMap<HostAPIVersion, HashMap<String, HostImportDescriptor<WB>>>,

    /// A WASI config.
    pub wasi: Option<MarineWASIConfig>,

    /// Mask used to filter logs, for details see `log_utf8_string`
    pub logging_mask: i32,
}

impl<WB: WasmBackend> MarineModuleConfig<WB> {
    pub fn extend_wasi_envs(&mut self, new_envs: HashMap<String, String>) {
        match &mut self.wasi {
            Some(MarineWASIConfig { envs, .. }) => envs.extend(new_envs),
            w @ None => {
                *w = Some(MarineWASIConfig {
                    envs: new_envs,
                    mapped_dirs: HashMap::new(),
                })
            }
        };
    }

    pub fn root_wasi_files_at(&mut self, root: &Path) {
        match &mut self.wasi {
            Some(MarineWASIConfig { mapped_dirs, .. }) => {
                mapped_dirs.values_mut().for_each(|path| {
                    *path = root.join(&path);
                });
            }
            None => {}
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MarineWASIConfig {
    /// A list of environment variables available for this module.
    pub envs: HashMap<String, String>,

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
use crate::config::raw_marine_config::MemoryLimit;

use std::convert::TryFrom;
use std::convert::TryInto;

impl<WB: WasmBackend> TryFrom<TomlMarineConfig> for MarineConfig<WB> {
    type Error = MarineError;

    fn try_from(toml_config: TomlMarineConfig) -> Result<Self, Self::Error> {
        let base_path = toml_config.base_path;
        let context = ConfigContext {
            base_path: Some(base_path),
        };

        let modules_dir = toml_config
            .modules_dir
            .map(|dir| as_relative_to_base(context.base_path.as_deref(), &dir))
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

        let total_memory_limit = match toml_config.total_memory_limit {
            MemoryLimit::Infinity => None,
            MemoryLimit::Value(bytesize) => Some(bytesize.as_u64()),
        };

        Ok(MarineConfig {
            modules_dir,
            total_memory_limit,
            modules_config,
            default_modules_config,
        })
    }
}

impl<'c, WB: WasmBackend> TryFrom<WithContext<'c, TomlMarineNamedModuleConfig>>
    for ModuleDescriptor<WB>
{
    type Error = MarineError;

    fn try_from(config: WithContext<'c, TomlMarineNamedModuleConfig>) -> Result<Self, Self::Error> {
        let WithContext {
            context,
            data: config,
        } = config;

        let file_name = config.file_name.unwrap_or(format!("{}.wasm", config.name));
        let load_from = config
            .load_from
            .map(|path| as_relative_to_base(context.base_path.as_deref(), &path))
            .transpose()?;

        Ok(ModuleDescriptor {
            load_from,
            file_name,
            import_name: config.name,
            config: context.wrapped(config.config).try_into()?,
        })
    }
}

impl<'c, WB: WasmBackend> TryFrom<WithContext<'c, TomlMarineModuleConfig>>
    for MarineModuleConfig<WB>
{
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

        let mut host_imports = HashMap::from([
            (HostAPIVersion::V0, HashMap::new()),
            (HostAPIVersion::V1, HashMap::new()),
            (HostAPIVersion::V2, HashMap::new()),
            (HostAPIVersion::V3, HashMap::new()),
        ]);
        for (import_name, host_cmd) in mounted_binaries {
            let host_cmd = as_relative_to_base(context.base_path.as_deref(), &host_cmd)?;
            for (_, host_cli_imports) in &mut host_imports {
                host_cli_imports.insert(
                    import_name.clone(),
                    crate::host_imports::create_mounted_binary_import(host_cmd.clone()),
                );
            }
        }

        let wasi = toml_config.wasi.map(|w| w.try_into()).transpose()?;

        Ok(MarineModuleConfig {
            logger_enabled: toml_config.logger_enabled.unwrap_or(true),
            host_imports,
            wasi,
            logging_mask: toml_config.logging_mask.unwrap_or(i32::max_value()),
        })
    }
}

impl TryFrom<TomlWASIConfig> for MarineWASIConfig {
    type Error = MarineError;

    fn try_from(toml_config: TomlWASIConfig) -> Result<Self, Self::Error> {
        let to_string = |elem: (String, toml::Value)| -> Result<(String, String), Self::Error> {
            let to = elem
                .1
                .try_into::<String>()
                .map_err(MarineError::ParseConfigError)?;
            Ok((elem.0, to))
        };

        let to_path = |elem: (String, toml::Value)| -> Result<(String, PathBuf), Self::Error> {
            let to = elem
                .1
                .try_into::<PathBuf>()
                .map_err(MarineError::ParseConfigError)?;

            Ok((elem.0, to))
        };

        let envs = toml_config.envs.unwrap_or_default();
        let envs = envs
            .into_iter()
            .map(to_string)
            .collect::<Result<HashMap<_, _>, _>>()?;

        let mapped_dirs = toml_config.mapped_dirs.unwrap_or_default();
        let mapped_dirs = mapped_dirs
            .into_iter()
            .map(to_path)
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(MarineWASIConfig { envs, mapped_dirs })
    }
}
