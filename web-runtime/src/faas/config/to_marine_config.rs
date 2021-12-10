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

use crate::faas::Result;
use crate::faas::config::FaaSModuleConfig;
//use crate::faas::host_imports::logger::log_utf8_string_closure;
//use crate::faas::host_imports::logger::LoggerFilter;
//use crate::faas::host_imports::logger::WASM_LOG_ENV_NAME;
//use crate::faas::host_imports::create_call_parameters_import;

use crate::config::MModuleConfig;
//use wasmer_core::import::ImportObject;
//use wasmer_core::import::Namespace;
//use wasmer_runtime::func;

//use std::collections::HashMap;
//use std::cell::RefCell;
//use std::rc::Rc;

/// Make Marine config from provided FaaS config.
pub(crate) fn make_marine_config(
    //module_name: String,
    faas_module_config: Option<FaaSModuleConfig>,
   // call_parameters: Rc<RefCell<marine_rs_sdk::CallParameters>>,
    //logger_filter: &LoggerFilter<'_>,
) -> Result<MModuleConfig> {
    let mut marine_module_cfg = MModuleConfig::default();

    let faas_module_config = match faas_module_config {
        Some(faas_module_config) => faas_module_config,
        None => return Ok(marine_module_cfg),
    };

    if let Some(mem_pages_count) = faas_module_config.mem_pages_count {
        marine_module_cfg.mem_pages_count = mem_pages_count;
    }
/*
    if let Some(wasi) = faas_module_config.wasi {
        marine_module_cfg.wasi_envs = wasi.envs;
        marine_module_cfg.wasi_preopened_files = wasi.preopened_files;
        marine_module_cfg.wasi_mapped_dirs = wasi.mapped_dirs;

        // create environment variables for all mapped directories
        let mapped_dirs = marine_module_cfg
            .wasi_mapped_dirs
            .iter()
            .map(|(from, to)| {
                (
                    from.as_bytes().to_vec(),
                    to.to_string_lossy().as_bytes().to_vec(),
                )
            })
            .collect::<HashMap<_, _>>();

        marine_module_cfg.wasi_envs.extend(mapped_dirs);
    };

    marine_module_cfg.host_imports = faas_module_config.host_imports;
    marine_module_cfg.host_imports.insert(
        String::from("get_call_parameters"),
        create_call_parameters_import(call_parameters),
    );


    let mut namespace = Namespace::new();
    if faas_module_config.logger_enabled {
        if let Some(level_filter) = logger_filter.module_level(&module_name) {
            let log_level = level_filter.to_level();
            let log_level_str = match log_level {
                Some(log_level) => log_level.to_string(),
                None => String::from("off"),
            };

            // overwrite possibly installed log variable in config
            marine_module_cfg.wasi_envs.insert(
                WASM_LOG_ENV_NAME.as_bytes().to_owned(),
                log_level_str.into_bytes(),
            );
        }

        let logging_mask = faas_module_config.logging_mask;
        namespace.insert(
            "log_utf8_string",
            func!(log_utf8_string_closure(logging_mask, module_name)),
        );
    }

    //let mut raw_host_imports = ImportObject::new();
    //raw_host_imports.register("host", namespace);
    marine_module_cfg.raw_imports = raw_host_imports;

    marine_module_cfg.wasi_version = wasmer_wasi::WasiVersion::Latest;
*/
    Ok(marine_module_cfg)
}
