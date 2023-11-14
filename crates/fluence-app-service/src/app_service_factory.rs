use marine_wasm_backend_traits::{WasmBackend};
use std::collections::HashMap;

use crate::{AppService, AppServiceConfig};

#[derive(Clone)]
pub struct AppServiceFactory<WB: WasmBackend> {
    backend: WB,
}

use marine_wasm_backend_traits::WasmBackendResult;
use marine_wasmtime_backend::WasmtimeWasmBackend;

impl AppServiceFactory<WasmtimeWasmBackend> {
    pub fn new() -> WasmBackendResult<Self> {
        let backend = WasmtimeWasmBackend::new_async_epoch_based()?;
        Ok(Self { backend })
    }

    pub async fn new_app_service<S>(
        &self,
        config: AppServiceConfig,
        service_id: S,
        envs: HashMap<String, String>,
    ) -> crate::Result<AppService>
    where
        S: Into<String>,
    {
        AppService::new_with_backend(self.backend.clone(), config, service_id, envs).await
    }

    #[cfg(feature = "raw-module-api")]
    pub async fn new_app_service_empty_facade<S>(
        &self,
        config: AppServiceConfig,
        service_id: S,
        envs: HashMap<String, String>,
    ) -> crate::Result<AppService>
    where
        S: Into<String>,
    {
        AppService::new_with_empty_facade(self.backend.clone(), config, service_id, envs).await
    }

    pub fn increment_epoch(&self) {
        self.backend.increment_epoch()
    }
}
