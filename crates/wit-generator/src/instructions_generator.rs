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

use wasmer_wit::types::InterfaceType as IType;
use wasmer_wit::ast::Interfaces;

#[derive(PartialEq, Debug, Default)]
pub(crate) struct WITResolver<'a> {
    pub(crate) types: std::collections::HashMap<String, u32>,
    pub(crate) interfaces: Interfaces<'a>,
}

impl<'a> WITResolver<'a> {
    pub(crate) fn get_record_type_id(&self, record_name: &str) -> Result<u32> {
        match self.types.get(record_name) {
            Some(type_index) => Ok(*type_index),
            None => Err(crate::errors::WITGeneratorError::CorruptedRecord(format!(
                "Can't find record with name='{}', don't you forget to wrap it with #[fce]",
                record_name
            ))),
        }
    }

    pub(crate) fn get_record_type(
        &self,
        record_name: &str,
    ) -> Result<wasmer_wit::types::RecordType> {
        match self.types.get(record_name) {
            Some(type_index) => match &self.interfaces.types[*type_index as usize] {
                wasmer_wit::ast::Type::Function { .. } => {
                    panic!("internal error inside WITResolver")
                }
                wasmer_wit::ast::Type::Record(record_type) => Ok(record_type.clone()),
            },
            None => Err(crate::errors::WITGeneratorError::CorruptedRecord(format!(
                "Can't find record with name='{}', don't you forget to wrap it with #[fce]",
                record_name
            ))),
        }
    }
}

pub(crate) trait WITGenerator {
    fn generate_wit<'a>(&'a self, wit_resolver: &mut WITResolver<'a>) -> Result<()>;
}
