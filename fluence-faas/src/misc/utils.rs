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

use super::config::ModuleConfig;
use super::log_utf8_string;

use fce::FCEModuleConfig;
use fce::HostImportDescriptor;
use wasmer_core::import::ImportObject;
use wasmer_core::import::Namespace;
use wasmer_runtime::func;
use wasmer_wit::values::InterfaceValue as IValue;
use wasmer_wit::types::InterfaceType as IType;

use std::collections::HashMap;
use std::path::PathBuf;

/// Make FCE config based on parsed config.
pub(crate) fn make_fce_config(
    module_config: Option<ModuleConfig>,
    call_parameters: std::rc::Rc<std::cell::RefCell<fluence_sdk_main::CallParameters>>,
) -> crate::Result<FCEModuleConfig> {
    let mut wasm_module_config = FCEModuleConfig::default();

    let module_config = match module_config {
        Some(module_config) => module_config,
        None => return Ok(wasm_module_config),
    };

    if let Some(mem_pages_count) = module_config.mem_pages_count {
        wasm_module_config.mem_pages_count = mem_pages_count;
    }

    let mut namespace = Namespace::new();

    if module_config.logger_enabled {
        namespace.insert("log_utf8_string", func!(log_utf8_string));
    }

    if let Some(wasi) = module_config.wasi {
        if let Some(envs) = wasi.envs {
            wasm_module_config.wasi_envs = envs;
        }

        if let Some(preopened_files) = wasi.preopened_files {
            wasm_module_config.wasi_preopened_files = preopened_files
                .iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>();
        }

        if let Some(mapped_dirs) = wasi.mapped_dirs {
            wasm_module_config.wasi_mapped_dirs = mapped_dirs
                .into_iter()
                .map(|(alias, path)| (alias, PathBuf::from(path)))
                .collect::<Vec<_>>();
        }

        let mapped_dirs = wasm_module_config
            .wasi_mapped_dirs
            .iter()
            .map(|(from, to)| (format!("{}={}", from, to.as_path().to_str().unwrap()).into_bytes()))
            .collect::<Vec<_>>();

        wasm_module_config.wasi_envs.extend(mapped_dirs);
    };

    let mut host_imports = HashMap::new();

    if let Some(imports) = module_config.imports {
        for (import_name, host_cmd) in imports {
            let host_cmd_closure = move |args: Vec<IValue>| {
                let arg = match &args[0] {
                    IValue::String(str) => str,
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

            host_imports.insert(
                import_name,
                HostImportDescriptor {
                    closure: Box::new(host_cmd_closure),
                    argument_types: vec![IType::String],
                    output_type: Some(IType::String),
                    error_handler: None,
                },
            );
        }
    }

    let call_parameters_closure = move |_args: Vec<IValue>| {
        let call_parameters_ret = call_parameters.borrow().clone();
        let result = crate::to_interface_value(&call_parameters_ret).unwrap();
        Some(result)
    };

    host_imports.insert(
        String::from("get_call_parameters"),
        HostImportDescriptor {
            closure: Box::new(call_parameters_closure),
            argument_types: vec![],
            output_type: Some(IType::Record(0)),
            error_handler: None,
        },
    );

    let mut import_object = ImportObject::new();
    import_object.register("host", namespace);

    wasm_module_config.imports = host_imports;
    wasm_module_config.raw_imports = import_object;
    wasm_module_config.wasi_version = wasmer_wasi::WasiVersion::Latest;

    Ok(wasm_module_config)
}
