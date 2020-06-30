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

/// Represent a function type inside FCE.
#[derive(Debug, serde::Serialize)]
pub struct FCEFunction<'a> {
    pub name: &'a str,
    pub input_types: &'a Vec<IType>,
    pub output_types: &'a Vec<IType>,
}

/// The base struct of the Fluence Compute Engine.
pub struct FCE {
    // set of modules registered inside FCE
    modules: HashMap<String, FCEModule>,
}

impl FCE {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    /// Invoke a function of a module inside FCE by given function name with given arguments.
    pub fn call<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        argument: &[IValue],
    ) -> Result<Vec<IValue>, FCEError> {
        match self.modules.get_mut(module_name.as_ref()) {
            // TODO: refactor errors
            Some(module) => module.call(func_name.as_ref(), argument),
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
    pub fn unload_module<S: AsRef<str>>(&mut self, module_name: S) -> Result<(), FCEError> {
        match self.modules.remove(module_name.as_ref()) {
            Some(_) => Ok(()),
            None => Err(FCEError::NoSuchModule),
        }
    }

    /// Return function signatures of all loaded info FCE modules with their names.
    pub fn interface(&self) -> impl Iterator<Item = (&str, Vec<FCEFunction<'_>>)> {
        self.modules.iter().map(|(module_name, module)| {
            (
                module_name.as_str(),
                Self::get_module_function_signatures(module),
            )
        })
    }

    /// Return function signatures exported by module with given name.
    pub fn module_interface<S: AsRef<str>>(
        &self,
        module_name: S,
    ) -> Result<Vec<FCEFunction<'_>>, FCEError> {
        match self.modules.get(module_name.as_ref()) {
            Some(module) => Ok(Self::get_module_function_signatures(module)),
            None => Err(FCEError::NoSuchModule),
        }
    }

    fn get_module_function_signatures(module: &FCEModule) -> Vec<FCEFunction<'_>> {
        module
            .get_exports_signatures()
            .map(|(name, inputs, outputs)| FCEFunction {
                name,
                input_types: inputs,
                output_types: outputs,
            })
            .collect::<Vec<_>>()
    }
}

impl Default for FCE {
    fn default() -> Self {
        Self::new()
    }
}
