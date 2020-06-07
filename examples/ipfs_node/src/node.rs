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

use super::errors::NodeError;

use wasmer_core::import::ImportObject;
use wasmer_runtime::func;
use fce::FCE;
use fce::WasmProcess;
use fce::NodeFunction;
use fce::IValue;
use fce::FCEModuleConfig;

use std::fs;
use std::path::PathBuf;
use crate::config::ModuleConfig;

pub struct IpfsNode {
    process: FCE,
    // names of core modules that is loaded to FCE
    module_names: Vec<String>,
    rpc_module_config: FCEModuleConfig,
}

#[derive(Debug)]
pub struct NodeModule<'a> {
    pub name: &'a str,
    pub functions: Vec<NodeFunction<'a>>,
}

impl IpfsNode {
    pub fn new(core_modules_dir: PathBuf, config_file_path: PathBuf) -> Result<Self, NodeError> {
        let mut wasm_process = FCE::new();
        let mut module_names = Vec::new();
        let mut core_modules_config = crate::config::parse_config_from_file(config_file_path)?;

        for entry in fs::read_dir(core_modules_dir)? {
            let path = entry?.path();
            if path.is_dir() {
                continue;
            }

            let module_name = path.file_name().unwrap();
            let module_name = module_name
                .to_os_string()
                .into_string()
                .map_err(|e| NodeError::IOError(format!("failed to read from {:?} file", e)))?;

            let module_bytes = fs::read(path.clone())?;

            let core_module_config =
                Self::make_wasm_config(core_modules_config.modules_config.remove(&module_name))?;
            wasm_process.load_module(module_name.clone(), &module_bytes, core_module_config)?;
            module_names.push(module_name);
        }

        let rpc_module_config = Self::make_wasm_config(core_modules_config.rpc_module_config)?;

        Ok(Self {
            process: wasm_process,
            module_names,
            rpc_module_config,
        })
    }

    pub fn rpc_call(&mut self, wasm_rpc: &[u8]) -> Result<Vec<IValue>, NodeError> {
        let rpc_module_name = "ipfs_rpc";

        self.process
            .load_module(rpc_module_name, wasm_rpc, self.rpc_module_config.clone())?;

        let call_result = self.process.call(
            rpc_module_name,
            "invoke",
            &[IValue::String("test".to_string())],
        )?;
        self.process.unload_module(rpc_module_name)?;

        Ok(call_result)
    }

    pub fn get_interface(&self) -> Vec<NodeModule> {
        let mut modules = Vec::with_capacity(self.module_names.len());

        for module_name in self.module_names.iter() {
            let functions = self.process.get_interface(module_name).unwrap();
            modules.push(NodeModule {
                name: module_name,
                functions,
            })
        }

        modules
    }

    fn make_wasm_config(config: Option<ModuleConfig>) -> Result<FCEModuleConfig, NodeError> {
        use crate::imports::create_host_import_func;
        use crate::imports::log_utf8_string;
        use wasmer_core::import::Namespace;

        let mut wasm_module_config = FCEModuleConfig::default();

        let module_config = match config {
            Some(config) => config,
            None => return Ok(wasm_module_config),
        };

        if let Some(mem_pages_count) = module_config.mem_pages_count {
            wasm_module_config.mem_pages_count = mem_pages_count;
        }

        let mut namespace = Namespace::new();

        if let Some(logger_enabled) = module_config.logger_enabled {
            if logger_enabled {
                namespace.insert("log_utf8_string", func!(log_utf8_string));
            }
        }

        if let Some(imports) = module_config.imports {
            for (import_name, host_cmd) in imports {
                let host_import = create_host_import_func(host_cmd);
                namespace.insert(import_name, host_import);
                //namespace.insert(import_name, func!(crate::imports::ipfs));
            }
        }

        let mut import_object = ImportObject::new();
        import_object.register("host", namespace);

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
                    .map(|(from, to)| (from, PathBuf::from(to)))
                    .collect::<Vec<_>>();
            }
        }

        wasm_module_config.imports = import_object;

        Ok(wasm_module_config)
    }
}
