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
