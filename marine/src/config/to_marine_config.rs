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

use crate::MarineWASIConfig;
use crate::MarineResult;
use crate::MarineError;
use crate::config::MarineModuleConfig;
use crate::host_imports::logger::log_utf8_string_closure;
use crate::host_imports::logger::LoggerFilter;
use crate::host_imports::logger::WASM_LOG_ENV_NAME;
use crate::host_imports::create_call_parameters_import;

use marine_core::generic::HostImportDescriptor;
use marine_core::generic::MModuleConfig;
use marine_core::HostAPIVersion;
use marine_wasm_backend_traits::HostFunction;
use marine_wasm_backend_traits::WasmBackend;

use marine_rs_sdk::CallParameters;

use parking_lot::Mutex;
use serde::Serialize;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Arc;

struct MModuleConfigBuilder<WB: WasmBackend> {
    config: MModuleConfig<WB>,
}

impl<WB: WasmBackend> MModuleConfigBuilder<WB> {
    pub(self) fn new() -> Self {
        Self {
            config: <_>::default(),
        }
    }

    pub(self) fn build(
        self,
        module_name: String,
        marine_module_config: Option<MarineModuleConfig<WB>>,
        call_parameters_v0: Arc<Mutex<marine_call_parameters_v0::CallParameters>>,
        call_parameters_v1: Arc<Mutex<marine_call_parameters_v1::CallParameters>>,
        call_parameters_v2: Arc<Mutex<marine_call_parameters_v2::CallParameters>>,
        call_parameters_v3: Arc<Mutex<CallParameters>>,
        logger_filter: &LoggerFilter<'_>,
    ) -> MarineResult<MModuleConfig<WB>> {
        let marine_module_config = match marine_module_config {
            Some(config) => config,
            None => return Ok(self.into_config()),
        };

        let MarineModuleConfig {
            logger_enabled,
            host_imports,
            wasi,
            logging_mask,
        } = marine_module_config;

        let config = self
            .populate_logger(logger_enabled, logging_mask, logger_filter, module_name)
            .populate_host_imports(
                host_imports,
                call_parameters_v0,
                call_parameters_v1,
                call_parameters_v2,
                call_parameters_v3,
            )
            .populate_wasi(wasi)?
            .into_config();

        Ok(config)
    }

    fn populate_wasi(mut self, wasi: Option<MarineWASIConfig>) -> MarineResult<Self> {
        let wasi = match wasi {
            Some(wasi) => wasi,
            None => return Ok(self),
        };

        self.config.wasi_parameters.envs = wasi.envs;

        self.config.wasi_parameters.mapped_dirs = wasi.mapped_dirs;

        // Preopened files and mapped dirs are treated in the same way by the wasm backends.
        // The only difference is that for preopened files the alias and the value are the same,
        // while for mapped dirs user defines the alias. To avoid having same alias pointing to different files,
        // preopened files are moved directly to the mapped dirs.
        for path in wasi.preopened_files {
            let alias = path.to_string_lossy();
            match self.config.wasi_parameters.mapped_dirs.entry(alias.to_string()) {
                Entry::Occupied(entry) => {
                    return Err(MarineError::InvalidConfig(format!(
                        "WASI preopened files conflict with WASI mapped dirs: preopen {} is also mapped to: {}. Remove one of the entries to fix this error.", entry.key(), entry.get().display())
                    ))
                },

                Entry::Vacant(entry) => {
                    entry.insert(path);
                }
            }
        }

        // create environment variables for all mapped directories
        let mapped_dirs = self
            .config
            .wasi_parameters
            .mapped_dirs
            .iter()
            .map(|(from, to)| (from.clone(), to.to_string_lossy().to_string()))
            .collect::<HashMap<_, _>>();

        self.config.wasi_parameters.envs.extend(mapped_dirs);

        Ok(self)
    }

    fn populate_host_imports(
        mut self,
        host_imports: HashMap<HostAPIVersion, HashMap<String, HostImportDescriptor<WB>>>,
        call_parameters_v0: Arc<Mutex<marine_call_parameters_v0::CallParameters>>,
        call_parameters_v1: Arc<Mutex<marine_call_parameters_v1::CallParameters>>,
        call_parameters_v2: Arc<Mutex<marine_call_parameters_v2::CallParameters>>,
        call_parameters_v3: Arc<Mutex<CallParameters>>,
    ) -> Self {
        self.config.host_imports = host_imports;
        self.add_call_parameters_import(HostAPIVersion::V0, call_parameters_v0)
            .add_call_parameters_import(HostAPIVersion::V1, call_parameters_v1)
            .add_call_parameters_import(HostAPIVersion::V2, call_parameters_v2)
            .add_call_parameters_import(HostAPIVersion::V3, call_parameters_v3)
    }

    fn add_call_parameters_import<CP: Serialize + Send + 'static>(
        mut self,
        api_version: HostAPIVersion,
        call_parameters: Arc<Mutex<CP>>,
    ) -> Self {
        self.config
            .host_imports
            .entry(api_version)
            .or_default()
            .insert(
                String::from("get_call_parameters"),
                create_call_parameters_import(call_parameters),
            );
        self
    }

    fn populate_logger(
        mut self,
        logger_enabled: bool,
        logging_mask: i32,
        logger_filter: &LoggerFilter<'_>,
        module_name: String,
    ) -> Self {
        if !logger_enabled {
            return self;
        }

        if let Some(level_filter) = logger_filter.module_level(&module_name) {
            let log_level = level_filter.to_level();
            let log_level_str = match log_level {
                Some(log_level) => log_level.to_string(),
                None => String::from("off"),
            };

            // overwrite possibly installed log variable in config
            self.config
                .wasi_parameters
                .envs
                .insert(WASM_LOG_ENV_NAME.to_string(), log_level_str);
        }

        let creator = Arc::new(move |mut store: <WB as WasmBackend>::ContextMut<'_>| {
            <WB as WasmBackend>::HostFunction::new_typed(
                &mut store,
                log_utf8_string_closure::<WB>(logging_mask, module_name.clone()),
            )
        });

        use HostAPIVersion::*;
        for api_version in [V0, V1, V2, V3] {
            self.config
                .raw_imports
                .entry(api_version)
                .or_default()
                .insert("log_utf8_string".to_string(), creator.clone());
        }

        self
    }

    fn into_config(self) -> MModuleConfig<WB> {
        self.config
    }
}

/// Make Marine config from provided Marine config.
pub(crate) fn make_marine_config<WB: WasmBackend>(
    module_name: String,
    marine_module_config: Option<MarineModuleConfig<WB>>,
    call_parameters_v0: Arc<Mutex<marine_call_parameters_v0::CallParameters>>,
    call_parameters_v1: Arc<Mutex<marine_call_parameters_v1::CallParameters>>,
    call_parameters_v2: Arc<Mutex<marine_call_parameters_v2::CallParameters>>,
    call_parameters_v3: Arc<Mutex<marine_rs_sdk::CallParameters>>,
    logger_filter: &LoggerFilter<'_>,
) -> MarineResult<MModuleConfig<WB>> {
    MModuleConfigBuilder::new().build(
        module_name,
        marine_module_config,
        call_parameters_v0,
        call_parameters_v1,
        call_parameters_v2,
        call_parameters_v3,
        logger_filter,
    )
}
