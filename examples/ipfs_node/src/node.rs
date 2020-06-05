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

use fce::FCE;
use fce::WasmProcess;
use fce::NodeFunction;
use fce::IValue;
use fce::FCEModuleConfig;

use std::fs;
use std::path::PathBuf;

pub(crate) struct IpfsNode {
    process: FCE,
    // names of core modules that is loaded to FCE
    module_names: Vec<String>,
}

pub struct NodeModule<'a> {
    pub name: &'a str,
    pub functions: Vec<NodeFunction<'a>>,
}

impl IpfsNode {
    pub fn new(core_modules_dir: PathBuf, _config_file: PathBuf) -> Result<Self, NodeError> {
        let mut wasm_process = FCE::new();
        let mut module_names = Vec::new();
        let core_module_config = FCEModuleConfig::default();

        for entry in fs::read_dir(core_modules_dir)? {
            let path = entry?.path();
            if !path.is_dir() {
                let module_name = path.file_name().unwrap();
                let module_name = module_name.to_os_string().into_string().unwrap();
                //.ok_or_else(|| Err(NodeError::IOError()))?;

                println!("module name is {}", module_name);
                let module_bytes = fs::read(path.clone())?;

                wasm_process.load_module(
                    module_name.clone(),
                    &module_bytes,
                    core_module_config.clone(),
                )?;
                module_names.push(module_name);
            }
        }

        Ok(Self {
            process: wasm_process,
            module_names,
        })
    }

    pub fn rpc_call(&mut self, wasm_rpc: &[u8]) -> Result<Vec<IValue>, NodeError> {
        let core_module_config = FCEModuleConfig::default();
        let rpc_module_name = "ipfs_rpc";

        self.process
            .load_module(rpc_module_name, wasm_rpc, core_module_config)?;
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
}
