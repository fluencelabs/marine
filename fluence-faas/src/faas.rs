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
use crate::FaaSResult;
use crate::IValue;
use crate::IType;
use crate::MemoryStats;
use crate::module_loading::load_modules_from_fs;
use crate::module_loading::ModulesLoadStrategy;
use crate::host_imports::logger::LoggerFilter;
use crate::host_imports::logger::WASM_LOG_ENV_NAME;
use crate::json_to_faas_err;

use marine::Marine;
use marine::IFunctionArg;
use marine_utils::SharedString;
use marine::MRecordTypes;
use marine_rs_sdk::CallParameters;

use serde_json::Value as JValue;
use std::cell::RefCell;
use std::convert::TryInto;
use std::collections::HashMap;
use std::rc::Rc;

type MFunctionSignature = (Rc<Vec<IFunctionArg>>, Rc<Vec<IType>>);
type MModuleInterface = (Rc<Vec<IFunctionArg>>, Rc<Vec<IType>>, Rc<MRecordTypes>);

struct ModuleInterface {
    function_signatures: HashMap<SharedString, MFunctionSignature>,
    record_types: Rc<MRecordTypes>,
}

// TODO: remove and use mutex instead
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for FluenceFaaS {}

pub struct FluenceFaaS {
    /// Marine instance.
    marine: Marine,

    /// Parameters of call accessible by Wasm modules.
    call_parameters: Rc<RefCell<CallParameters>>,

    /// Cached module interfaces by names.
    module_interfaces_cache: HashMap<String, ModuleInterface>,
}

impl FluenceFaaS {
    /// Creates FaaS from config deserialized from TOML.
    pub fn with_raw_config<C>(config: C) -> FaaSResult<Self>
    where
        C: TryInto<FaaSConfig>,
        FaaSError: From<C::Error>,
    {
        let config = config.try_into()?;
        let modules = config
            .modules_config
            .iter()
            .map(|m| (m.file_name.clone(), m.import_name.clone()))
            .collect();
        Self::with_module_names::<FaaSConfig>(&modules, config)
    }

    /// Creates FaaS with given modules.
    pub fn with_modules<C>(mut modules: HashMap<String, Vec<u8>>, config: C) -> FaaSResult<Self>
    where
        C: TryInto<FaaSConfig>,
        FaaSError: From<C::Error>,
    {
        let mut marine = Marine::new();
        let config = config.try_into()?;
        let call_parameters = Rc::new(RefCell::new(<_>::default()));

        let modules_dir = config.modules_dir;

        // LoggerFilter can be initialized with an empty string
        let wasm_log_env = std::env::var(WASM_LOG_ENV_NAME).unwrap_or_default();
        let logger_filter = LoggerFilter::from_env_string(&wasm_log_env);

        for module in config.modules_config {
            let module_bytes = modules.remove(&module.import_name).ok_or_else(|| {
                FaaSError::InstantiationError {
                    module_import_name: module.import_name.clone(),
                    modules_dir: modules_dir.clone(),
                    provided_modules: modules.keys().cloned().collect::<Vec<_>>(),
                }
            })?;

            let marine_module_config = crate::config::make_marine_config(
                module.import_name.clone(),
                Some(module.config),
                call_parameters.clone(),
                &logger_filter,
            )?;
            marine.load_module(module.import_name, &module_bytes, marine_module_config)?;
        }

        Ok(Self {
            marine,
            call_parameters,
            module_interfaces_cache: HashMap::new(),
        })
    }

