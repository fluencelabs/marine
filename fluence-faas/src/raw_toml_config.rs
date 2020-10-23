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
use crate::config::*;

use serde_derive::Serialize;
use serde_derive::Deserialize;

use std::convert::TryInto;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

/*
An example of the config:

modules_dir = "wasm/artifacts/wasm_modules"

[[module]]
    name = "ipfs_node.wasm"
    mem_pages_count = 100
    logger_enabled = true

    [module.imports]
    mysql = "/usr/bin/mysql"
    ipfs = "/usr/local/bin/ipfs"

    [module.wasi]
    envs = { "IPFS_ADDR" = "/dns4/relay02.fluence.dev/tcp/15001" }
    preopened_files = ["/Users/user/tmp"]
    mapped_dirs = {"tmp" = "/Users/user/tmp"}

[default]
    mem_pages_count = 100
    logger_enabled = true

    [default.imports]
    mysql = "/usr/bin/mysql"
    ipfs = "/usr/local/bin/ipfs"

    [default.wasi]
    envs = []
    preopened_files = ["/Users/user/tmp"]
    mapped_dirs = {"tmp" = "/Users/user/tmp"}
 */

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlFaaSConfig {
    pub modules_dir: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub module: Vec<TomlFaaSNamedModuleConfig>,
    pub default: Option<TomlFaaSModuleConfig>,
}

impl TomlFaaSConfig {
    /// Load config from filesystem.
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        let file_content = std::fs::read(&path)?;
        toml::from_slice(&file_content).map_err(|e| {
            FaaSError::ConfigParseError(format!("Error parsing config {:?}: {:?}", path, e))
        })
    }
}

impl TryInto<FaaSConfig> for TomlFaaSConfig {
    type Error = FaaSError;

    fn try_into(self) -> Result<FaaSConfig> {
        from_toml_faas_config(self)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlFaaSNamedModuleConfig {
    pub name: String,
    #[serde(flatten)]
    pub config: TomlFaaSModuleConfig,
}

impl TryInto<(String, FaaSModuleConfig)> for TomlFaaSNamedModuleConfig {
    type Error = FaaSError;

    fn try_into(self) -> Result<(String, FaaSModuleConfig)> {
        from_toml_named_module_config(self)
    }
}

impl TryInto<FaaSModuleConfig> for TomlFaaSNamedModuleConfig {
    type Error = FaaSError;

    fn try_into(self) -> Result<FaaSModuleConfig> {
        from_toml_named_module_config(self).map(|(_, module_config)| module_config)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlFaaSModuleConfig {
    pub mem_pages_count: Option<u32>,
    pub logger_enabled: Option<bool>,
    pub wasi: Option<TomlWASIConfig>,
    pub mounted_binaries: Option<toml::value::Table>,
    pub logging_mask: Option<i64>,
}

impl TomlFaaSNamedModuleConfig {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            config: <_>::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlWASIConfig {
    pub preopened_files: Option<Vec<String>>,
    pub envs: Option<toml::value::Table>,
    pub mapped_dirs: Option<toml::value::Table>,
}

/// Prepare config after parsing it from TOML.
pub fn from_toml_faas_config(config: TomlFaaSConfig) -> Result<FaaSConfig> {
    let modules_config = config
        .module
        .into_iter()
        .map(from_toml_named_module_config)
        .collect::<Result<Vec<_>>>()?;

    let default_modules_config = config.default.map(from_toml_module_config).transpose()?;

    Ok(FaaSConfig {
        modules_dir: config.modules_dir.map(PathBuf::from),
        modules_config,
        default_modules_config,
    })
}

pub fn from_toml_named_module_config(
    config: TomlFaaSNamedModuleConfig,
) -> Result<(String, FaaSModuleConfig)> {
    let module_config = from_toml_module_config(config.config)?;
    Ok((config.name, module_config))
}

pub fn from_toml_module_config(config: TomlFaaSModuleConfig) -> Result<FaaSModuleConfig> {
    let mounted_binaries = config.mounted_binaries.unwrap_or_default();
    let mounted_binaries = mounted_binaries
        .into_iter()
        .map(|(import_func_name, host_cmd)| {
            let host_cmd = host_cmd.try_into::<String>()?;
            Ok((import_func_name, host_cmd))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut host_cli_imports = HashMap::new();
    for (import_name, host_cmd) in mounted_binaries {
        host_cli_imports.insert(import_name, crate::misc::create_host_import(host_cmd));
    }

    let wasi = config.wasi.map(from_toml_wasi_config).transpose()?;
    Ok(FaaSModuleConfig {
        mem_pages_count: config.mem_pages_count,
        logger_enabled: config.logger_enabled.unwrap_or(true),
        host_imports: host_cli_imports,
        wasi,
        logging_mask: config.logging_mask.unwrap_or(i64::max_value()),
    })
}

pub fn from_toml_wasi_config(wasi: TomlWASIConfig) -> Result<FaaSWASIConfig> {
    let to_vec = |elem: (String, toml::Value)| -> Result<(Vec<u8>, Vec<u8>)> {
        let to = elem
            .1
            .try_into::<String>()
            .map_err(FaaSError::ParseConfigError)?;
        Ok((elem.0.into_bytes(), to.into_bytes()))
    };

    let to_path = |elem: (String, toml::Value)| -> Result<(String, PathBuf)> {
        let to = elem
            .1
            .try_into::<String>()
            .map_err(FaaSError::ParseConfigError)?;
        Ok((elem.0, PathBuf::from(to)))
    };

    let envs = wasi.envs.unwrap_or_default();
    let envs = envs
        .into_iter()
        .map(to_vec)
        .collect::<Result<HashMap<_, _>>>()?;

    let preopened_files = wasi.preopened_files.unwrap_or_default();
    let preopened_files = preopened_files
        .into_iter()
        .map(PathBuf::from)
        .collect::<HashSet<_>>();

    let mapped_dirs = wasi.mapped_dirs.unwrap_or_default();
    let mapped_dirs = mapped_dirs
        .into_iter()
        .map(to_path)
        .collect::<Result<HashMap<_, _>>>()?;

    Ok(FaaSWASIConfig {
        envs,
        preopened_files,
        mapped_dirs,
    })
}

#[cfg(test)]
mod tests {
    use crate::{TomlFaaSNamedModuleConfig, TomlFaaSModuleConfig, TomlWASIConfig};

    #[test]
    fn serialize_named() {
        let config = TomlFaaSNamedModuleConfig {
            name: "name".to_string(),
            config: TomlFaaSModuleConfig {
                mem_pages_count: Some(100),
                logger_enabled: Some(false),
                wasi: Some(TomlWASIConfig {
                    preopened_files: Some(vec!["a".to_string()]),
                    envs: None,
                    mapped_dirs: None,
                }),
                mounted_binaries: None,
                logging_mask: None,
            },
        };

        assert!(toml::to_string(&config).is_ok())
    }
}
