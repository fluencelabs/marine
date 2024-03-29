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
use crate::config::AppServiceConfig;
use crate::MemoryStats;
use crate::service_interface::ServiceInterface;
use super::AppServiceError;

#[cfg(feature = "raw-module-api")]
use marine_wasm_backend_traits::WasiState;
use marine::Marine;
use marine::MarineModuleConfig;
use marine::IValue;

use serde_json::Value as JValue;

use std::convert::TryInto;
use std::collections::HashMap;
use std::path::Path;
use std::io::ErrorKind;

const SERVICE_ID_ENV_NAME: &str = "service_id";

pub struct AppService {
    marine: Marine,
    facade_module_name: String,
}

impl AppService {
    /// Create Service with given modules and service id.
    pub fn new<C, S>(config: C, service_id: S, envs: HashMap<String, String>) -> Result<Self>
    where
        C: TryInto<AppServiceConfig>,
        S: Into<String>,
        AppServiceError: From<C::Error>,
    {
        let mut config: AppServiceConfig = config.try_into()?;
        let facade_module_name = config
            .marine_config
            .modules_config
            .last()
            .ok_or_else(|| {
                AppServiceError::ConfigParseError(String::from(
                    "config should contain at least one module",
                ))
            })?
            .import_name
            .clone();

        let service_id = service_id.into();
        Self::set_env_and_dirs(&mut config, service_id, envs)?;

        let marine = Marine::with_raw_config(config.marine_config)?;

        Ok(Self {
            marine,
            facade_module_name,
        })
    }

    /// Call a specified function of loaded module by its name with arguments in json format.
    pub fn call(
        &mut self,
        func_name: impl AsRef<str>,
        arguments: JValue,
        call_parameters: crate::CallParameters,
    ) -> Result<JValue> {
        self.marine
            .call_with_json(
                &self.facade_module_name,
                func_name,
                arguments,
                call_parameters,
            )
            .map_err(Into::into)
    }

    /// Call a specified function of loaded module by its name with arguments in IValue format.
    pub fn call_with_ivalues(
        &mut self,
        func_name: impl AsRef<str>,
        arguments: &[IValue],
        call_parameters: crate::CallParameters,
    ) -> Result<Vec<IValue>> {
        self.marine
            .call_with_ivalues(
                &self.facade_module_name,
                func_name,
                arguments,
                call_parameters,
            )
            .map_err(Into::into)
    }

    /// Return interface (function signatures and record types) of this service.
    pub fn get_interface(&self) -> ServiceInterface {
        use crate::service_interface::into_service_interface;

        let marine_facade_interface = self
            .marine
            .get_interface()
            .modules
            .remove(self.facade_module_name.as_str())
            // facade module must be loaded into FaaS, so unwrap is safe here
            .unwrap();

        into_service_interface(marine_facade_interface)
    }

    /// Prepare service before starting by:
    ///  1. rooting all mapped directories at service_working_dir, keeping absolute paths as-is
    ///  2. adding service_id to environment variables
    fn set_env_and_dirs(
        config: &mut AppServiceConfig,
        service_id: String,
        mut envs: HashMap<String, String>,
    ) -> Result<()> {
        let working_dir = &config.service_working_dir;

        envs.insert(SERVICE_ID_ENV_NAME.to_string(), service_id);

        for module in &mut config.marine_config.modules_config {
            module.config.extend_wasi_envs(envs.clone());
            // Moves relative paths in mapped dirs to the &working dir, keeping old aliases.
            module.config.root_wasi_files_at(working_dir);

            // Create all mapped directories if they do not exist
            // Needed to provide ability to run the same services both in mrepl and rust-peer
            create_wasi_dirs(&module.config)?;
        }
        Ok(())
    }

    /// Return statistics of Wasm modules heap footprint.
    /// This operation is cheap.
    pub fn module_memory_stats(&self) -> MemoryStats<'_> {
        self.marine.module_memory_stats()
    }
}

// This API is intended for testing purposes (mostly in Marine REPL)
#[cfg(feature = "raw-module-api")]
impl AppService {
    pub fn new_with_empty_facade<C, S>(
        config: C,
        service_id: S,
        envs: HashMap<String, String>,
    ) -> Result<Self>
    where
        S: Into<String>,
        C: TryInto<AppServiceConfig>,
        AppServiceError: From<C::Error>,
    {
        let mut config: AppServiceConfig = config.try_into()?;
        let service_id = service_id.into();
        Self::set_env_and_dirs(&mut config, service_id, envs)?;

        let marine = Marine::with_raw_config(config.marine_config)?;

        Ok(Self {
            marine,
            facade_module_name: String::new(),
        })
    }

    pub fn call_module(
        &mut self,
        module_name: impl AsRef<str>,
        func_name: impl AsRef<str>,
        arguments: JValue,
        call_parameters: crate::CallParameters,
    ) -> Result<JValue> {
        self.marine
            .call_with_json(module_name, func_name, arguments, call_parameters)
            .map_err(Into::into)
    }

    pub fn load_module<C, S>(&mut self, name: S, wasm_bytes: &[u8], config: Option<C>) -> Result<()>
    where
        S: Into<String>,
        C: TryInto<marine::MarineModuleConfig>,
        marine::MarineError: From<C::Error>,
    {
        self.marine
            .load_module(name, wasm_bytes, config)
            .map_err(Into::into)
    }

    pub fn unload_module(&mut self, module_name: impl AsRef<str>) -> Result<()> {
        self.marine.unload_module(module_name).map_err(Into::into)
    }

    /// Return raw interface of the underlying [[Marine]] instance
    pub fn get_full_interface(&self) -> marine::MarineInterface<'_> {
        self.marine.get_interface()
    }

    /// Return
    pub fn get_wasi_state(
        &mut self,
        module_name: impl AsRef<str>,
    ) -> Result<Box<dyn WasiState + '_>> {
        self.marine
            .module_wasi_state(module_name)
            .map_err(Into::into)
    }
}

fn create_wasi_dirs(config: &MarineModuleConfig) -> Result<()> {
    if let Some(wasi_config) = &config.wasi {
        for dir in wasi_config.mapped_dirs.values() {
            create(dir)?;
        }
    }

    Ok(())
}

fn create(dir: &Path) -> Result<()> {
    match std::fs::create_dir_all(dir) {
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(err) => Err(AppServiceError::CreateDir {
            err,
            path: dir.to_owned(),
        }),
        _ => Ok(()),
    }
}
