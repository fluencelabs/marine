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

use crate::config::MarineConfig;
use crate::marine_interface::MarineInterface;
use crate::MarineError;
use crate::MarineResult;
use crate::IValue;
use crate::IType;
use crate::MemoryStats;
use crate::module_loading::load_modules_from_fs;
use crate::host_imports::logger::LoggerFilter;
use crate::host_imports::logger::WASM_LOG_ENV_NAME;
use crate::host_imports::call_parameters_v3_to_v0;
use crate::host_imports::call_parameters_v3_to_v1;
use crate::host_imports::call_parameters_v3_to_v2;
use crate::json_to_marine_err;

use marine_wasm_backend_traits::WasmBackend;
#[cfg(feature = "raw-module-api")]
use marine_wasm_backend_traits::WasiState;

use marine_core::MError;
use marine_core::generic::MarineCore;
use marine_core::IFunctionArg;
use marine_core::MarineCoreConfig;
use marine_core::MRecordTypes;
use marine_utils::SharedString;
use marine_rs_sdk::CallParameters;

use parking_lot::Mutex;
use serde_json::Value as JValue;

use std::convert::TryInto;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

type MFunctionSignature = (Arc<Vec<IFunctionArg>>, Arc<Vec<IType>>);
type MModuleInterface = (Arc<Vec<IFunctionArg>>, Arc<Vec<IType>>, Arc<MRecordTypes>);

struct ModuleInterface {
    function_signatures: HashMap<SharedString, MFunctionSignature>,
    record_types: Arc<MRecordTypes>,
}

pub struct Marine<WB: WasmBackend> {
    /// Marine instance.
    core: MarineCore<WB>,

    /// Parameters of call accessible by Wasm modules.
    call_parameters_v0: Arc<Mutex<marine_call_parameters_v0::CallParameters>>,

    call_parameters_v1: Arc<Mutex<marine_call_parameters_v1::CallParameters>>,

    call_parameters_v2: Arc<Mutex<marine_call_parameters_v2::CallParameters>>,

    /// Parameters of call accessible by Wasm modules.
    call_parameters_v3: Arc<Mutex<CallParameters>>,

    /// Cached module interfaces by names.
    module_interfaces_cache: HashMap<String, ModuleInterface>,
}

impl<WB: WasmBackend> Marine<WB> {
    /// Creates Marine from config deserialized from TOML.
    pub async fn with_raw_config<C>(backend: WB, config: C) -> MarineResult<Self>
    where
        C: TryInto<MarineConfig<WB>>,
        MarineError: From<C::Error>,
    {
        let config = config.try_into()?;
        let modules = config
            .modules_config
            .iter()
            .map(|m| -> MarineResult<(String, PathBuf)> {
                Ok((m.import_name.clone(), m.get_path(&config.modules_dir)?))
            })
            .collect::<MarineResult<HashMap<String, PathBuf>>>()?;

        Self::with_module_names::<MarineConfig<WB>>(backend, &modules, config).await
    }

