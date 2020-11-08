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

use fluence_sdk_wit::AstRecordItem;

use wasmer_wit::types::RecordFieldType as IRecordFieldType;
use wasmer_wit::types::RecordType;
use wasmer_wit::vec1::Vec1;

impl WITGenerator for AstRecordItem {
    fn generate_wit<'ast_type, 'resolver>(
        &'ast_type self,
        wit_resolver: &'resolver mut WITResolver<'ast_type>,
    ) -> Result<()> {
        use super::utils::ptype_to_itype_unchecked;

        let fields = self
            .fields
            .iter()
            .map(|field| IRecordFieldType {
                name: field.name.clone().unwrap_or_default(),
                ty: ptype_to_itype_unchecked(&field.ty, wit_resolver),
            })
            .collect::<Vec<_>>();

        let fields = Vec1::new(fields).map_err(|_| {
            crate::errors::WITGeneratorError::CorruptedRecord(format!(
                "serialized record with name '{}' contains no fields",
                self.name
            ))
        })?;

        let new_record_type = RecordType {
            name: self.name.clone(),
            fields,
        };

        wit_resolver.insert_record_type(new_record_type);

        Ok(())
    }
}
