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

use marine_wasm_backend_traits::WasmBackend;

use marine::Marine;
use marine::IValue;
use serde_json::Value as JValue;

use std::convert::TryInto;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::io::ErrorKind;

const SERVICE_ID_ENV_NAME: &str = "service_id";
const SERVICE_LOCAL_DIR_NAME: &str = "local";
const SERVICE_TMP_DIR_NAME: &str = "tmp";

pub struct AppService<WB: WasmBackend> {
    marine: Marine<WB>,
    facade_module_name: String,
}

impl<WB: WasmBackend> AppService<WB> {
    /// Create Service with given modules and service id.
    pub fn new<C, S>(config: C, service_id: S, envs: HashMap<Vec<u8>, Vec<u8>>) -> Result<Self>
    where
        C: TryInto<AppServiceConfig<WB>>,
        S: Into<String>,
        AppServiceError: From<C::Error>,
    {
        let mut config: AppServiceConfig<WB> = config.try_into()?;
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
    ///  1. creating a directory structure in the following form:
    ///     - service_base_dir/service_id/SERVICE_LOCAL_DIR_NAME
    ///     - service_base_dir/service_id/SERVICE_TMP_DIR_NAME
    ///  2. adding service_id to environment variables
    fn set_env_and_dirs(
        config: &mut AppServiceConfig<WB>,
        service_id: String,
        mut envs: HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<()> {
        use maplit::hashmap;

        let create = |dir: &PathBuf| match std::fs::create_dir(dir) {
            Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(err) => Err(AppServiceError::CreateDir {
                err,
                path: dir.clone(),
            }),
            _ => Ok(()),
        };

        let base_dir = &config.service_base_dir;
        let service_dir = base_dir.join(&service_id);
        create(&service_dir)?;

        let local_dir = service_dir.join(SERVICE_LOCAL_DIR_NAME);
        create(&local_dir)?;

        let tmp_dir = service_dir.join(SERVICE_TMP_DIR_NAME);
        create(&tmp_dir)?;

        let local_dir = local_dir.to_string_lossy().to_string();
        let tmp_dir = tmp_dir.to_string_lossy().to_string();

        let mut preopened_files = HashSet::new();
        preopened_files.insert(PathBuf::from(local_dir.clone()));
        preopened_files.insert(PathBuf::from(tmp_dir.clone()));

        let mapped_dirs = hashmap! {
            String::from(SERVICE_LOCAL_DIR_NAME) => PathBuf::from(local_dir),
            String::from(SERVICE_TMP_DIR_NAME) => PathBuf::from(tmp_dir),
        };

        envs.insert(
            SERVICE_ID_ENV_NAME.as_bytes().to_vec(),
            service_id.into_bytes(),
        );

        for module in &mut config.marine_config.modules_config {
            module.config.extend_wasi_envs(envs.clone());
            module
                .config
                .extend_wasi_files(preopened_files.clone(), mapped_dirs.clone());
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
impl<WB: WasmBackend> AppService<WB> {
    pub fn new_with_empty_facade<C, S>(
        config: C,
        service_id: S,
        envs: HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<Self>
    where
        S: Into<String>,
        C: TryInto<AppServiceConfig<WB>>,
        AppServiceError: From<C::Error>,
    {
        let mut config: AppServiceConfig<WB> = config.try_into()?;
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
        C: TryInto<crate::MarineModuleConfig<WB>>,
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
