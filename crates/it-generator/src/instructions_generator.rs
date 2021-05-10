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

use crate::Result;

use wasmer_it::IType;
use wasmer_it::ast::Interfaces;
use wasmer_it::IRecordType;

use std::rc::Rc;

#[derive(PartialEq, Debug, Default)]
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
                "Can't find record with name='{}', don't you forget to wrap it with #[fce]",
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
                    .push(Type::Record(Rc::new(IRecordType::default())));

                self.unresolved_types_count += 1;
                new_type_id
            }
        }
    }

    pub(crate) fn get_record_type(&self, record_type_id: u64) -> Result<&IRecordType> {
        if record_type_id >= self.interfaces.types.len() as u64 {
            return Err(crate::errors::ITGeneratorError::CorruptedRecord(format!(
                "Can't find record with id {}, don't you forget to wrap it with #[fce]",
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
                self.interfaces.types[*pos] = Type::Record(Rc::new(record));
                self.unresolved_types_count -= 1;
            }
            None => {
                self.types
                    .insert(record.name.clone(), self.interfaces.types.len());

                self.interfaces.types.push(Type::Record(Rc::new(record)));
            }
        }
    }

    pub(crate) fn unresolved_types_count(&self) -> usize {
        self.unresolved_types_count
    }
}

pub(crate) trait ITGenerator {
    fn generate_it<'a>(&'a self, it_resolver: &mut ITResolver<'a>) -> Result<()>;
}