    /// Searches for modules in `config.modules_dir`, loads only those in the `names` set
    pub fn with_module_names<C>(names: &HashMap<String, String>, config: C) -> FaaSResult<Self>
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
    pub fn call_with_ivalues(
        &mut self,
        module_name: impl AsRef<str>,
        func_name: impl AsRef<str>,
        args: &[IValue],
        call_parameters: marine_rs_sdk::CallParameters,
    ) -> FaaSResult<Vec<IValue>> {
        self.call_parameters.replace(call_parameters);

        self.marine
            .call(module_name, func_name, args)
            .map_err(Into::into)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call_with_json(
        &mut self,
        module_name: impl AsRef<str>,
        func_name: impl AsRef<str>,
        json_args: JValue,
        call_parameters: marine_rs_sdk::CallParameters,
    ) -> FaaSResult<JValue> {
        use it_json_serde::json_to_ivalues;
        use it_json_serde::ivalues_to_json;

        let module_name = module_name.as_ref();
        let func_name = func_name.as_ref();

        let (func_signature, output_types, record_types) =
            self.lookup_module_interface(module_name, func_name)?;
        let iargs = json_to_faas_err!(
            json_to_ivalues(
                json_args,
                func_signature.iter().map(|arg| (&arg.name, &arg.ty)),
                &record_types,
            ),
            module_name.to_string(),
            func_name.to_string()
        )?;

        self.call_parameters.replace(call_parameters);
        let result = self.marine.call(module_name, func_name, &iargs)?;

        json_to_faas_err!(
            ivalues_to_json(result, &output_types, &record_types),
            module_name.to_string(),
            func_name.to_string()
        )
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> FaaSInterface<'_> {
        let modules = self.marine.interface().collect();

        FaaSInterface { modules }
    }

    /// Return statistic of Wasm modules heap footprint.
    pub fn module_memory_stats(&self) -> MemoryStats<'_> {
        self.marine.module_memory_stats()
    }

    /// At first, tries to find function signature and record types in module_interface_cache,
    /// if there is no them, tries to look
    fn lookup_module_interface<'faas>(
        &'faas mut self,
        module_name: &str,
        func_name: &str,
    ) -> FaaSResult<MModuleInterface> {
        use FaaSError::NoSuchModule;
        use FaaSError::MissingFunctionError;

        if let Some(module_interface) = self.module_interfaces_cache.get(module_name) {
            if let Some(function) = module_interface.function_signatures.get(func_name) {
                return Ok((
                    function.0.clone(),
                    function.1.clone(),
                    module_interface.record_types.clone(),
                ));
            }

            return Err(MissingFunctionError(func_name.to_string()));
        }

        let module_interface = self
            .marine
            .module_interface(module_name)
            .ok_or_else(|| NoSuchModule(module_name.to_string()))?;

        let function_signatures = module_interface
            .function_signatures
            .iter()
            .cloned()
            .map(|f| (SharedString(f.name), (f.arguments, f.outputs)))
            .collect::<HashMap<_, _>>();

        let (arg_types, output_types) = function_signatures
            .get(func_name)
            .ok_or_else(|| MissingFunctionError(func_name.to_string()))?;

        let arg_types = arg_types.clone();
        let output_types = output_types.clone();
        let record_types = Rc::new(module_interface.record_types.clone());

        let module_interface = ModuleInterface {
            function_signatures,
            record_types: record_types.clone(),
        };

        self.module_interfaces_cache
            .insert(func_name.to_string(), module_interface);

        Ok((arg_types, output_types, record_types))
    }
}

// This API is intended for testing purposes (mostly in Marine REPL)
#[cfg(feature = "raw-module-api")]
impl FluenceFaaS {
    pub fn load_module<C, S>(
        &mut self,
        name: S,
        wasm_bytes: &[u8],
        config: Option<C>,
    ) -> FaaSResult<()>
    where
        S: Into<String>,
        C: TryInto<crate::FaaSModuleConfig>,
        FaaSError: From<C::Error>,
    {
        let config = config.map(|c| c.try_into()).transpose()?;
        let name = name.into();

        // LoggerFilter can be initialized with an empty string
        let wasm_log_env = std::env::var(WASM_LOG_ENV_NAME).unwrap_or_default();
        let logger_filter = LoggerFilter::from_env_string(&wasm_log_env);

        let marine_module_config = crate::config::make_marine_config(
            name.clone(),
            config,
            self.call_parameters.clone(),
            &logger_filter,
        )?;
        self.marine
            .load_module(name, wasm_bytes, marine_module_config)
            .map_err(Into::into)
    }

    pub fn unload_module(&mut self, module_name: impl AsRef<str>) -> FaaSResult<()> {
        self.marine.unload_module(module_name).map_err(Into::into)
    }

    pub fn module_wasi_state(
        &mut self,
        module_name: impl AsRef<str>,
    ) -> FaaSResult<&wasmer_wasi::state::WasiState> {
        let module_name = module_name.as_ref();

        self.marine
            .module_wasi_state(module_name)
            .ok_or_else(|| FaaSError::NoSuchModule(module_name.to_string()))
    }
}
