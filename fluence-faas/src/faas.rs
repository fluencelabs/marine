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

use crate::config::FaaSConfig;
use crate::faas_interface::FaaSInterface;
use crate::FaaSError;
use crate::Result;
use crate::IValue;
use crate::misc::load_modules_from_fs;
use crate::misc::ModulesLoadStrategy;

use fce::FCE;
use fluence_sdk_main::CallParameters;

use serde_json::Value as JValue;
use std::cell::RefCell;
use std::convert::TryInto;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;
use std::path::PathBuf;

// TODO: remove and use mutex instead
unsafe impl Send for FluenceFaaS {}

pub struct FluenceFaaS {
    /// The Fluence Compute Engine instance.
    fce: FCE,

    /// Parameters of call accessible by Wasm modules.
    call_parameters: Rc<RefCell<CallParameters>>,
}

impl FluenceFaaS {
    /// Creates FaaS from config on filesystem.
    pub fn with_config_path<P: Into<PathBuf>>(config_file_path: P) -> Result<Self> {
        let config = crate::raw_toml_config::TomlFaaSConfig::load(config_file_path.into())?;
        Self::with_raw_config(config)
    }

    /// Creates FaaS from config deserialized from TOML.
    pub fn with_raw_config<C>(config: C) -> Result<Self>
    where
        C: TryInto<FaaSConfig>,
        FaaSError: From<C::Error>,
    {
        let config = config.try_into()?;
        let modules = config
            .modules_dir
            .as_ref()
            .map_or(Ok(HashMap::new()), |dir| {
                load_modules_from_fs(dir, ModulesLoadStrategy::WasmOnly)
            })?;

        Self::with_modules::<FaaSConfig>(modules, config)
    }

    /// Creates FaaS with given modules.
    pub fn with_modules<C>(mut modules: HashMap<String, Vec<u8>>, config: C) -> Result<Self>
    where
        C: TryInto<FaaSConfig>,
        FaaSError: From<C::Error>,
    {
        let mut fce = FCE::new();
        let config = config.try_into()?;
        let call_parameters = Rc::new(RefCell::new(<_>::default()));

        let modules_dir = config.modules_dir;
        for (module_name, module_config) in config.modules_config {
            let module_bytes =
                modules.remove(&module_name).ok_or_else(|| {
                    FaaSError::InstantiationError(format!(
                    "module with name {} is specified in config (dir: {:?}), but not found in provided modules: {:?}",
                    module_name, modules_dir, modules.keys().collect::<Vec<_>>()
                ))
                })?;

            let fce_module_config = crate::misc::make_fce_config(
                module_name.clone(),
                Some(module_config),
                call_parameters.clone(),
            )?;
            fce.load_module(module_name, &module_bytes, fce_module_config)?;
        }

        Ok(Self {
            fce,
            call_parameters,
        })
    }

    /// Searches for modules in `config.modules_dir`, loads only those in the `names` set
    pub fn with_module_names<C>(names: &HashSet<String>, config: C) -> Result<Self>
    where
        C: TryInto<FaaSConfig>,
        FaaSError: From<C::Error>,
    {
        let config = config.try_into()?;
        let modules = config
            .modules_dir
            .as_ref()
            .map_or(Ok(HashMap::new()), |dir| {
                load_modules_from_fs(dir, ModulesLoadStrategy::Named(names))
            })?;

        Self::with_modules::<FaaSConfig>(modules, config)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call_with_ivalues<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        args: &[IValue],
        call_parameters: fluence_sdk_main::CallParameters,
    ) -> Result<Vec<IValue>> {
        self.call_parameters.replace(call_parameters);

        self.fce
            .call(module_name, func_name, args)
            .map_err(Into::into)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call_with_json<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        json_args: JValue,
        call_parameters: fluence_sdk_main::CallParameters,
    ) -> Result<JValue> {
        use crate::misc::json_to_ivalues;
        use crate::misc::ivalues_to_json;

        let module_name = module_name.as_ref();
        let func_name = func_name.as_ref();

        // TODO: cache module interface
        let module_interface = self
            .fce
            .module_interface(module_name)
            .ok_or_else(|| FaaSError::NoSuchModule(module_name.to_string()))?;

        let func_signature = module_interface
            .function_signatures
            .iter()
            .find(|sign| sign.name == func_name)
            .ok_or_else(|| FaaSError::MissingFunctionError(func_name.to_string()))?;

        let record_types = module_interface.record_types.clone();

        let iargs = json_to_ivalues(json_args, func_signature, &record_types)?;
        let outputs = func_signature.outputs.clone();

        self.call_parameters.replace(call_parameters);
        let result = self.fce.call(module_name, func_name, &iargs)?;

        ivalues_to_json(result, &outputs, &record_types)
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> FaaSInterface<'_> {
        let modules = self.fce.interface().collect();

        FaaSInterface { modules }
    }
}

// This API is intended for testing purposes (mostly in FCE REPL)
#[cfg(feature = "raw-module-api")]
impl FluenceFaaS {
    pub fn load_module<S, C>(&mut self, name: S, wasm_bytes: &[u8], config: Option<C>) -> Result<()>
    where
        S: Into<String>,
        C: TryInto<crate::FaaSModuleConfig>,
        FaaSError: From<C::Error>,
    {
        let config = config.map(|c| c.try_into()).transpose()?;
        let name = name.into();

        let fce_module_config =
            crate::misc::make_fce_config(name.clone(), config, self.call_parameters.clone())?;
        self.fce
            .load_module(name, &wasm_bytes, fce_module_config)
            .map_err(Into::into)
    }

    pub fn unload_module<S: AsRef<str>>(&mut self, module_name: S) -> Result<()> {
        self.fce.unload_module(module_name).map_err(Into::into)
    }

    pub fn module_wasi_state<S: AsRef<str>>(
        &mut self,
        module_name: S,
    ) -> Result<&wasmer_wasi::state::WasiState> {
        let module_name = module_name.as_ref();

        self.fce
            .module_wasi_state(module_name)
            .ok_or_else(|| FaaSError::NoSuchModule(module_name.to_string()))
    }
}
