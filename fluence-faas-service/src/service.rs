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
use super::ServiceError;
use super::IValue;

use fluence_faas::FluenceFaaS;
use fluence_faas::ModulesConfig;

use std::convert::TryInto;

const TMP_DIR_NAME: &str = "tmp";
const SERVICE_ID_ENV_NAME: &str = "service_id";

// TODO: remove and use mutex instead
unsafe impl Send for FluenceFaaSService {}

pub struct FluenceFaaSService {
    faas: FluenceFaaS,
}

impl FluenceFaaSService {
    /// Creates Service with given modules and service id.
    pub fn new<I, C, S>(modules: I, config: C, service_id: S) -> Result<Self>
    where
        I: IntoIterator<Item = String>,
        C: TryInto<ModulesConfig>,
        S: AsRef<str>,
        ServiceError: From<C::Error>,
    {
        let config: ModulesConfig = config.try_into()?;
        let service_id = service_id.as_ref();

        let service_base_dir = Self::prepare_filesystem(&config, service_id)?;
        let config = Self::prepare_wasi(config, &service_base_dir, service_id)?;

        let modules = modules.into_iter().collect();
        let faas = FluenceFaaS::with_module_names(&modules, config)?;

        Ok(Self { faas })
    }

    /// Call a specified function of loaded module by its name.
    // TODO: replace serde_json::Value with Vec<u8>?
    pub fn call_module<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        arguments: serde_json::Value,
    ) -> Result<Vec<IValue>> {
        let arguments = Self::prepare_arguments(arguments)?;

        self.faas
            .call_module(module_name, func_name, &arguments)
            .map_err(Into::into)
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> fluence_faas::FaaSInterface<'_> {
        self.faas.get_interface()
    }

    // returns service base directory
    fn prepare_filesystem(config: &ModulesConfig, service_id: &str) -> Result<String> {
        let base_dir = match &config.service_base_dir {
            Some(base_dir) => base_dir.clone(),
            None => String::new(),
        };

        let service_dir = std::path::Path::new(&base_dir).join(service_id);
        std::fs::create_dir(service_dir)?; // will return an error if dir is already exists

        Ok(base_dir.clone())
    }

    fn prepare_wasi(
        mut config: ModulesConfig,
        service_base_dir: &str,
        service_id: &str,
    ) -> Result<ModulesConfig> {
        let service_id_env =
            vec![format!("{}={}", SERVICE_ID_ENV_NAME, service_base_dir).into_bytes()];
        let preopened_files = vec![String::from(service_id), String::from(TMP_DIR_NAME)];
        let mapped_dirs = vec![
            (String::from("service_dir"), String::from(service_id)),
            (String::from("tmp"), String::from(TMP_DIR_NAME)),
        ];

        config.modules_config = config
            .modules_config
            .into_iter()
            .map(|(name, module_config)| {
                let module_config = module_config
                    .extend_wasi_envs(service_id_env.clone())
                    .extend_wasi_files(preopened_files.clone(), mapped_dirs.clone());

                (name, module_config)
            })
            .collect();

        Ok(config)
    }

    fn prepare_arguments(arguments: serde_json::Value) -> Result<Vec<IValue>> {
        // If arguments are on of: null, [] or {}, avoid calling `to_interface_value`
        let is_null = arguments.is_null();
        let is_empty_arr = arguments.as_array().map_or(false, |a| a.is_empty());
        let is_empty_obj = arguments.as_object().map_or(false, |m| m.is_empty());
        let arguments = if !is_null && !is_empty_arr && !is_empty_obj {
            Some(fluence_faas::to_interface_value(&arguments).map_err(|e| {
                ServiceError::InvalidArguments(format!(
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
            other => Err(ServiceError::InvalidArguments(format!(
                "expected array of interface values: got {:?}",
                other
            ))),
        }
    }
}
