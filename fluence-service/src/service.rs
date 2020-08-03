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

use crate::misc::{CoreModulesConfig, make_fce_config};
use crate::RawCoreModulesConfig;
use crate::Result;

use super::faas_interface::FaaSInterface;
use super::FaaSError;
use super::IValue;

use fce::FCE;
use fce::FCEModuleConfig;
use fluence_faas::FluenceFaaS;

use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;
use crate::faas_interface::FaaSFunctionSignature;
use std::collections::HashSet;

// TODO: remove and use mutex instead
unsafe impl Send for Service {}

pub struct Service {
    faas: FluenceFaaS,

    // config for code loaded by call_code function
    faas_code_config: FCEModuleConfig,
}

impl Service {
    /// Creates FaaS with given modules.
    pub fn new<I, C, S>(modules: I, config: C, service_id: S) -> Result<Self>
    where
        I: IntoIterator<Item = (String, Vec<u8>)>,
        C: TryInto<CoreModulesConfig>,
        S: AsRef<str>,
        FaaSError: From<C::Error>,
    {
        let mut fce = FCE::new();
        let mut config = config.try_into()?;

        for (name, bytes) in modules {
            let module_config = crate::misc::make_fce_config(config.modules_config.remove(&name))?;
            fce.load_module(name.clone(), &bytes, module_config)?;
        }

        let faas_code_config = make_fce_config(config.rpc_module_config)?;
        let faas = FluenceFaaS::with_modules(modules, config)?;

        Ok(Self {
            faas,
            faas_code_config,
        })
    }

    /// Loads modules from a directory at a given path. Non-recursive, ignores subdirectories.
    fn load_modules(
        core_modules_dir: &str,
        modules: ModulesLoadStrategy,
    ) -> Result<Vec<(String, Vec<u8>)>> {
        use FaaSError::IOError;

        let mut dir_entries = fs::read_dir(core_modules_dir)
            .map_err(|e| IOError(format!("{}: {}", core_modules_dir, e)))?;

        let loaded = dir_entries.try_fold(vec![], |mut vec, entry| {
            let entry = entry?;
            let path = entry.path();
            // Skip directories
            if path.is_dir() {
                return Ok(vec);
            }

            let module_name = path
                .file_name()
                .ok_or_else(|| IOError(format!("No file name in path {:?}", path)))?
                .to_os_string()
                .into_string()
                .map_err(|name| IOError(format!("invalid file name: {:?}", name)))?;

            if modules.should_load(&module_name) {
                let module_bytes = fs::read(path)?;
                vec.push((module_name, module_bytes));
            }

            Result::Ok(vec)
        })?;

        if modules.required_modules_len() > loaded.len() {
            let loaded = loaded.iter().map(|(n, _)| n);
            let not_found = modules.missing_modules(loaded);
            return Err(FaaSError::ConfigParseError(format!(
                "the following modules were not found: {:?}",
                not_found
            )));
        }

        Ok(loaded)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call_module<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        args: &[IValue],
    ) -> Result<Vec<IValue>> {
        self.fce
            .call(module_name, func_name, args)
            .map_err(Into::into)
    }

    /// Return all export functions (name and signatures) of loaded on a startup modules.
    pub fn get_interface(&self) -> FaaSInterface {
        let modules = self
            .fce
            .interface()
            .map(|(name, signatures)| {
                let signatures = signatures
                    .iter()
                    .map(|f| {
                        (
                            f.name,
                            FaaSFunctionSignature {
                                input_types: f.input_types,
                                output_types: f.output_types,
                            },
                        )
                    })
                    .collect();
                (name, signatures)
            })
            .collect();

        FaaSInterface { modules }
    }
}
