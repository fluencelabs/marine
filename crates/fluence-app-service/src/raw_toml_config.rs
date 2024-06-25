/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::Result;
use crate::AppServiceError;
use crate::config::AppServiceConfig;

use marine::TomlMarineConfig;
use marine_wasm_backend_traits::WasmBackend;

use serde_derive::Serialize;
use serde_derive::Deserialize;

use std::convert::TryInto;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TomlAppServiceConfig {
    pub service_working_dir: Option<String>,

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
        let service_working_dir = match self.service_working_dir {
            Some(service_working_dir) => PathBuf::from(service_working_dir),
            // use current dir for service base dir if it isn't defined
            None => std::env::current_dir()?,
        };

        Ok(AppServiceConfig {
            service_working_dir,
            marine_config,
        })
    }
}
