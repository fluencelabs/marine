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

use super::WITGenerator;
use super::WITResolver;
use crate::Result;

use fluence_sdk_wit::RecordItem;
use fluence_sdk_wit::RecordFields;

use wasmer_wit::IRecordFieldType;
use wasmer_wit::IRecordType;
use wasmer_wit::NEVec;

impl WITGenerator for RecordItem {
    fn generate_wit<'a>(&'a self, wit_resolver: &mut WITResolver<'a>) -> Result<()> {
        let fields = match &self.fields {
            RecordFields::Named(fields) => fields,
            RecordFields::Unnamed(fields) => fields,
            RecordFields::Unit => return Ok(()),
        };

        let fields = fields
            .iter()
            .map(|field| IRecordFieldType {
                name: field.name.clone().unwrap_or_default(),
                ty: super::utils::ptype_to_itype_unchecked(&field.ty, wit_resolver),
            })
            .collect::<Vec<_>>();

        let fields = NEVec::new(fields).map_err(|_| {
            crate::errors::WITGeneratorError::CorruptedRecord(format!(
                "serialized record with name '{}' contains no fields",
                self.name
            ))
        })?;

        let new_record_type = IRecordType {
            name: self.name.clone(),
            fields,
        };

        wit_resolver.insert_record_type(new_record_type);

        Ok(())
    }
}