    /// Creates Marine with given modules.
    pub async fn with_modules<C>(
        backend: WB,
        mut modules: HashMap<String, Vec<u8>>,
        config: C,
    ) -> MarineResult<Self>
    where
        C: TryInto<MarineConfig<WB>>,
        MarineError: From<C::Error>,
    {
        let config = config.try_into()?;
        let core_config = MarineCoreConfig::new(backend, config.total_memory_limit);
        let mut marine = MarineCore::new(core_config)?;
        let call_parameters_v0 = Arc::<Mutex<marine_call_parameters_v0::CallParameters>>::default();
        let call_parameters_v1 = Arc::<Mutex<marine_call_parameters_v1::CallParameters>>::default();
        let call_parameters_v2 = Arc::<Mutex<marine_call_parameters_v2::CallParameters>>::default();
        let call_parameters_v3 = Arc::<Mutex<CallParameters>>::default();

        let modules_dir = config.modules_dir;

        // LoggerFilter can be initialized with an empty string
        let wasm_log_env = std::env::var(WASM_LOG_ENV_NAME).unwrap_or_default();
        let logger_filter = LoggerFilter::from_env_string(&wasm_log_env);

        for module in config.modules_config {
            let module_bytes = modules.remove(&module.import_name).ok_or_else(|| {
                MarineError::InstantiationError {
                    module_import_name: module.import_name.clone(),
                    modules_dir: modules_dir.clone(),
                    provided_modules: modules.keys().cloned().collect::<Vec<_>>(),
                }
            })?;

            let marine_module_config = crate::config::make_marine_config(
                module.import_name.clone(),
                Some(module.config),
                call_parameters_v0.clone(),
                call_parameters_v1.clone(),
                call_parameters_v2.clone(),
                call_parameters_v3.clone(),
                &logger_filter,
            )?;

            marine
                .load_module(module.import_name, &module_bytes, marine_module_config)
                .await
                .map_err(|e| check_for_oom_and_convert_error(&marine, e))?;
        }

        Ok(Self {
            core: marine,
            call_parameters_v0,
            call_parameters_v1,
            call_parameters_v2,
            call_parameters_v3,
            module_interfaces_cache: HashMap::new(),
        })
    }

