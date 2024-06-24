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

mod fn_instructions;
mod foreign_mod_instructions;
mod record_instructions;
#[cfg(test)]
mod tests;
mod utils;

use crate::Result;

use wasmer_it::IType;
use wasmer_it::ast::Interfaces;
use wasmer_it::IRecordType;

use std::sync::Arc;

#[derive(PartialEq, Eq, Debug, Default)]
pub(crate) struct ITResolver<'a> {
    types: std::collections::HashMap<String, usize>,
    pub(crate) interfaces: Interfaces<'a>,
    unresolved_types_count: usize,
}

impl<'a> ITResolver<'a> {
    pub(crate) fn get_record_type_id(&self, record_name: &str) -> Result<usize> {
        match self.types.get(record_name) {
            Some(type_index) => Ok(*type_index),
            None => Err(crate::errors::ITGeneratorError::CorruptedRecord(format!(
                "Can't find record with name='{}', don't you forget to wrap it with #[marine]",
                record_name
            ))),
        }
    }

    // adds a stub for type with such a name if it wasn't found
    pub(crate) fn get_record_type_id_unchecked(&mut self, record_name: &str) -> usize {
        use wasmer_it::ast::Type;

        match self.types.get(record_name) {
            Some(type_index) => *type_index,
            None => {
                let new_type_id = self.interfaces.types.len();
                self.types.insert(record_name.to_string(), new_type_id);
                self.interfaces
                    .types
                    .push(Type::Record(Arc::new(IRecordType::default())));

                self.unresolved_types_count += 1;
                new_type_id
            }
        }
    }

    pub(crate) fn get_record_type(&self, record_type_id: u64) -> Result<&IRecordType> {
        if record_type_id >= self.interfaces.types.len() as u64 {
            return Err(crate::errors::ITGeneratorError::CorruptedRecord(format!(
                "Can't find record with id {}, don't you forget to wrap it with #[marine]",
                record_type_id
            )));
        }

        match &self.interfaces.types[record_type_id as usize] {
            wasmer_it::ast::Type::Function { .. } => {
                panic!("internal error inside ITResolver: interfaces AST type should be record not record")
            }
            wasmer_it::ast::Type::Record(record_type) => Ok(record_type),
        }
    }

    pub(crate) fn insert_record_type(&mut self, record: IRecordType) {
        use wasmer_it::ast::Type;

        match self.types.get(&record.name) {
            Some(pos) => {
                self.interfaces.types[*pos] = Type::Record(Arc::new(record));
                self.unresolved_types_count -= 1;
            }
            None => {
                self.types
                    .insert(record.name.clone(), self.interfaces.types.len());

                self.interfaces.types.push(Type::Record(Arc::new(record)));
            }
        }
    }

    pub(crate) fn unresolved_types_count(&self) -> usize {
        self.unresolved_types_count
    }
}

impl<'a> ITResolver<'a> {
    pub(crate) fn add_adapter(
        &mut self,
        function_type: u32,
        instructions: Vec<wasmer_it::interpreter::Instruction>,
    ) {
        let adapter = wasmer_it::ast::Adapter {
            function_type,
            instructions,
        };

        self.interfaces.adapters.push(adapter);
    }

    pub(crate) fn add_implementation(
        &mut self,
        core_function_type: u32,
        adapter_function_type: u32,
    ) {
        let implementation = wasmer_it::ast::Implementation {
            core_function_type,
            adapter_function_type,
        };

        self.interfaces.implementations.push(implementation);
    }

    pub(crate) fn add_export(&mut self, name: &'a str, function_type: u32) {
        let export = wasmer_it::ast::Export {
            name,
            function_type,
        };

        self.interfaces.exports.push(export);
    }

    pub(crate) fn add_import(&mut self, namespace: &'a str, name: &'a str, function_type: u32) {
        let import = wasmer_it::ast::Import {
            namespace,
            name,
            function_type,
        };

        self.interfaces.imports.push(import);
    }

    pub(crate) fn add_fn_type(
        &mut self,
        arguments: Arc<Vec<wasmer_it::ast::FunctionArg>>,
        output_types: Arc<Vec<IType>>,
    ) {
        let fn_type = wasmer_it::ast::Type::Function {
            arguments,
            output_types,
        };

        self.interfaces.types.push(fn_type);
    }

    pub(crate) fn add_record_type(
        &mut self,
        name: String,
        fields: wasmer_it::NEVec<wasmer_it::IRecordFieldType>,
    ) {
        let record = wasmer_it::IRecordType { name, fields };
        self.insert_record_type(record);
    }
}

pub(crate) trait ITGenerator {
    fn generate_it<'a>(&'a self, it_resolver: &mut ITResolver<'a>) -> Result<()>;
}
