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
        let config = Self::prepare_before_creation(config, service_id)?;

        let modules = modules.into_iter().collect();
        let faas = FluenceFaaS::with_module_names(&modules, config)?;

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
        let arguments = Self::prepare_arguments(arguments)?;

        self.faas
            .call(module_name, func_name, &arguments)
            .map_err(Into::into)
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> fluence_faas::FaaSInterface<'_> {
        self.faas.get_interface()
    }

    fn prepare_before_creation(
        mut config: ModulesConfig,
        service_id: &str,
    ) -> Result<ModulesConfig> {
        let base_dir = match config.service_base_dir {
            Some(ref base_dir) => base_dir,
            // TODO: refactor it later
            None => {
                return Err(ServiceError::IOError(String::from(
                    "service_base_dir should be specified",
                )))
            }
        };

        let service_dir = std::path::Path::new(base_dir).join(service_id);
        std::fs::create_dir(service_dir.clone())?; // will return an error if dir is already exists

        let service_id_env = vec![format!("{}={}", SERVICE_ID_ENV_NAME, service_id).into_bytes()];
        let preopened_files = vec![];
        let mapped_dirs = vec![(
            String::from("service_dir"),
            service_dir.to_string_lossy().into(),
        )];

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

#[cfg(feature = "module-raw-api")]
impl FluenceFaaSService {
    fn load_module<S, C>(
        &mut self,
        module_name: S,
        wasm_bytes: &[u8],
        config: Option<C>,
    ) -> Result<()>
    where
        S: Into<String>,
        C: TryInto<crate::ModuleConfig>,
    {
        let mut config = config.try_into()?;

        let fce_module_config = crate::misc::make_fce_config(module_config, None)?;
        self.fce
            .load_module(name.clone(), &bytes, fce_module_config)
            .map_err(Into::into)
    }

    fn unload_module<S: AsRef<str>>(&mut self, module_name: S) -> Result<()> {
        self.fce.unload_module(module_name).map_err(Into::into)
    }
}

// This API is intended for testing purposes (mostly in FCE REPL)
#[cfg(feature = "raw-module-api")]
impl FluenceFaaSService {
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
}

#[cfg(feature = "raw-module-api")]
impl Default for FluenceFaaSService {
    fn default() -> Self {
        Self {
            faas: <_>::default(),
        }
    }
}
