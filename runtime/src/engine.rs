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

use super::*;
use crate::module::MModule;
use crate::module::MRecordTypes;

use serde::Serialize;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

/// Represent Marine module interface.
#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct MModuleInterface<'a> {
    pub record_types: &'a MRecordTypes,
    pub function_signatures: Vec<MFunctionSignature>,
}

/// The base struct of Marine, the Fluence compute runtime.
pub struct Marine {
    // set of modules registered inside Marine
    modules: HashMap<String, MModule>,
}

impl Marine {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    /// Invoke a function of a module inside Marine by given function name with given arguments.
    pub fn call<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        arguments: &[IValue],
    ) -> MResult<Vec<IValue>> {
        self.modules.get_mut(module_name.as_ref()).map_or_else(
            || Err(MError::NoSuchModule(module_name.as_ref().to_string())),
            |module| module.call(module_name.as_ref(), func_name.as_ref(), arguments),
        )
    }

    /// Load a new module inside Marine.
    pub fn load_module<S: Into<String>>(
        &mut self,
        name: S,
        wasm_bytes: &[u8],
        config: MModuleConfig,
    ) -> MResult<()> {
        self.load_module_(name.into(), wasm_bytes, config)
    }

    fn load_module_(
        &mut self,
        name: String,
        wasm_bytes: &[u8],
        config: MModuleConfig,
    ) -> MResult<()> {
        let _prepared_wasm_bytes =
            crate::misc::prepare_module(wasm_bytes, config.max_heap_pages_count)?;
        let module = MModule::new(&name, wasm_bytes, config, &self.modules)?;

        match self.modules.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(module);
                Ok(())
            }
            Entry::Occupied(entry) => Err(MError::NonUniqueModuleName(entry.key().clone())),
        }
    }

    /// Unload previously loaded module.
    pub fn unload_module<S: AsRef<str>>(&mut self, name: S) -> MResult<()> {
        // TODO: clean up all reference from adaptors after adding support of lazy linking
        self.modules
            .remove(name.as_ref())
            .map(|_| ())
            .ok_or_else(|| MError::NoSuchModule(name.as_ref().to_string()))
    }

    pub fn module_wasi_state<S: AsRef<str>>(
        &mut self,
        module_name: S,
    ) -> Option<&wasmer_wasi::state::WasiState> {
        self.modules
            .get_mut(module_name.as_ref())
            .map(|module| module.get_wasi_state())
    }

    /// Return function signatures of all loaded info Marine modules with their names.
    pub fn interface(&self) -> impl Iterator<Item = (&str, MModuleInterface<'_>)> {
        self.modules
            .iter()
            .map(|(module_name, module)| (module_name.as_str(), Self::get_module_interface(module)))
    }

    /// Return function signatures exported by module with given name.
    pub fn module_interface<S: AsRef<str>>(&self, module_name: S) -> Option<MModuleInterface<'_>> {
        self.modules
            .get(module_name.as_ref())
            .map(Self::get_module_interface)
    }

    /// Return record types exported by module with given name.
    pub fn module_record_types<S: AsRef<str>>(&self, module_name: S) -> Option<&MRecordTypes> {
        self.modules
            .get(module_name.as_ref())
            .map(|module| module.export_record_types())
    }

    /// Return record type for supplied record id exported by module with given name.
    pub fn module_record_type_by_id<S: AsRef<str>>(
        &self,
        module_name: S,
        record_id: u64,
    ) -> Option<&Rc<IRecordType>> {
        self.modules
            .get(module_name.as_ref())
            .and_then(|module| module.export_record_type_by_id(record_id))
    }

    /// Returns a heap size that all modules consume in bytes.
    pub fn module_memory_stats(&self) -> MemoryStats<'_> {
        let records = self
            .modules
            .iter()
            .map(|(module_name, module)| (module_name.as_str(), module.memory_size()).into())
            .collect::<Vec<_>>();

        records.into()
    }

    fn get_module_interface(module: &MModule) -> MModuleInterface<'_> {
        let record_types = module.export_record_types();

        let function_signatures = module.get_exports_signatures().collect::<Vec<_>>();

        MModuleInterface {
            record_types,
            function_signatures,
        }
    }
}

impl Default for Marine {
    fn default() -> Self {
        Self::new()
    }
}
