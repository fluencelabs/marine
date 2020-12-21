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
use crate::config::FaaSModuleConfig;
use crate::errors::FaaSError;
use crate::logger::log_utf8_string_closure;
use crate::logger::LoggerFilter;
use crate::logger::WASM_LOG_ENV_NAME;

use fce::FCEModuleConfig;
use fce::HostImportDescriptor;
use wasmer_core::import::ImportObject;
use wasmer_core::import::Namespace;
use wasmer_core::vm::Ctx;
use wasmer_runtime::func;
use wasmer_wit::values::InterfaceValue as IValue;
use wasmer_wit::types::InterfaceType as IType;

use std::collections::HashMap;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::ops::Deref;

pub(crate) fn create_host_import(host_cmd: String) -> HostImportDescriptor {
    let host_cmd_closure = move |_ctx: &mut Ctx, args: Vec<IValue>| {
        let arg = match &args[0] {
            IValue::String(str) => str,
            // this closure will be linked to import function with signature from supplied
            // HostImportDescriptor. So it should be invoked only with string as an arg.
            _ => unreachable!(),
        };

        let result = match cmd_lib::run_fun!("{} {}", host_cmd, arg) {
            Ok(result) => result,
            Err(e) => {
                log::error!("error occurred `{} {}`: {:?} ", host_cmd, arg, e);
                String::new()
            }
        };

        Some(IValue::String(result))
    };

    HostImportDescriptor {
        host_exported_func: Box::new(host_cmd_closure),
        argument_types: vec![IType::String],
        output_type: Some(IType::String),
        error_handler: None,
    }
}

fn create_call_parameters_import(
    call_parameters: Rc<RefCell<fluence_sdk_main::CallParameters>>,
) -> HostImportDescriptor {
    let call_parameters_closure = move |_ctx: &mut Ctx, _args: Vec<IValue>| {
        let result = crate::to_interface_value(call_parameters.borrow().deref()).unwrap();
        Some(result)
    };

    HostImportDescriptor {
        host_exported_func: Box::new(call_parameters_closure),
        argument_types: vec![],
        output_type: Some(IType::Record(0)),
        error_handler: None,
    }
}

/// Make FCE config from provided FaaS config.
pub(crate) fn make_fce_config(
    module_name: String,
    faas_module_config: Option<FaaSModuleConfig>,
    call_parameters: Rc<RefCell<fluence_sdk_main::CallParameters>>,
    logger_filter: &LoggerFilter<'_>,
) -> Result<FCEModuleConfig> {
    let mut fce_module_config = FCEModuleConfig::default();

    let faas_module_config = match faas_module_config {
        Some(faas_module_config) => faas_module_config,
        None => return Ok(fce_module_config),
    };

    if let Some(mem_pages_count) = faas_module_config.mem_pages_count {
        fce_module_config.mem_pages_count = mem_pages_count;
    }

    if let Some(wasi) = faas_module_config.wasi {
        fce_module_config.wasi_envs = wasi.envs;
        fce_module_config.wasi_preopened_files = wasi.preopened_files;
        fce_module_config.wasi_mapped_dirs = wasi.mapped_dirs;

        // create environment variables for all mapped directories
        let mapped_dirs = fce_module_config
            .wasi_mapped_dirs
            .iter()
            .map(|(from, to)| {
                (
                    from.as_bytes().to_vec(),
                    to.to_string_lossy().as_bytes().to_vec(),
                )
            })
            .collect::<HashMap<_, _>>();

        fce_module_config.wasi_envs.extend(mapped_dirs);
    };

    fce_module_config.host_imports = faas_module_config.host_imports;
    fce_module_config.host_imports.insert(
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
            fce_module_config.wasi_envs.insert(
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

    let mut raw_host_imports = ImportObject::new();
    raw_host_imports.register("host", namespace);
    fce_module_config.raw_imports = raw_host_imports;

    fce_module_config.wasi_version = wasmer_wasi::WasiVersion::Latest;

    Ok(fce_module_config)
}

use crate::misc::ModulesLoadStrategy;

/// Loads modules from a directory at a given path. Non-recursive, ignores subdirectories.
pub(crate) fn load_modules_from_fs(
    modules_dir: &PathBuf,
    modules: ModulesLoadStrategy<'_>,
) -> Result<HashMap<String, Vec<u8>>> {
    use FaaSError::IOError;

    let mut dir_entries =
        std::fs::read_dir(modules_dir).map_err(|e| IOError(format!("{:?}: {}", modules_dir, e)))?;

    let loaded = dir_entries.try_fold(HashMap::new(), |mut hash_map, entry| {
        let entry = entry?;
        let path = entry.path();
        // Skip directories
        if path.is_dir() {
            return Ok(hash_map);
        }

        let module_name = path
            .file_name()
            .ok_or_else(|| IOError(format!("No file name in path {:?}", path)))?
            .to_os_string()
            .into_string()
            .map_err(|name| IOError(format!("invalid file name: {:?}", name)))?;

        if modules.should_load(&module_name.as_ref()) {
            let module_bytes = std::fs::read(path)?;
            let module_name = modules.extract_module_name(module_name);
            if hash_map.insert(module_name, module_bytes).is_some() {
                return Err(FaaSError::ConfigParseError(String::from(
                    "module {} is duplicated in config",
                )));
            }
        }

        Ok(hash_map)
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
