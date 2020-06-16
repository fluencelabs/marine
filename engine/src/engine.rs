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

use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// The base struct of the Fluence Compute Engine.
pub struct FCE {
    // set of modules registered inside FCE
    modules: HashMap<String, FCEModule>,
}

/// Represent a function type inside FCE.
#[derive(Debug)]
pub struct FCEFunction<'a> {
    pub name: &'a str,
    pub inputs: &'a Vec<IType>,
    pub outputs: &'a Vec<IType>,
}

impl FCE {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    /// Invoke a function of a module inside FCE by given function name with given arguments.
    pub fn call(
        &mut self,
        module_name: &str,
        func_name: &str,
        argument: &[IValue],
    ) -> Result<Vec<IValue>, FCEError> {
        match self.modules.get_mut(module_name) {
            // TODO: refactor errors
            Some(module) => module.call(func_name, argument),
            None => Err(FCEError::NoSuchModule),
        }
    }

    /// Load a new module inside FCE.
    pub fn load_module<S>(
        &mut self,
        module_name: S,
        wasm_bytes: &[u8],
        config: FCEModuleConfig,
    ) -> Result<(), FCEError>
    where
        S: Into<String>,
    {
        let _prepared_wasm_bytes = crate::misc::prepare_module(wasm_bytes, config.mem_pages_count)?;

        let module = FCEModule::new(&wasm_bytes, config, &self.modules)?;

        match self.modules.entry(module_name.into()) {
            Entry::Vacant(entry) => {
                entry.insert(module);
                Ok(())
            }
            Entry::Occupied(_) => Err(FCEError::NonUniqueModuleName),
        }
    }

    /// Unload previously loaded module.
    pub fn unload_module(&mut self, module_name: &str) -> Result<(), FCEError> {
        match self.modules.entry(module_name.to_string()) {
            Entry::Vacant(_) => Err(FCEError::NoSuchModule),

            Entry::Occupied(module) => {
                module.remove_entry();
                Ok(())
            }
        }
    }

    /// Return signatures of all exported by this module functions.
    pub fn get_interface(&self, module_name: &str) -> Result<Vec<FCEFunction<'_>>, FCEError> {
        match self.modules.get(module_name) {
            Some(module) => {
                let signatures = module
                    .get_exports_signatures()
                    .map(|(name, inputs, outputs)| FCEFunction {
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

impl Default for FCE {
    fn default() -> Self {
        Self::new()
    }
}
