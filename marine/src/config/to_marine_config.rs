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

use marine_core::HostImportDescriptor;
use marine_core::MModuleConfig;
use marine_rs_sdk::CallParameters;
use marine_utils::bytes_to_wasm_pages_ceil;
use wasmer_core::import::ImportObject;
use wasmer_core::import::Namespace;
use wasmer_runtime::func;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

const WASM_MAX_HEAP_SIZE: u64 = 4 * 1024 * 1024 * 1024 - 1; // 4 GiB - 1

struct MModuleConfigBuilder {
    config: MModuleConfig,
}

impl MModuleConfigBuilder {
    pub(self) fn new() -> Self {
        Self {
            config: <_>::default(),
        }
    }

    pub(self) fn build(
        self,
        module_name: String,
        marine_module_config: Option<MarineModuleConfig>,
        call_parameters: Rc<RefCell<CallParameters>>,
        logger_filter: &LoggerFilter<'_>,
    ) -> MarineResult<MModuleConfig> {
        let marine_module_config = match marine_module_config {
            Some(config) => config,
            None => return Ok(self.into_config()),
        };

        let MarineModuleConfig {
            mem_pages_count,
            max_heap_size,
            logger_enabled,
            host_imports,
            wasi,
            logging_mask,
        } = marine_module_config;

        let config = self
            .populate_max_heap_size(mem_pages_count, max_heap_size)?
            .populate_logger(logger_enabled, logging_mask, logger_filter, module_name)
            .populate_host_imports(host_imports, call_parameters)
            .populate_wasi(wasi)
            .add_version()
            .into_config();

        Ok(config)
    }

    fn populate_wasi(mut self, wasi: Option<MarineWASIConfig>) -> Self {
        let wasi = match wasi {
            Some(wasi) => wasi,
            None => return self,
        };

        self.config.wasi_envs = wasi.envs;
        self.config.wasi_preopened_files = wasi.preopened_files;
        self.config.wasi_mapped_dirs = wasi.mapped_dirs;

        // create environment variables for all mapped directories
        let mapped_dirs = self
            .config
            .wasi_mapped_dirs
            .iter()
            .map(|(from, to)| {
                (
                    from.as_bytes().to_vec(),
                    to.to_string_lossy().as_bytes().to_vec(),
                )
            })
            .collect::<HashMap<_, _>>();

        self.config.wasi_envs.extend(mapped_dirs);

        self
    }

    fn populate_host_imports(
        mut self,
        host_imports: HashMap<String, HostImportDescriptor>,
        call_parameters: Rc<RefCell<CallParameters>>,
    ) -> Self {
        self.config.host_imports = host_imports;
        self.config.host_imports.insert(
            String::from("get_call_parameters"),
            create_call_parameters_import(call_parameters),
        );

        self
    }

    fn populate_max_heap_size(
        mut self,
        mem_pages_count: Option<u32>,
        max_heap_size: Option<u64>,
    ) -> MarineResult<Self> {
        let max_heap_pages_count = match (mem_pages_count, max_heap_size) {
            (Some(v), None) => v,
            (_, Some(max_heap_size_wanted)) => {
                if max_heap_size_wanted > WASM_MAX_HEAP_SIZE {
                    return Err(MarineError::MaxHeapSizeOverflow {
                        max_heap_size_wanted,
                        max_heap_size_allowed: WASM_MAX_HEAP_SIZE,
                    });
                };
                bytes_to_wasm_pages_ceil(max_heap_size_wanted as u32)
            }
            // leave the default value
            (None, None) => return Ok(self),
        };

        self.config.max_heap_pages_count = max_heap_pages_count;

        Ok(self)
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
            self.config.wasi_envs.insert(
                WASM_LOG_ENV_NAME.as_bytes().to_owned(),
                log_level_str.into_bytes(),
            );
        }

        let logging_mask = logging_mask;
        let mut namespace = Namespace::new();
        namespace.insert(
            "log_utf8_string",
            func!(log_utf8_string_closure(logging_mask, module_name)),
        );

        let mut raw_host_imports = ImportObject::new();
        raw_host_imports.register("host", namespace);
        self.config.raw_imports = raw_host_imports;

        self
    }

    fn add_version(mut self) -> Self {
        self.config.wasi_version = wasmer_wasi::WasiVersion::Latest;
        self
    }

    fn into_config(self) -> MModuleConfig {
        self.config
    }
}

/// Make Marine config from provided Marine config.
pub(crate) fn make_marine_config(
    module_name: String,
    marine_module_config: Option<MarineModuleConfig>,
    call_parameters: Rc<RefCell<marine_rs_sdk::CallParameters>>,
    logger_filter: &LoggerFilter<'_>,
) -> MarineResult<MModuleConfig> {
    MModuleConfigBuilder::new().build(
        module_name,
        marine_module_config,
        call_parameters,
        logger_filter,
    )
}
