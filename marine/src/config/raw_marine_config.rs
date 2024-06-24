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

use crate::MarineError;
use crate::MarineResult;

use bytesize::ByteSize;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use serde_with::serde_as;
use serde_with::skip_serializing_none;

use std::path::Path;
use std::path::PathBuf;

/*
An example of the config:

modules_dir = "wasm/artifacts/wasm_modules"

[[module]]
    name = "ipfs_node.wasm"
    mem_pages_count = 100
    logger_enabled = true

    [module.mounted_binaries]
    mysql = "/usr/bin/mysql"
    ipfs = "/usr/local/bin/ipfs"

    [module.wasi]
    envs = { "IPFS_ADDR" = "/dns4/relay02.fluence.dev/tcp/15001" }
    mapped_dirs = {"tmp" = "/Users/user/tmp"}

[default]
    mem_pages_count = 100
    logger_enabled = true

    [default.mounted_binaries]
    mysql = "/usr/bin/mysql"
    ipfs = "/usr/local/bin/ipfs"

    [default.wasi]
    envs = []
    mapped_dirs = {"tmp" = "/Users/user/tmp"}
 */

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlMarineConfig {
    pub modules_dir: Option<PathBuf>,
    pub total_memory_limit: MemoryLimit,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub module: Vec<TomlMarineNamedModuleConfig>,
    pub default: Option<TomlMarineModuleConfig>,
    #[serde(skip)]
    pub base_path: PathBuf,
}

impl TomlMarineConfig {
    /// Load config from filesystem.
    pub fn load<P: AsRef<Path>>(path: P) -> MarineResult<Self> {
        let path = PathBuf::from(path.as_ref()).canonicalize().map_err(|e| {
            MarineError::IOError(format!(
                "failed to canonicalize path {}: {}",
                path.as_ref().display(),
                e
            ))
        })?;

        let file_content = std::fs::read(&path).map_err(|e| {
            MarineError::IOError(format!("failed to load {}: {}", path.display(), e))
        })?;

        let mut config: TomlMarineConfig = toml::from_slice(&file_content)?;

        let default_base_path = Path::new("/");
        config.base_path = path
            .canonicalize()
            .map_err(|e| {
                MarineError::IOError(format!(
                    "Failed to canonicalize config path {}: {}",
                    path.display(),
                    e
                ))
            })?
            .parent()
            .unwrap_or(default_base_path)
            .to_path_buf();

        Ok(config)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlMarineNamedModuleConfig {
    pub name: String,
    #[serde(default)]
    pub load_from: Option<PathBuf>,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(flatten)]
    pub config: TomlMarineModuleConfig,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlMarineModuleConfig {
    pub logger_enabled: Option<bool>,
    pub logging_mask: Option<i32>,
    pub wasi: Option<TomlWASIConfig>,
    pub mounted_binaries: Option<toml::value::Table>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlWASIConfig {
    pub envs: Option<toml::value::Table>,
    pub mapped_dirs: Option<toml::value::Table>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde_as]
pub enum MemoryLimit {
    #[default]
    #[serde(alias = "infinity")]
    Infinity,
    #[serde(untagged)]
    Value(#[serde_as(as = "Option<DisplayFromStr>")] ByteSize),
}

#[cfg(test)]
mod tests {
    use super::TomlMarineNamedModuleConfig;
    use super::TomlMarineModuleConfig;
    use super::TomlWASIConfig;

    #[test]
    fn serialize_marine_named_module_config() {
        let mut mounted_binaries = toml::value::Table::new();
        mounted_binaries.insert(
            "curl".to_string(),
            toml::Value::String("/usr/local/bin/curl".to_string()),
        );

        let config = TomlMarineNamedModuleConfig {
            name: "name".to_string(),
            file_name: Some("file_name".to_string()),
            load_from: <_>::default(),
            config: TomlMarineModuleConfig {
                logger_enabled: Some(false),
                logging_mask: Some(1),
                wasi: Some(TomlWASIConfig {
                    envs: None,
                    mapped_dirs: None,
                }),
                mounted_binaries: Some(mounted_binaries),
            },
        };

        assert!(toml::to_string(&config).is_ok())
    }
}
