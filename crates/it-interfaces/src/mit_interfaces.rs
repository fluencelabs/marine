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

use super::errors::MITInterfacesError;

use wasmer_it::interpreter::Instruction;
use wasmer_it::ast::*;
use wasmer_it::IRecordType;
use multimap::MultiMap;

use std::iter::Iterator;
use std::collections::HashMap;
use std::sync::Arc;

pub type CoreFunctionType = u32;
pub type AdapterFunctionType = u32;
pub type ExportName<'a> = &'a str;
pub type ImportName<'a> = &'a str;
pub type ImportNamespace<'a> = &'a str;
pub type ITAstType = Type;

#[derive(Debug)]
pub struct MITInterfaces<'a> {
    /// All the types.
    types: Vec<ITAstType>,

    /// All the imported functions.
    imports: Vec<Import<'a>>,
    core_type_to_imports: MultiMap<CoreFunctionType, (ImportName<'a>, ImportNamespace<'a>)>,

    /// All the adapters.
    adapters: HashMap<AdapterFunctionType, Vec<Instruction>>,

    /// All the exported functions.
    exports: Vec<Export<'a>>,
    core_type_to_exports: MultiMap<CoreFunctionType, ExportName<'a>>,

    /// All the implementations.
    adapter_type_to_core: MultiMap<AdapterFunctionType, CoreFunctionType>,
    core_type_to_adapter: MultiMap<CoreFunctionType, AdapterFunctionType>,
}

impl<'a> MITInterfaces<'a> {
    pub fn new(interfaces: Interfaces<'a>) -> Self {
        let core_type_to_imports = interfaces
            .imports
            .iter()
            .map(|import| (import.function_type, (import.namespace, import.name)))
            .collect::<MultiMap<_, _>>();

        let adapters = interfaces
            .adapters
            .into_iter()
            .map(|adapter| (adapter.function_type, adapter.instructions))
            .collect::<HashMap<_, _>>();

        let core_type_to_exports = interfaces
            .exports
            .iter()
            .map(|export| (export.function_type, export.name))
            .collect::<MultiMap<_, _>>();

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
            imports: interfaces.imports,
            core_type_to_imports,
            adapters,
            exports: interfaces.exports,
            core_type_to_exports,
            adapter_type_to_core,
            core_type_to_adapter,
        }
    }

    pub fn types(&self) -> impl Iterator<Item = &Type> {
        self.types.iter()
    }

    pub fn record_types(&self) -> impl Iterator<Item = (u64, &Arc<IRecordType>)> {
        self.types.iter().enumerate().filter_map(|(id, t)| match t {
            ITAstType::Record(r) => Some((id as u64, r)),
            _ => None,
        })
    }

    pub fn type_by_idx(&self, idx: u32) -> Option<&Type> {
        self.types.get(idx as usize)
    }

    pub fn type_by_idx_r(&self, idx: u32) -> Result<&Type, MITInterfacesError> {
        self.types
            .get(idx as usize)
            .ok_or(MITInterfacesError::NoSuchType(idx))
    }

    pub fn imports(&self) -> impl Iterator<Item = &Import<'_>> {
        self.imports.iter()
    }

    pub fn imports_by_type(
        &self,
        import_type: CoreFunctionType,
    ) -> Option<&Vec<(ImportName<'a>, ImportNamespace<'a>)>> {
        self.core_type_to_imports.get_vec(&import_type)
    }

    pub fn imports_by_type_r(
        &self,
        import_type: CoreFunctionType,
    ) -> Result<&Vec<(ImportName<'a>, ImportNamespace<'a>)>, MITInterfacesError> {
        self.core_type_to_imports
            .get_vec(&import_type)
            .ok_or(MITInterfacesError::NoSuchImport(import_type))
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
    ) -> Result<&Vec<Instruction>, MITInterfacesError> {
        self.adapters
            .get(&adapter_type)
            .ok_or(MITInterfacesError::NoSuchAdapter(adapter_type))
    }

    pub fn exports(&self) -> impl Iterator<Item = &Export<'_>> {
        self.exports.iter()
    }

    pub fn exports_by_type(&self, export_type: u32) -> Option<&Vec<ExportName<'a>>> {
        self.core_type_to_exports.get_vec(&export_type)
    }

    pub fn exports_by_type_r(
        &self,
        export_type: CoreFunctionType,
    ) -> Result<&Vec<ExportName<'a>>, MITInterfacesError> {
        self.core_type_to_exports
            .get_vec(&export_type)
            .ok_or(MITInterfacesError::NoSuchImport(export_type))
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
        self.core_type_to_adapter.get_vec(&core_function_type)
    }

    pub fn core_types_by_adapter_type(
        &self,
        adapter_function_type: AdapterFunctionType,
    ) -> Option<&Vec<CoreFunctionType>> {
        self.adapter_type_to_core.get_vec(&adapter_function_type)
    }
}
