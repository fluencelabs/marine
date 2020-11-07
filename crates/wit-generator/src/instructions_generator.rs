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

mod fn_instructions;
mod foreign_mod_instructions;
mod record_instructions;
mod utils;

pub(crate) use utils::add_function_type;

use crate::*;
use crate::default_export_api_config::ApiExportFuncDescriptor;

use std::rc::Rc;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Default)]
pub(crate) struct WITResolver<'a> {
    record_types: HashMap<String, usize>,
    unresolved_types_count: usize,
    function_types: HashMap<IFunctionType, u32>,
    interfaces: Interfaces<'a>,
}

impl<'a> WITResolver<'a> {
    pub(crate) fn get_record_type_id(&self, record_name: &str) -> Result<usize> {
        match self.record_types.get(record_name) {
            Some(type_index) => Ok(*type_index),
            None => Err(crate::errors::WITGeneratorError::CorruptedRecord(format!(
                "Can't find record with name='{}', don't you forget to wrap it with #[fce]",
                record_name
            ))),
        }
    }

    // adds a stub for type with such a name if it wasn't found
    pub(crate) fn get_record_type_id_unchecked(&mut self, record_name: &str) -> usize {
        match self.record_types.get(record_name) {
            Some(type_index) => *type_index,
            None => {
                self.record_types
                    .insert(record_name.to_string(), self.interfaces.types.len());
                self.interfaces
                    .types
                    .push(AstType::Record(Rc::new(IRecordType::default())));

                self.unresolved_types_count += 1;
                self.interfaces.types.len()
            }
        }
    }

    pub(crate) fn get_record_type(&self, record_type_id: u64) -> Result<&IRecordType> {
        if record_type_id >= self.interfaces.types.len() as u64 {
            return Err(crate::errors::WITGeneratorError::CorruptedRecord(format!(
                "Can't find record with id {}, don't you forget to wrap it with #[fce]",
                record_type_id
            )));
        }

        match &self.interfaces.types[record_type_id as usize] {
            AstType::Function { .. } => {
                panic!("internal error inside WITResolver: interfaces AST type should be record not record")
            }
            AstType::Record(record_type) => Ok(record_type),
        }
    }

    pub(crate) fn insert_record_type(&mut self, record: IRecordType) {
        match self.record_types.get(&record.name) {
            Some(pos) => {
                self.interfaces.types[*pos] = AstType::Record(Rc::new(record));
                self.unresolved_types_count -= 1;
            }
            None => {
                self.record_types
                    .insert(record.name.clone(), self.interfaces.types.len());

                self.interfaces.types.push(AstType::Record(Rc::new(record)));
            }
        }
    }

    pub(crate) fn validate_records(&self) -> Result<()> {
        use crate::errors::WITGeneratorError::CorruptedRecord;
        const TYPE_RESOLVE_RECURSION_LIMIT: u32 = 1024;

        fn validate_record_type(
            record_type: &IRecordType,
            recursion_level: u32,
            wit_resolver: &WITResolver<'_>,
        ) -> Result<()> {
            if recursion_level >= TYPE_RESOLVE_RECURSION_LIMIT {
                return Err(CorruptedRecord(String::from(
                    "too many inner structures level",
                )));
            }

            for field in record_type.fields.iter() {
                match &field.ty {
                    wasmer_wit::types::InterfaceType::Record(record_type_id) => {
                        let inner_record_type = wit_resolver.get_record_type(*record_type_id)?;
                        validate_record_type(
                            &inner_record_type,
                            recursion_level + 1,
                            wit_resolver,
                        )?;
                    }
                    _ => continue,
                }
            }

            Ok(())
        }

        if self.unresolved_types_count != 0 {
            return Err(CorruptedRecord(format!(
                "{} types unresolved",
                self.unresolved_types_count
            )));
        }

        for ty in self.interfaces.types.iter() {
            let record_type = match ty {
                wasmer_wit::ast::Type::Record(ty) => ty,
                _ => continue,
            };

            validate_record_type(record_type, 0, self)?;
        }

        Ok(())
    }

    pub(crate) fn insert_default_api(&mut self, function_descriptor: &ApiExportFuncDescriptor) {
        let function_type = IFunctionType {
            arguments: function_descriptor.arguments.clone(),
            output_types: function_descriptor.output_types.clone(),
        };
        self.interfaces
            .types
            .push(AstType::Function(Rc::new(function_type)));

        let export = AstExport {
            name: function_descriptor.name,
            function_type: function_descriptor.id,
        };
        self.interfaces.exports.push(export);
    }

    /// Insert a new function type if there is no one already,
    /// return already inserted otherwise.
    pub(crate) fn insert_function_type(&mut self, function_type: IFunctionType) -> u32 {
        use std::collections::hash_map::Entry::*;

        let next_id = (self.interfaces.types.len() + self.function_types.len()) as u32;
        match self.function_types.entry(function_type) {
            Occupied(entry) => *entry.get(),
            Vacant(entry) => {
                entry.insert(next_id);
                next_id
            }
        }
    }

    /// Insert a new adapter, returns its id.
    pub(crate) fn insert_adapter(&mut self, adapter: AstAdapter) -> u32 {
        self.interfaces.adapters.push(adapter);

        (self.interfaces.adapters.len() - 1) as u32
    }

    /// Insert a new implementation.
    pub(crate) fn insert_implementation(&mut self, implementation: AstImplementation) -> u32 {
        self.interfaces.implementations.push(implementation);

        (self.interfaces.implementations.len() - 1) as u32
    }

    /// Insert a new export type and returns its id, that uniquely identifies its
    /// and could be used in a call-core instruction.
    pub(crate) fn insert_export_type(&mut self, export: AstExport<'a>) -> u32 {
        let interfaces = &mut self.interfaces;
        interfaces.exports.push(export);

        (interfaces.exports.len() - 1) as u32
    }

    /// Insert a new import type and returns its id, that uniquely identifies its
    /// and could be used in a call-core instruction.
    pub(crate) fn insert_import_type(&mut self, import: AstImport<'a>) -> u32 {
        let interfaces = &mut self.interfaces;
        interfaces.imports.push(import);

        (interfaces.imports.len() + interfaces.exports.len() - 1) as u32
    }

    /// Prepares types by insert collected function types and return resulted Interfaces.
    pub(crate) fn finalize(mut self) -> Interfaces<'a> {
        use itertools::Itertools;

        let function_types: Vec<_> = self
            .function_types
            .into_iter()
            .sorted_by(|(_, v1), (_, v2)| v1.cmp(&v2))
            .map(|(function_type, _)| AstType::Function(Rc::new(function_type)))
            .collect();
        self.interfaces.types.extend(function_types);

        self.interfaces
    }
}

pub(crate) trait WITGenerator {
    fn generate_wit<'ast_type, 'resolver>(&'ast_type self, wit_resolver: &'resolver mut WITResolver<'ast_type>) -> Result<()>;
}
