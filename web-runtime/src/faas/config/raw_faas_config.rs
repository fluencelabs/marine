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

//use crate::faas::Result;

use serde_derive::Serialize;
use serde_derive::Deserialize;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::DisplayFromStr;

//use std::path::Path;

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
    preopened_files = ["/Users/user/tmp"]
    mapped_dirs = {"tmp" = "/Users/user/tmp"}

[default]
    mem_pages_count = 100
    logger_enabled = true

    [default.mounted_binaries]
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
/*
impl TomlFaaSConfig {
    /// Load config from filesystem.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file_content = std::fs::read(path)?;
        Ok(toml::from_slice(&file_content)?)
    }
}
*/
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlFaaSNamedModuleConfig {
    pub name: String,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(flatten)]
    pub config: TomlFaaSModuleConfig,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlFaaSModuleConfig {
    pub mem_pages_count: Option<u32>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub max_heap_size: Option<bytesize::ByteSize>,
    pub logger_enabled: Option<bool>,
    //pub wasi: Option<TomlWASIConfig>,
    //pub mounted_binaries: Option<toml::value::Table>,
    pub logging_mask: Option<i32>,
}
/*
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlWASIConfig {
    pub preopened_files: Option<Vec<String>>,
    pub envs: Option<toml::value::Table>,
    pub mapped_dirs: Option<toml::value::Table>,
}
*/
#[cfg(test)]
mod tests {
    use bytesize::ByteSize;
    use super::{TomlFaaSNamedModuleConfig, TomlFaaSModuleConfig, TomlWASIConfig};

    #[test]
    fn serialize_named() {
        let config = TomlFaaSNamedModuleConfig {
            name: "name".to_string(),
            file_name: Some("file_name".to_string()),
            config: TomlFaaSModuleConfig {
                mem_pages_count: Some(100),
                max_heap_size: Some(ByteSize::gib(4)),
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
