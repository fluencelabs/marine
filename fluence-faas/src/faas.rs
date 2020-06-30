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
use super::faas_interface::FaaSModuleInterface;
use super::FaaSError;
use super::IValue;

use fce::FCE;
use fce::FCEModuleConfig;

use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;

/// FluenceFaas isn't thread safe.
// impl !Sync for FluenceFaaS {}

// TODO: remove and use mutex instead
unsafe impl Send for FluenceFaaS {}

pub struct FluenceFaaS {
    fce: FCE,

    // names of core modules loaded to FCE
    module_names: Vec<String>,

    // config for code loaded by call_code function
    faas_code_config: FCEModuleConfig,
}

impl FluenceFaaS {
    /// Creates FaaS from config on filesystem.
    pub fn new<P: Into<PathBuf>>(config_file_path: P) -> Result<Self> {
        let config = crate::misc::load_config(config_file_path.into())?;
        Self::with_raw_config(config)
    }

    /// Creates FaaS from config deserialized from TOML.
    pub fn with_raw_config(config: RawCoreModulesConfig) -> Result<Self> {
        let config = crate::misc::from_raw_config(config)?;
        let modules = config
            .core_modules_dir
            .as_ref()
            .map_or(Ok(vec![]), |dir| Self::load_modules(dir))?;
        Self::with_modules(modules, config)
    }

    /// Creates FaaS with given modules.
    pub fn with_modules<I, C>(modules: I, config: C) -> Result<Self>
    where
        I: IntoIterator<Item = (String, Vec<u8>)>,
        C: TryInto<CoreModulesConfig>,
        FaaSError: From<C::Error>,
    {
        let mut fce = FCE::new();
        let mut module_names = Vec::new();
        let mut config = config.try_into()?;

        for (name, bytes) in modules {
            let module_config = crate::misc::make_fce_config(config.modules_config.remove(&name))?;
            fce.load_module(name.clone(), &bytes, module_config)?;
            module_names.push(name);
        }

        let faas_code_config = make_fce_config(config.rpc_module_config)?;

        Ok(Self {
            fce,
            module_names,
            faas_code_config,
        })
    }

    /// Loads modules from a directory at a given path. Non-recursive, ignores subdirectories.
    fn load_modules(core_modules_dir: &str) -> Result<Vec<(String, Vec<u8>)>> {
        use FaaSError::IOError;

        let mut dir_entries = fs::read_dir(core_modules_dir)
            .map_err(|e| IOError(format!("{}: {}", core_modules_dir, e)))?;

        dir_entries.try_fold(vec![], |mut vec, entry| {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                let module_name = path
                    .file_name()
                    .ok_or(IOError(format!("No file name in path {:?}", path)))?
                    .to_os_string()
                    .into_string()
                    .map_err(|name| IOError(format!("invalid file name: {:?}", name)))?;
                let module_bytes = fs::read(path)?;
                vec.push((module_name, module_bytes))
            }

            Ok(vec)
        })
    }

    /// Executes provided Wasm code in the internal environment (with access to module exports).
    pub fn call_code(
        &mut self,
        wasm: &[u8],
        func_name: &str,
        args: &[IValue],
    ) -> Result<Vec<IValue>> {
        // We need this because every wasm code loaded into VM needs a module name
        let anonymous_module = "anonymous_module_name";

        self.fce
            .load_module(anonymous_module, wasm, self.faas_code_config.clone())?;

        let call_result = self.fce.call(anonymous_module, func_name, args)?;
        self.fce.unload_module(anonymous_module)?;

        Ok(call_result)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call_module(
        &mut self,
        module_name: &str,
        func_name: &str,
        args: &[IValue],
    ) -> Result<Vec<IValue>> {
        self.fce
            .call(module_name, func_name, args)
            .map_err(Into::into)
    }

    /// Return all export functions (name and signatures) of loaded on a startup modules.
    pub fn get_interface(&self) -> FaaSInterface {
        let mut modules = Vec::with_capacity(self.module_names.len());

        for module_name in self.module_names.iter() {
            let functions = self.fce.get_interface(module_name).unwrap();
            modules.push(FaaSModuleInterface {
                name: module_name,
                functions,
            })
        }

        FaaSInterface { modules }
    }
}
