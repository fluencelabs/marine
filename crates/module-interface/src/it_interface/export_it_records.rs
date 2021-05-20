/*
 * Copyright 2021 Fluence Labs Limited
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

use super::IRecordTypes;
use super::RIResult;
use super::ITInterfaceError;
use super::IFunctionSignature;

use marine_it_interfaces::MITInterfaces;
use wasmer_it::IType;

use std::collections::HashMap;

const TYPE_RESOLVE_RECURSION_LIMIT: u32 = 1024;

pub struct FullRecordTypes {
    pub record_types: IRecordTypes,
    pub export_record_types: IRecordTypes,
}

pub fn get_record_types<'f>(
    mit: &MITInterfaces<'_>,
    export_funcs: impl ExactSizeIterator<Item = &'f IFunctionSignature>,
) -> RIResult<FullRecordTypes> {
    let all_record_types = get_all_records(mit);
    let mut export_record_types = HashMap::new();

    let itypes = export_funcs.flat_map(|s| {
        s.arguments
            .as_ref()
            .iter()
            .map(|a| &a.ty)
            .chain(s.outputs.as_ref().iter())
    });

    for itype in itypes {
        handle_itype(itype, &all_record_types, &mut export_record_types, 0)?;
    }

    let full_record_types = FullRecordTypes {
        record_types: all_record_types,
        export_record_types,
    };

    Ok(full_record_types)
}

fn handle_itype(
    itype: &IType,
    all_record_types: &IRecordTypes,
    export_record_types: &mut IRecordTypes,
    recursion_level: u32,
) -> RIResult<()> {
    if recursion_level > TYPE_RESOLVE_RECURSION_LIMIT {
        return Err(ITInterfaceError::TooManyRecursionLevels);
    }

    match itype {
        IType::Record(record_type_id) => handle_record_type(
            *record_type_id,
            all_record_types,
            export_record_types,
            recursion_level + 1,
        )?,
        IType::Array(array_ty) => handle_itype(
            array_ty,
            all_record_types,
            export_record_types,
            recursion_level + 1,
        )?,
        _ => {}
    }

    Ok(())
}

fn handle_record_type(
    record_type_id: u64,
    all_record_types: &IRecordTypes,
    export_record_types: &mut IRecordTypes,
    recursion_level: u32,
) -> RIResult<()> {
    let record_type = all_record_types
        .get(&record_type_id)
        .ok_or(ITInterfaceError::NotFoundRecordTypeId(record_type_id))?;

    export_record_types.insert(record_type_id, record_type.clone());

    for field in record_type.fields.iter() {
        handle_itype(
            &field.ty,
            all_record_types,
            export_record_types,
            recursion_level + 1,
        )?;
    }

    Ok(())
}

fn get_all_records(mit: &MITInterfaces<'_>) -> IRecordTypes {
    use marine_it_interfaces::ITAstType;

    mit.types()
        .enumerate()
        .fold(HashMap::new(), |mut record_types_by_id, (id, ty)| {
            match ty {
                ITAstType::Record(record_type) => {
                    record_types_by_id.insert(id as u64, record_type.clone());
                }
                ITAstType::Function { .. } => {}
            };

            record_types_by_id
        })
}
