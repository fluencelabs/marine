/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::ITGenerator;
use super::ITResolver;
use crate::Result;

use marine_macro_impl::RecordType;
use marine_macro_impl::RecordFields;

use wasmer_it::IRecordFieldType;
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

        it_resolver.add_record_type(self.name.clone(), fields);

        Ok(())
    }
}
