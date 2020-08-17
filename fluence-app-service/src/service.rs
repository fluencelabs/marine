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
use super::AppServiceError;
use super::IValue;

use fluence_faas::FluenceFaaS;
use fluence_faas::ModulesConfig;

use std::convert::TryInto;
use std::path::{Path, PathBuf};

const SERVICE_ID_ENV_NAME: &str = "service_id";
const SERVICE_LOCAL_DIR_NAME: &str = "local";
const SERVICE_TMP_DIR_NAME: &str = "tmp";

// TODO: remove and use mutex instead
unsafe impl Send for AppService {}

pub struct AppService {
    faas: FluenceFaaS,
}

impl AppService {
    /// Create Service with given modules and service id.
    pub fn new<C, S>(config: C, service_id: S, envs: Vec<String>) -> Result<Self>
    where
        C: TryInto<ModulesConfig>,
        S: AsRef<str>,
        AppServiceError: From<C::Error>,
    {
        let config: ModulesConfig = config.try_into()?;
        let service_id = service_id.as_ref();
        let config = Self::set_env_and_dirs(config, service_id, envs)?;

        let faas = FluenceFaaS::with_raw_config(config)?;

        Ok(Self { faas })
    }

    /// Call a specified function of loaded module by its name.
    // TODO: replace serde_json::Value with Vec<u8>?
    pub fn call<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        arguments: serde_json::Value,
    ) -> Result<Vec<IValue>> {
        let arguments = Self::json_to_ivalue(arguments)?;

        self.faas
            .call(module_name, func_name, &arguments)
            .map_err(Into::into)
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> fluence_faas::FaaSInterface<'_> {
        self.faas.get_interface()
    }

    /// Prepare service before starting by:
    ///  1. creating a directory structure in the following form:
    ///     - service_base_dir/service_id/SERVICE_LOCAL_DIR_NAME
    ///     - service_base_dir/service_id/SERVICE_TMP_DIR_NAME
    ///  2. adding service_id to environment variables
    fn set_env_and_dirs(
        mut config: ModulesConfig,
        service_id: &str,
        mut envs: Vec<String>,
    ) -> Result<ModulesConfig> {
        let base_dir: &Path = config
            .service_base_dir
            .as_ref()
            .ok_or(AppServiceError::MissingServiceDir)?
            .as_ref();

        let create = |dir: &PathBuf| {
            std::fs::create_dir(dir).map_err(|err| AppServiceError::dir_exists(err, dir.clone()))
        };

        let service_dir = base_dir.join(service_id);
        create(&service_dir)?;

        let local_dir = service_dir.join(SERVICE_LOCAL_DIR_NAME);
        create(&local_dir)?;

        let tmp_dir = service_dir.join(SERVICE_TMP_DIR_NAME);
        create(&tmp_dir)?;

        let local_dir = local_dir.to_string_lossy().to_string();
        let tmp_dir = tmp_dir.to_string_lossy().to_string();

        let preopened_files = vec![local_dir.clone(), tmp_dir.clone()];
        let mapped_dirs = vec![
            (String::from(SERVICE_LOCAL_DIR_NAME), local_dir),
            (String::from(SERVICE_TMP_DIR_NAME), tmp_dir),
        ];
        envs.push(format!("{}={}", SERVICE_ID_ENV_NAME, service_id));

        config.modules_config = config
            .modules_config
            .into_iter()
            .map(|(name, module_config)| {
                let module_config = module_config
                    .extend_wasi_envs(envs.iter().map(|s| s.clone().into_bytes()).collect())
                    .extend_wasi_files(preopened_files.clone(), mapped_dirs.clone());

                (name, module_config)
            })
            .collect();

        Ok(config)
    }

    fn json_to_ivalue(arguments: serde_json::Value) -> Result<Vec<IValue>> {
        // If arguments are on of: null, [] or {}, avoid calling `to_interface_value`
        let is_null = arguments.is_null();
        let is_empty_arr = arguments.as_array().map_or(false, |a| a.is_empty());
        let is_empty_obj = arguments.as_object().map_or(false, |m| m.is_empty());
        let arguments = if !is_null && !is_empty_arr && !is_empty_obj {
            Some(fluence_faas::to_interface_value(&arguments).map_err(|e| {
                AppServiceError::InvalidConfig(format!(
                    "can't parse arguments as array of interface types: {}",
                    e
                ))
            })?)
        } else {
            None
        };

        match arguments {
            Some(IValue::Record(arguments)) => Ok(arguments.into_vec()),
            // Convert null, [] and {} into vec![]
            None => Ok(vec![]),
            other => Err(AppServiceError::InvalidConfig(format!(
                "expected array of interface values: got {:?}",
                other
            ))),
        }
    }
}

// This API is intended for testing purposes (mostly in FCE REPL)
#[cfg(feature = "raw-module-api")]
impl AppService {
    pub fn load_module<S, C>(&mut self, name: S, wasm_bytes: &[u8], config: Option<C>) -> Result<()>
    where
        S: Into<String>,
        C: TryInto<crate::ModuleConfig>,
        fluence_faas::FaaSError: From<C::Error>,
    {
        self.faas
            .load_module(name, &wasm_bytes, config)
            .map_err(Into::into)
    }

    pub fn unload_module<S: AsRef<str>>(&mut self, module_name: S) -> Result<()> {
        self.faas.unload_module(module_name).map_err(Into::into)
    }

    pub fn get_wasi_state<S: AsRef<str>>(
        &mut self,
        module_name: S,
    ) -> Result<&wasmer_wasi::state::WasiState> {
        self.faas.module_wasi_state(module_name).map_err(Into::into)
    }
}
