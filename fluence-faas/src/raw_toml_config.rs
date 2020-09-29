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
    pub module: Vec<TomlFaaSModuleConfig>,
    pub default: Option<TomlDefaultFaaSModuleConfig>,
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
        from_raw_modules_config(self)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlFaaSModuleConfig {
    pub name: String,
    pub mem_pages_count: Option<u32>,
    pub logger_enabled: Option<bool>,
    pub imports: Option<toml::value::Table>,
    pub wasi: Option<TomlWASIConfig>,
}

impl TryInto<FaaSModuleConfig> for TomlFaaSModuleConfig {
    type Error = FaaSError;

    fn try_into(self) -> Result<FaaSModuleConfig> {
        from_toml_module_config(self).map(|(_, module_config)| module_config)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlDefaultFaaSModuleConfig {
    pub mem_pages_count: Option<u32>,
    pub logger_enabled: Option<bool>,
    pub imports: Option<toml::value::Table>,
    pub wasi: Option<TomlWASIConfig>,
}

impl TomlFaaSModuleConfig {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            mem_pages_count: None,
            logger_enabled: None,
            imports: None,
            wasi: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlWASIConfig {
    pub envs: Option<toml::value::Table>,
    pub preopened_files: Option<Vec<String>>,
    pub mapped_dirs: Option<toml::value::Table>,
}

/// Prepare config after parsing it from TOML.
fn from_raw_modules_config(config: TomlFaaSConfig) -> Result<FaaSConfig> {
    let modules_config = config
        .module
        .into_iter()
        .map(from_toml_module_config)
        .collect::<Result<HashMap<_, _>>>()?;

    let default_modules_config = config
        .default
        .map(from_raw_default_module_config)
        .transpose()?;

    Ok(FaaSConfig {
        modules_dir: config.modules_dir,
        modules_config,
        default_modules_config,
    })
}

fn from_toml_module_config(config: TomlFaaSModuleConfig) -> Result<(String, FaaSModuleConfig)> {
    let imports = config.imports.map(parse_imports).transpose()?;
    let wasi = config.wasi.map(from_raw_wasi_config);
    Ok((
        config.name,
        FaaSModuleConfig {
            mem_pages_count: config.mem_pages_count,
            logger_enabled: config.logger_enabled.unwrap_or(true),
            host_cli_imports: imports,
            wasi,
        },
    ))
}

fn from_raw_default_module_config(config: TomlDefaultFaaSModuleConfig) -> Result<FaaSModuleConfig> {
    let imports = config.imports.map(parse_imports).transpose()?;
    let wasi = config.wasi.map(from_raw_wasi_config);
    Ok(FaaSModuleConfig {
        mem_pages_count: config.mem_pages_count,
        logger_enabled: config.logger_enabled.unwrap_or(true),
        host_cli_imports: imports,
        wasi,
    })
}

fn from_raw_wasi_config(wasi: TomlWASIConfig) -> WASIConfig {
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
