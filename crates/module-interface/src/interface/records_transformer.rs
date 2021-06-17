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

use super::RecordType;
use super::RecordField;
use super::InterfaceResult;
use super::InterfaceError;
use crate::it_interface::IRecordTypes;

use wasmer_it::IRecordType;
use wasmer_it::IType;

use std::collections::HashSet;
use std::rc::Rc;
use itertools::Itertools;

pub(crate) struct RecordsTransformer {
    used: HashSet<u64>,
    sorted_order: Vec<u64>,
}

impl RecordsTransformer {
    pub(crate) fn transform(record_types: &IRecordTypes) -> InterfaceResult<Vec<RecordType>> {
        let records_count = record_types.len();

        let mut transformer = Self {
            used: HashSet::with_capacity(records_count),
            sorted_order: Vec::with_capacity(records_count),
        };

        // TODO: check for cycles
        transformer.topological_sort(record_types)?;
        let record_types = transformer.into_transformed_records(record_types);

        Ok(record_types)
    }

    fn topological_sort(&mut self, exported_records: &IRecordTypes) -> InterfaceResult<()> {
        for (id, record) in exported_records.iter().sorted_by_key(|(_, v)| &v.name) {
            self.dfs(*id, record, exported_records)?;
        }

        Ok(())
    }

    fn dfs(
        &mut self,
        record_id: u64,
        record: &Rc<IRecordType>,
        exported_records: &IRecordTypes,
    ) -> InterfaceResult<()> {
        if !self.used.insert(record_id) {
            return Ok(());
        }

        for field in (&record.fields).iter() {
            self.type_dfs(&field.ty, exported_records)?;
        }

        self.sorted_order.push(record_id);

        Ok(())
    }

    fn type_dfs(
        &mut self,
        field_ty: &IType,
        exported_records: &IRecordTypes,
    ) -> InterfaceResult<()> {
        match field_ty {
            IType::Record(type_id) => {
                let child_record = exported_records
                    .get(type_id)
                    .ok_or(InterfaceError::NotFoundRecordTypeId(*type_id))?;

                self.dfs(*type_id, child_record, exported_records)
            }
            IType::Array(ty) => self.type_dfs(ty, exported_records),
            _ => Ok(()),
        }
    }

    fn into_transformed_records(self, record_types: &IRecordTypes) -> Vec<RecordType> {
        self.sorted_order
            .into_iter()
            .map(|id| {
                // unwrap is safe here because sorted_order is constructed based on record_types
                let record = record_types.get(&id).unwrap();
                Self::convert_record(id, record, &record_types)
            })
            .collect::<Vec<_>>()
    }

    fn convert_record(
        id: u64,
        record: &Rc<IRecordType>,
        record_types: &IRecordTypes,
    ) -> RecordType {
        use super::itype_text_view;

        let fields = record
            .fields
            .iter()
            .map(|field| RecordField {
                name: field.name.clone(),
                ty: itype_text_view(&field.ty, record_types),
            })
            .collect::<Vec<_>>();

        RecordType {
            name: record.name.clone(),
            id,
            fields,
        }
    }
}
