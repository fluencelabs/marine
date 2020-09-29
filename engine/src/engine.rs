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
#[derive(Debug)]
pub struct FCEFunctionSignature<'a> {
    pub name: &'a str,
    pub arguments: &'a Vec<IFunctionArg>,
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
    ) -> Result<Vec<IValue>> {
        self.call_(module_name.as_ref(), func_name.as_ref(), argument)
    }

    fn call_(
        &mut self,
        module_name: &str,
        func_name: &str,
        argument: &[IValue],
    ) -> Result<Vec<IValue>> {
        match self.modules.get_mut(module_name) {
            // TODO: refactor errors
            Some(module) => module.call(func_name.as_ref(), argument),
            None => Err(FCEError::NoSuchModule(format!(
                "trying to call module with name {} that is not loaded",
                module_name
            ))),
        }
    }

    /// Load a new module inside FCE.
    pub fn load_module<S: Into<String>>(
        &mut self,
        name: S,
        wasm_bytes: &[u8],
        config: FCEModuleConfig,
    ) -> Result<()> {
        self.load_module_(name.into(), wasm_bytes, config)
    }

    fn load_module_(
        &mut self,
        name: String,
        wasm_bytes: &[u8],
        config: FCEModuleConfig,
    ) -> Result<()> {
        let _prepared_wasm_bytes = crate::misc::prepare_module(wasm_bytes, config.mem_pages_count)?;

        let module = FCEModule::new(&wasm_bytes, config, &self.modules)?;

        match self.modules.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(module);
                Ok(())
            }
            Entry::Occupied(_) => Err(FCEError::NonUniqueModuleName),
        }
    }

    /// Unload previously loaded module.
    pub fn unload_module<S: AsRef<str>>(&mut self, name: S) -> Result<()> {
        self.unload_module_(name.as_ref())
    }

    fn unload_module_(&mut self, module_name: &str) -> Result<()> {
        match self.modules.remove(module_name) {
            Some(_) => Ok(()),
            None => Err(FCEError::NoSuchModule(format!(
                "trying to unload module with name {} that is not loaded",
                module_name
            ))),
        }
    }

    pub fn module_wasi_state<S: AsRef<str>>(
        &mut self,
        module_name: S,
    ) -> Result<&wasmer_wasi::state::WasiState> {
        self.module_wasi_state_(module_name.as_ref())
    }

    fn module_wasi_state_(&mut self, module_name: &str) -> Result<&wasmer_wasi::state::WasiState> {
        match self.modules.get_mut(module_name) {
            Some(module) => Ok(module.get_wasi_state()),
            None => Err(FCEError::NoSuchModule(format!(
                "trying to get wasi state from module with name {} that is not loaded",
                module_name
            ))),
        }
    }

    /// Return function signatures of all loaded info FCE modules with their names.
    pub fn interface(&self) -> impl Iterator<Item = (&str, Vec<FCEFunctionSignature<'_>>)> {
        self.modules.iter().map(|(module_name, module)| {
            (
                module_name.as_str(),
                Self::get_module_function_signatures(module).collect::<Vec<_>>(),
            )
        })
    }

    /// Return function signatures exported by module with given name.
    pub fn module_interface<S: AsRef<str>>(
        &self,
        module_name: S,
    ) -> Result<impl Iterator<Item = FCEFunctionSignature<'_>>> {
        match self.modules.get(module_name.as_ref()) {
            Some(module) => Ok(Self::get_module_function_signatures(module)),
            None => Err(FCEError::NoSuchModule(format!(
                "trying to get interface from module with name {} that is not loaded",
                module_name.as_ref()
            ))),
        }
    }

    /// Return record types of export functions of all loaded info FCE modules.
    pub fn record_types(&self) -> impl Iterator<Item = &(u64, IRecordType)> {
        self.modules
            .iter()
            .flat_map(|(_, module)| module.get_export_record_types())
    }

    /// Return record types exported by module with given name.
    pub fn module_record_types<S: AsRef<str>>(
        &self,
        module_name: S,
    ) -> Result<impl Iterator<Item = &(u64, IRecordType)>> {
        match self.modules.get(module_name.as_ref()) {
            Some(module) => Ok(module.get_export_record_types()),
            None => Err(FCEError::NoSuchModule(format!(
                "trying to get record types from module with name {} that is not loaded",
                module_name.as_ref()
            ))),
        }
    }

    fn get_module_function_signatures(
        module: &FCEModule,
    ) -> impl Iterator<Item = FCEFunctionSignature<'_>> {
        module
            .get_exports_signatures()
            .map(|(name, arguments, output_types)| FCEFunctionSignature {
                name,
                arguments,
                output_types,
            })
    }
}

impl Default for FCE {
    fn default() -> Self {
        Self::new()
    }
}
