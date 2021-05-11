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

use super::ITGenerator;
use super::ITResolver;
use crate::Result;

use marine_macro_impl::RecordType;
use marine_macro_impl::RecordFields;

use wasmer_it::IRecordFieldType;
use wasmer_it::IRecordType;
use wasmer_it::NEVec;

impl ITGenerator for RecordType {
    fn generate_it<'a>(&'a self, it_resolver: &mut ITResolver<'a>) -> Result<()> {
        let fields = match &self.fields {
            RecordFields::Named(fields) => fields,
            RecordFields::Unnamed(fields) => fields,
            RecordFields::Unit => return Ok(()),
        };

        let fields = fields
            .iter()
            .map(|field| IRecordFieldType {
                name: field.name.clone().unwrap_or_default(),
                ty: super::utils::ptype_to_itype_unchecked(&field.ty, it_resolver),
            })
            .collect::<Vec<_>>();

        let fields = NEVec::new(fields).map_err(|_| {
            crate::errors::ITGeneratorError::CorruptedRecord(format!(
                "serialized record with name '{}' contains no fields",
                self.name
            ))
        })?;

        let new_record_type = IRecordType {
            name: self.name.clone(),
            fields,
        };

        it_resolver.insert_record_type(new_record_type);

        Ok(())
    }
}
