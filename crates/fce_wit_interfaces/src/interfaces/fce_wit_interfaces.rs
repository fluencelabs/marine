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

use super::errors::FCEWITInterfacesError;

use wasmer_wit::interpreter::Instruction;
use wasmer_wit::ast::*;
use multimap::MultiMap;

use std::iter::Iterator;
use std::collections::HashMap;

pub type CoreFunctionType = u32;
pub type AdapterFunctionType = u32;
pub type ExportName<'a> = &'a str;
pub type ImportName<'a> = &'a str;
pub type ImportNamespace<'a> = &'a str;
pub type WITAstType = Type;

pub struct FCEWITInterfaces<'a> {
    /// All the types.
    types: Vec<WITAstType>,

    /// All the imported functions.
    imports: HashMap<CoreFunctionType, (ImportName<'a>, ImportNamespace<'a>)>,

    /// All the adapters.
    adapters: HashMap<AdapterFunctionType, Vec<Instruction>>,

    /// All the exported functions.
    exports: HashMap<CoreFunctionType, ExportName<'a>>,

    /// All the implementations.
    adapter_type_to_core: MultiMap<AdapterFunctionType, CoreFunctionType>,
    core_type_to_adapter: MultiMap<CoreFunctionType, AdapterFunctionType>,
}

impl<'a> FCEWITInterfaces<'a> {
    pub fn new(interfaces: Interfaces<'a>) -> Self {
        let imports = interfaces
            .imports
            .into_iter()
            .map(|import| (import.function_type, (import.namespace, import.name)))
            .collect::<HashMap<_, _>>();

        let adapters = interfaces
            .adapters
            .into_iter()
            .map(|adapter| (adapter.function_type, adapter.instructions))
            .collect::<HashMap<_, _>>();

        let exports = interfaces
            .exports
            .into_iter()
            .map(|export| (export.function_type, export.name))
            .collect::<HashMap<_, _>>();

        let adapter_type_to_core = interfaces
            .implementations
            .iter()
            .map(|implementation| {
                (
                    implementation.adapter_function_type,
                    implementation.core_function_type,
                )
            })
            .collect::<MultiMap<_, _>>();

        let core_type_to_adapter = interfaces
            .implementations
            .iter()
            .map(|implementation| {
                (
                    implementation.core_function_type,
                    implementation.adapter_function_type,
                )
            })
            .collect::<MultiMap<_, _>>();

        Self {
            types: interfaces.types,
            imports,
            adapters,
            exports,
            adapter_type_to_core,
            core_type_to_adapter,
        }
    }

    pub fn types(&self) -> impl Iterator<Item = &Type> {
        self.types.iter()
    }

    pub fn type_by_idx(&self, idx: u32) -> Option<&Type> {
        self.types.get(idx as usize)
    }

    pub fn type_by_idx_r(&self, idx: u32) -> Result<&Type, FCEWITInterfacesError> {
        self.types
            .get(idx as usize)
            .ok_or_else(|| FCEWITInterfacesError::NoSuchType(idx))
    }

    pub fn imports(
        &self,
    ) -> impl Iterator<Item = (&CoreFunctionType, &(ImportName<'a>, ImportNamespace<'a>))> {
        self.imports.iter()
    }

    pub fn import_by_type(
        &self,
        import_type: CoreFunctionType,
    ) -> Option<&(ImportName<'a>, ImportNamespace<'a>)> {
        self.imports.get(&import_type)
    }

    pub fn import_by_type_r(
        &self,
        import_type: CoreFunctionType,
    ) -> Result<&(ImportName<'a>, ImportNamespace<'a>), FCEWITInterfacesError> {
        self.imports
            .get(&import_type)
            .ok_or_else(|| FCEWITInterfacesError::NoSuchImport(import_type))
    }

    pub fn adapters(&self) -> impl Iterator<Item = (&AdapterFunctionType, &Vec<Instruction>)> {
        self.adapters.iter()
    }

    pub fn adapter_by_type(&self, adapter_type: AdapterFunctionType) -> Option<&Vec<Instruction>> {
        self.adapters.get(&adapter_type)
    }

    pub fn adapter_by_type_r(
        &self,
        adapter_type: AdapterFunctionType,
    ) -> Result<&Vec<Instruction>, FCEWITInterfacesError> {
        self.adapters
            .get(&adapter_type)
            .ok_or_else(|| FCEWITInterfacesError::NoSuchAdapter(adapter_type))
    }

    pub fn export_by_type(&self, export_type: u32) -> Option<&ExportName<'a>> {
        self.exports.get(&export_type)
    }

    pub fn exports(&self) -> impl Iterator<Item = (&CoreFunctionType, &ExportName<'a>)> {
        self.exports.iter()
    }

    pub fn implementations(
        &self,
    ) -> impl Iterator<Item = (&AdapterFunctionType, &CoreFunctionType)> {
        self.adapter_type_to_core.iter()
    }

    pub fn adapter_types_by_core_type(
        &self,
        core_function_type: CoreFunctionType,
    ) -> Option<&Vec<AdapterFunctionType>> {
        self.adapter_type_to_core.get_vec(&core_function_type)
    }

    pub fn core_types_by_adapter_type(
        &self,
        adapter_function_type: AdapterFunctionType,
    ) -> Option<&Vec<CoreFunctionType>> {
        self.core_type_to_adapter.get_vec(&adapter_function_type)
    }
}
