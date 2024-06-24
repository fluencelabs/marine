/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use super::generic::*;
use crate::config::MarineCoreConfig;
use crate::module::MModule;
use crate::module::MRecordTypes;
use crate::{IRecordType, IValue, MemoryStats, MError, MFunctionSignature, ModuleMemoryStat, MResult};

use marine_wasm_backend_traits::AsContextMut;
use marine_wasm_backend_traits::Store;
use marine_wasm_backend_traits::WasiState;
use marine_wasm_backend_traits::WasmBackend;

use serde::Serialize;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;

/// Represent Marine module interface.
#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct MModuleInterface<'a> {
    pub record_types: &'a MRecordTypes,
    pub function_signatures: Vec<MFunctionSignature>,
}

/// # Description
///
/// The base struct of Marine, the Fluence compute runtime.
/// Allows dynamic loading and unloading modules, but never frees resources used for instantiation.
/// A new module can import functions from previously loaded modules.
///
/// # Recommendations
///
/// Its not recommended to use this struct to load/unload unlimited number of modules.
/// Better alternative is to use multiple instances of this struct for independent groups of modules
/// and drop them when the group is no longer needed.
pub struct MarineCore<WB: WasmBackend> {
    // set of modules registered inside Marine
    modules: HashMap<String, MModule<WB>>,
    // Wasm backend may have state in the future
    #[allow(unused)]
    wasm_backend: WB,
    /// Container for all objects created by a Wasm backend.
    store: RefCell<<WB as WasmBackend>::Store>,
}

impl<WB: WasmBackend> MarineCore<WB> {
    pub fn new(config: MarineCoreConfig<WB>) -> MResult<Self> {
        let mut store = <WB as WasmBackend>::Store::new(&config.wasm_backend);
        store.set_total_memory_limit(config.total_memory_limit);
        Ok(Self {
            modules: HashMap::new(),
            wasm_backend: config.wasm_backend,
            store: RefCell::new(store),
        })
    }

    /// Invoke a function of a module inside Marine by given function name with given arguments.
    pub async fn call_async(
        &mut self,
        module_name: impl AsRef<str>,
        func_name: impl AsRef<str>,
        arguments: &[IValue],
    ) -> MResult<Vec<IValue>> {
        let module_name = module_name.as_ref();
        let store = &mut self.store;
        let module = self
            .modules
            .get_mut(module_name)
            .ok_or_else(|| MError::NoSuchModule(module_name.to_string()))?;

        module
            .call_async(
                &mut store.get_mut().as_context_mut(),
                module_name,
                func_name.as_ref(),
                arguments,
            )
            .await
    }

    /// Load a new module inside Marine.
    pub async fn load_module(
        &mut self,
        name: impl Into<String>,
        wasm_bytes: &[u8],
        config: MModuleConfig<WB>,
    ) -> MResult<()> {
        self.load_module_(name.into(), wasm_bytes, config).await
    }

    async fn load_module_(
        &mut self,
        name: String,
        wasm_bytes: &[u8],
        config: MModuleConfig<WB>,
    ) -> MResult<()> {
        let module = MModule::new(
            &name,
            self.store.get_mut(),
            wasm_bytes,
            config,
            &self.modules,
        )
        .await?;

        match self.modules.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(module);
                Ok(())
            }
            Entry::Occupied(entry) => Err(MError::NonUniqueModuleName(entry.key().clone())),
        }
    }

    /// Unload previously loaded module.
    pub fn unload_module(&mut self, name: impl AsRef<str>) -> MResult<()> {
        // TODO: clean up all reference from adaptors after adding support of lazy linking
        self.modules
            .remove(name.as_ref())
            .map(|_| ())
            .ok_or_else(|| MError::NoSuchModule(name.as_ref().to_string()))
    }

    pub fn module_wasi_state<'s>(
        &'s mut self,
        module_name: impl AsRef<str>,
    ) -> Option<Box<dyn WasiState + 's>> {
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
    pub fn module_interface(&self, module_name: impl AsRef<str>) -> Option<MModuleInterface<'_>> {
        self.modules
            .get(module_name.as_ref())
            .map(Self::get_module_interface)
    }

    /// Return record types exported by module with given name.
    pub fn module_record_types(&self, module_name: impl AsRef<str>) -> Option<&MRecordTypes> {
        self.modules
            .get(module_name.as_ref())
            .map(|module| module.export_record_types())
    }

    /// Return record type for supplied record id exported by module with given name.
    pub fn module_record_type_by_id(
        &self,
        module_name: impl AsRef<str>,
        record_id: u64,
    ) -> Option<&Arc<IRecordType>> {
        self.modules
            .get(module_name.as_ref())
            .and_then(|module| module.export_record_type_by_id(record_id))
    }

    /// Returns a heap size that all modules consume in bytes.
    pub fn module_memory_stats(&self) -> MemoryStats<'_> {
        let records = self
            .modules
            .iter()
            .map(|(module_name, module)| {
                ModuleMemoryStat::new(
                    module_name,
                    module.memory_size(&mut self.store.borrow_mut().as_context_mut()),
                )
            })
            .collect::<Vec<_>>();
        let allocation_stats = self.store.borrow_mut().report_memory_allocation_stats();
        MemoryStats::new(records, allocation_stats)
    }

    pub fn clear_allocation_stats(&mut self) {
        self.store.borrow_mut().clear_allocation_stats()
    }

    fn get_module_interface(module: &MModule<WB>) -> MModuleInterface<'_> {
        let record_types = module.export_record_types();

        let function_signatures = module.get_exports_signatures().collect::<Vec<_>>();

        MModuleInterface {
            record_types,
            function_signatures,
        }
    }
}
