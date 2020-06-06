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

use super::module::FCEModule;
use super::*;
use crate::WasmProcess;
use crate::NodeFunction;

use std::sync::Arc;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct FCE {
    // set of modules registered inside FCE
    modules: HashMap<String, Arc<FCEModule>>,
}

impl FCE {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
}

impl Default for FCE {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmProcess for FCE {
    fn call(
        &mut self,
        module_name: &str,
        func_name: &str,
        argument: &[IValue],
    ) -> Result<Vec<IValue>, FCEError> {
        match self.modules.get_mut(module_name) {
            // TODO: refactor errors
            Some(mut module) => unsafe {
                Ok(Arc::get_mut_unchecked(&mut module).call(func_name, argument)?)
            },
            None => {
                println!("no such module");
                Err(FCEError::NoSuchModule)
            }
        }
    }

    fn load_module<S>(
        &mut self,
        module_name: S,
        wasm_bytes: &[u8],
        config: FCEModuleConfig,
    ) -> Result<(), FCEError>
    where
        S: Into<String>,
    {
        let _prepared_wasm_bytes =
            super::prepare::prepare_module(wasm_bytes, config.mem_pages_count)?;

        let module = FCEModule::new(&wasm_bytes, config.imports, &self.modules)?;

        match self.modules.entry(module_name.into()) {
            Entry::Vacant(entry) => {
                entry.insert(Arc::new(module));
                Ok(())
            }
            Entry::Occupied(_) => Err(FCEError::NonUniqueModuleName),
        }
    }

    fn unload_module(&mut self, module_name: &str) -> Result<(), FCEError> {
        match self.modules.entry(module_name.to_string()) {
            Entry::Vacant(_) => Err(FCEError::NoSuchModule),

            Entry::Occupied(module) => {
                module.remove_entry();
                Ok(())
            }
        }
    }

    fn get_interface(&self, module_name: &str) -> Result<Vec<NodeFunction<'_>>, FCEError> {
        match self.modules.get(module_name) {
            Some(module) => {
                let signatures = module
                    .as_ref()
                    .get_exports_signatures()
                    .map(|(name, inputs, outputs)| NodeFunction {
                        name,
                        inputs,
                        outputs,
                    })
                    .collect::<Vec<_>>();
                Ok(signatures)
            }
            None => Err(FCEError::NoSuchModule),
        }
    }
}