    /// Searches for modules in `config.modules_dir`, loads only those in the `names` set
    pub async fn with_module_names<C>(
        backend: WB,
        names: &HashMap<String, PathBuf>,
        config: C,
    ) -> MarineResult<Self>
    where
        C: TryInto<MarineConfig<WB>>,
        MarineError: From<C::Error>,
    {
        let config = config.try_into()?;
        let modules = load_modules_from_fs(names)?;

        Self::with_modules::<MarineConfig<WB>>(backend, modules, config).await
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub async fn call_with_ivalues_async(
        &mut self,
        module_name: impl AsRef<str>,
        func_name: impl AsRef<str>,
        args: &[IValue],
        call_parameters: marine_rs_sdk::CallParameters,
    ) -> MarineResult<Vec<IValue>> {
        self.update_call_parameters(call_parameters);

        let result = self
            .core
            .call_async(module_name, func_name, args)
            .await
            .map_err(|e| check_for_oom_and_convert_error(&self.core, e))?;

        self.core.clear_allocation_stats();

        Ok(result)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub async fn call_with_json_async(
        &mut self,
        module_name: impl AsRef<str>,
        func_name: impl AsRef<str>,
        json_args: JValue,
        call_parameters: marine_rs_sdk::CallParameters,
    ) -> MarineResult<JValue> {
        use it_json_serde::json_to_ivalues;
        use it_json_serde::ivalues_to_json;

        let module_name = module_name.as_ref();
        let func_name = func_name.as_ref();

        let (func_signature, output_types, record_types) =
            self.lookup_module_interface(module_name, func_name)?;
        let iargs = json_to_marine_err!(
            json_to_ivalues(
                json_args,
                func_signature.iter().map(|arg| (&arg.name, &arg.ty)),
                &record_types,
            ),
            module_name.to_string(),
            func_name.to_string()
        )?;

        self.update_call_parameters(call_parameters);

        let result = self
            .core
            .call_async(module_name, func_name, &iargs)
            .await
            .map_err(|e| check_for_oom_and_convert_error(&self.core, e))?;

        self.core.clear_allocation_stats();

        json_to_marine_err!(
            ivalues_to_json(result, &output_types, &record_types),
            module_name.to_string(),
            func_name.to_string()
        )
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> MarineInterface<'_> {
        let modules = self.core.interface().collect();

        MarineInterface { modules }
    }

    /// Return statistic of Wasm modules heap footprint.
    pub fn module_memory_stats(&self) -> MemoryStats<'_> {
        self.core.module_memory_stats()
    }

    /// At first, tries to find function signature and record types in module_interface_cache,
    /// if there is no them, tries to look
    fn lookup_module_interface(
        &mut self,
        module_name: &str,
        func_name: &str,
    ) -> MarineResult<MModuleInterface> {
        use MarineError::NoSuchModule;
        use MarineError::MissingFunctionError;

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
            .core
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
        let record_types = Arc::new(module_interface.record_types.clone());

        let module_interface = ModuleInterface {
            function_signatures,
            record_types: record_types.clone(),
        };

        self.module_interfaces_cache
            .insert(func_name.to_string(), module_interface);

        Ok((arg_types, output_types, record_types))
    }

    fn update_call_parameters(&mut self, call_parameters: CallParameters) {
        {
            // a separate code block to unlock the mutex ASAP and to avoid double locking
            let mut cp = self.call_parameters_v0.lock();
            *cp = call_parameters_v3_to_v0(call_parameters.clone());
        }

        {
            // a separate code block to unlock the mutex ASAP and to avoid double locking
            let mut cp = self.call_parameters_v1.lock();
            *cp = call_parameters_v3_to_v1(call_parameters.clone());
        }

        {
            // a separate code block to unlock the mutex ASAP and to avoid double locking
            let mut cp = self.call_parameters_v2.lock();
            *cp = call_parameters_v3_to_v2(call_parameters.clone());
        }

        {
            // a separate code block to unlock the mutex ASAP and to avoid double locking
            let mut cp = self.call_parameters_v3.lock();
            *cp = call_parameters;
        }
    }
}

// This API is intended for testing purposes (mostly in Marine REPL)
#[cfg(feature = "raw-module-api")]
impl<WB: WasmBackend> Marine<WB> {
    pub async fn load_module<C, S>(
        &mut self,
        name: S,
        wasm_bytes: &[u8],
        config: Option<C>,
    ) -> MarineResult<()>
    where
        S: Into<String>,
        C: TryInto<crate::generic::MarineModuleConfig<WB>>,
        MarineError: From<C::Error>,
    {
        let config = config.map(|c| c.try_into()).transpose()?;
        let name = name.into();

        // LoggerFilter can be initialized with an empty string
        let wasm_log_env = std::env::var(WASM_LOG_ENV_NAME).unwrap_or_default();
        let logger_filter = LoggerFilter::from_env_string(&wasm_log_env);

        let marine_module_config = crate::config::make_marine_config(
            name.clone(),
            config,
            self.call_parameters_v0.clone(),
            self.call_parameters_v1.clone(),
            self.call_parameters_v2.clone(),
            self.call_parameters_v3.clone(),
            &logger_filter,
        )?;
        self.core
            .load_module(name, wasm_bytes, marine_module_config)
            .await
            .map_err(|e| check_for_oom_and_convert_error(&self.core, e))
    }

    pub fn unload_module(&mut self, module_name: impl AsRef<str>) -> MarineResult<()> {
        self.core.unload_module(module_name).map_err(Into::into)
    }

    pub fn module_wasi_state(
        &mut self,
        module_name: impl AsRef<str>,
    ) -> MarineResult<Box<dyn WasiState + '_>> {
        let module_name = module_name.as_ref();

        self.core
            .module_wasi_state(module_name)
            .ok_or_else(|| MarineError::NoSuchModule(module_name.to_string()))
    }
}

fn check_for_oom_and_convert_error<WB: WasmBackend>(
    core: &MarineCore<WB>,
    error: MError,
) -> MarineError {
    let allocation_stats = match core.module_memory_stats().allocation_stats {
        Some(allocation_stats) => allocation_stats,
        _ => return error.into(),
    };

    if allocation_stats.allocation_rejects == 0 {
        return error.into();
    }

    match error {
        MError::ITInstructionError(_)
        | MError::HostImportError(_)
        | MError::WasmBackendError(_) => MarineError::HighProbabilityOOM {
            allocation_stats,
            original_error: error,
        },
        _ => error.into(),
    }
}
