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
use super::node_public_interface::NodePublicInterface;
use super::node_public_interface::NodeModulePublicInterface;

use fce::FCE;
use fce::WasmProcess;
use fce::IValue;
use fce::FCEModuleConfig;

use std::fs;
use std::path::PathBuf;

pub struct IpfsNode {
    process: FCE,
    // names of core modules that is loaded to FCE
    module_names: Vec<String>,
    rpc_module_config: FCEModuleConfig,
}

impl IpfsNode {
    pub fn new<P: Into<PathBuf>>(
        core_modules_dir: P,
        config_file_path: P,
    ) -> Result<Self, NodeError> {
        let mut wasm_process = FCE::new();
        let mut module_names = Vec::new();
        let mut core_modules_config =
            super::config::parse_config_from_file(config_file_path.into())?;

        for entry in fs::read_dir(core_modules_dir.into())? {
            let path = entry?.path();
            if path.is_dir() {
                // just skip directories
                continue;
            }

            let module_name = path.file_name().unwrap();
            let module_name = module_name
                .to_os_string()
                .into_string()
                .map_err(|e| NodeError::IOError(format!("failed to read from {:?} file", e)))?;

            let module_bytes = fs::read(path.clone())?;

            let core_module_config = super::utils::make_wasm_process_config(
                core_modules_config.modules_config.remove(&module_name),
            )?;
            wasm_process.load_module(module_name.clone(), &module_bytes, core_module_config)?;
            module_names.push(module_name);
        }

        let rpc_module_config =
            super::utils::make_wasm_process_config(core_modules_config.rpc_module_config)?;

        Ok(Self {
            process: wasm_process,
            module_names,
            rpc_module_config,
        })
    }
}

impl crate::node_wasm_service::NodeWasmService for IpfsNode {
    fn rpc_call(
        &mut self,
        wasm_rpc: &[u8],
        func_name: &str,
        args: &[IValue],
    ) -> Result<Vec<IValue>, NodeError> {
        let rpc_module_name = "ipfs_rpc";

        self.process
            .load_module(rpc_module_name, wasm_rpc, self.rpc_module_config.clone())?;

        let call_result = self.process.call(rpc_module_name, func_name, args)?;
        self.process.unload_module(rpc_module_name)?;

        Ok(call_result)
    }

    fn core_call(
        &mut self,
        module_name: &str,
        func_name: &str,
        args: &[IValue],
    ) -> Result<Vec<IValue>, NodeError> {
        self.process
            .call(module_name, func_name, args)
            .map_err(Into::into)
    }

    fn get_interface(&self) -> NodePublicInterface {
        let mut modules = Vec::with_capacity(self.module_names.len());

        for module_name in self.module_names.iter() {
            let functions = self.process.get_interface(module_name).unwrap();
            modules.push(NodeModulePublicInterface {
                name: module_name,
                functions,
            })
        }

        NodePublicInterface { modules }
    }
}
