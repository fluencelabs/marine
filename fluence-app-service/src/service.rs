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
use crate::HeapStatistic;
use crate::service_interface::ServiceInterface;
use super::AppServiceError;

use fluence_faas::FluenceFaaS;
use fluence_faas::IValue;
use serde_json::Value as JValue;

use std::convert::TryInto;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::io::ErrorKind;

const SERVICE_ID_ENV_NAME: &str = "service_id";
const SERVICE_LOCAL_DIR_NAME: &str = "local";
const SERVICE_TMP_DIR_NAME: &str = "tmp";

pub struct AppService {
    faas: FluenceFaaS,
    facade_module_name: String,
}

impl AppService {
    /// Create Service with given modules and service id.
    pub fn new<C, S>(config: C, service_id: S, envs: HashMap<Vec<u8>, Vec<u8>>) -> Result<Self>
    where
        C: TryInto<AppServiceConfig>,
        S: Into<String>,
        AppServiceError: From<C::Error>,
    {
        let mut config: AppServiceConfig = config.try_into()?;
        let facade_module_name = config
            .faas_config
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

        let faas = FluenceFaaS::with_raw_config(config.faas_config)?;

        Ok(Self {
            faas,
            facade_module_name,
        })
    }

    /// Call a specified function of loaded module by its name with arguments in json format.
    // TODO: replace serde_json::Value with Vec<u8>?
    pub fn call<S: AsRef<str>>(
        &mut self,
        func_name: S,
        arguments: JValue,
        call_parameters: crate::CallParameters,
    ) -> Result<JValue> {
        self.faas
            .call_with_json(
                &self.facade_module_name,
                func_name,
                arguments,
                call_parameters,
            )
            .map_err(Into::into)
    }

    /// Call a specified function of loaded module by its name with arguments in IValue format.
    pub fn call_with_ivalues<S: AsRef<str>>(
        &mut self,
        func_name: S,
        arguments: &[IValue],
        call_parameters: crate::CallParameters,
    ) -> Result<Vec<IValue>> {
        self.faas
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

        let faas_facade_interface = self
            .faas
            .get_interface()
            .modules
            .remove(self.facade_module_name.as_str())
            // facade module must be loaded into FaaS, so unwrap is safe here
            .unwrap();

        into_service_interface(faas_facade_interface)
    }

    /// Prepare service before starting by:
    ///  1. creating a directory structure in the following form:
    ///     - service_base_dir/service_id/SERVICE_LOCAL_DIR_NAME
    ///     - service_base_dir/service_id/SERVICE_TMP_DIR_NAME
    ///  2. adding service_id to environment variables
    fn set_env_and_dirs(
        config: &mut AppServiceConfig,
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

        for module in &mut config.faas_config.modules_config {
            module.config.extend_wasi_envs(envs.clone());
            module
                .config
                .extend_wasi_files(preopened_files.clone(), mapped_dirs.clone());
        }

        Ok(())
    }
}

// This API is intended for testing purposes (mostly in Marine REPL)
#[cfg(feature = "raw-module-api")]
impl AppService {
    pub fn new_with_empty_facade<C, S>(
        config: C,
        service_id: S,
        envs: HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<Self>
    where
        C: TryInto<AppServiceConfig>,
        S: Into<String>,
        AppServiceError: From<C::Error>,
    {
        let mut config: AppServiceConfig = config.try_into()?;
        let service_id = service_id.into();
        Self::set_env_and_dirs(&mut config, service_id, envs)?;

        let faas = FluenceFaaS::with_raw_config(config.faas_config)?;

        Ok(Self {
            faas,
            facade_module_name: String::new(),
        })
    }

    pub fn call_module<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        arguments: JValue,
        call_parameters: crate::CallParameters,
    ) -> Result<JValue> {
        self.faas
            .call_with_json(module_name, func_name, arguments, call_parameters)
            .map_err(Into::into)
    }

    pub fn load_module<S, C>(&mut self, name: S, wasm_bytes: &[u8], config: Option<C>) -> Result<()>
    where
        S: Into<String>,
        C: TryInto<crate::FaaSModuleConfig>,
        fluence_faas::FaaSError: From<C::Error>,
    {
        self.faas
            .load_module(name, wasm_bytes, config)
            .map_err(Into::into)
    }

    pub fn unload_module<S: AsRef<str>>(&mut self, module_name: S) -> Result<()> {
        self.faas.unload_module(module_name).map_err(Into::into)
    }

    /// Return raw interface of the underlying [[FluenceFaaS]] instance
    pub fn get_full_interface(&self) -> fluence_faas::FaaSInterface<'_> {
        self.faas.get_interface()
    }

    pub fn get_wasi_state<S: AsRef<str>>(
        &mut self,
        module_name: S,
    ) -> Result<&wasmer_wasi::state::WasiState> {
        self.faas.module_wasi_state(module_name).map_err(Into::into)
    }

    /// Return statistic of Wasm modules heap footprint.
    pub fn heap_statistic(&self) -> HeapStatistic<'_> {
        self.faas.heap_statistic()
    }
}
