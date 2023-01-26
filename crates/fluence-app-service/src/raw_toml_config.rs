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

use crate::Result;
use crate::AppServiceError;
use crate::config::AppServiceConfig;

use marine_wasm_backend_traits::WasmBackend;
use marine::TomlMarineConfig;

use serde_derive::Serialize;
use serde_derive::Deserialize;

use std::convert::TryInto;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlAppServiceConfig {
    pub service_base_dir: Option<String>,

    #[serde(flatten)]
    pub toml_marine_config: TomlMarineConfig,
}

impl TomlAppServiceConfig {
    /// Load config from filesystem.
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        let file_content = std::fs::read(&path)?;
        toml::from_slice(&file_content).map_err(|e| {
            AppServiceError::ConfigParseError(format!("Error parsing config {:?}: {:?}", path, e))
        })
    }
}

impl<WB: WasmBackend> TryInto<AppServiceConfig<WB>> for TomlAppServiceConfig {
    type Error = AppServiceError;

    fn try_into(self) -> Result<AppServiceConfig<WB>> {
        let marine_config = self.toml_marine_config.try_into()?;
        let service_base_dir = match self.service_base_dir {
            Some(service_base_dir) => PathBuf::from(service_base_dir),
            // use tmp dir for service base dir if it isn't defined
            None => std::env::temp_dir(),
        };

        Ok(AppServiceConfig {
            service_base_dir,
            marine_config,
        })
    }
}
