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

use crate::generic::AppService;
use crate::generic::AppServiceConfig;
use crate::AppServiceError;

use marine_wasm_backend_traits::WasmBackend;
use marine_wasmtime_backend::WasmtimeConfig;
use marine_wasmtime_backend::WasmtimeWasmBackend;

use std::collections::HashMap;

#[derive(Clone)]
pub struct AppServiceFactory<WB: WasmBackend> {
    backend: WB,
}

#[derive(Clone)]
pub struct EpochTicker(WasmtimeWasmBackend);

impl<WB: WasmBackend> AppServiceFactory<WB> {
    pub async fn new_app_service<S>(
        &self,
        config: AppServiceConfig<WB>,
        service_id: S,
        envs: HashMap<String, String>,
    ) -> crate::Result<AppService<WB>>
    where
        S: Into<String>,
    {
        AppService::new_with_backend(self.backend.clone(), config, service_id, envs).await
    }

    #[cfg(feature = "raw-module-api")]
    pub async fn new_app_service_empty_facade<S>(
        &self,
        config: AppServiceConfig<WB>,
        service_id: S,
        envs: HashMap<String, String>,
    ) -> crate::Result<AppService<WB>>
    where
        S: Into<String>,
    {
        AppService::new_with_empty_facade(self.backend.clone(), config, service_id, envs).await
    }

    pub fn backend(&self) -> WB {
        self.backend.clone()
    }
}

impl AppServiceFactory<WasmtimeWasmBackend> {
    /// Creates a new factory
    pub fn new(
        config: WasmtimeConfig,
    ) -> Result<(AppServiceFactory<WasmtimeWasmBackend>, EpochTicker), AppServiceError> {
        let config = config;
        let backend =
            WasmtimeWasmBackend::new(config).map_err(AppServiceError::WasmBackendError)?;

        let ticker = EpochTicker(backend.clone());
        let factory = Self { backend };
        Ok((factory, ticker))
    }
}

impl EpochTicker {
    pub fn increment_epoch(&self) {
        self.0.increment_epoch()
    }
}
