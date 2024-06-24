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

use super::IFunctionSignature;
use super::ITInterfaceError;
use super::RIResult;

use marine_it_interfaces::MITInterfaces;

use std::sync::Arc;

pub struct ITExportFuncDescriptor<'n> {
    pub adapter_function_type: u32,
    pub name: &'n str,
}

/// Returns all exported IT functions descriptors.
pub fn get_export_funcs_descriptors<'i>(
    mit: &'i MITInterfaces<'_>,
) -> Vec<ITExportFuncDescriptor<'i>> {
    // An IT function is exported if it lies in export functions and have implementation.
    // An export IT function without implementation is a hack and used to call core function from
    // a Wasm module. This hack is needed because there is only one call instruction in the
    // interface-types crates and it's needed to distinguish somehow between calling export IT or
    // core functions. This scheme is a kind of mess and it needs to be refactored one day.
    mit.implementations()
        .filter_map(|(adapter_function_type, core_function_type)| {
            mit.exports_by_type(*core_function_type)
                .map(|export_function_name| (adapter_function_type, export_function_name))
        })
        .flat_map(|(&adapter_function_type, export_function_names)| {
            export_function_names
                .iter()
                .map(move |name| ITExportFuncDescriptor {
                    adapter_function_type,
                    name,
                })
        })
        .collect::<Vec<_>>()
}

/// Returns all exported IT functions.
pub fn get_export_funcs(mit: &MITInterfaces<'_>) -> RIResult<Vec<IFunctionSignature>> {
    use marine_it_interfaces::ITAstType;

    let funcs_descriptors = get_export_funcs_descriptors(mit);

    funcs_descriptors
        .into_iter()
        .map(|descriptor| {
            let it_type = mit.type_by_idx_r(descriptor.adapter_function_type)?;

            match it_type {
                ITAstType::Function {
                    arguments,
                    output_types,
                } => {
                    let signature = IFunctionSignature {
                        name: Arc::new(descriptor.name.to_string()),
                        arguments: arguments.clone(),
                        outputs: output_types.clone(),
                        adapter_function_type: descriptor.adapter_function_type,
                    };
                    Ok(signature)
                }
                _ => Err(ITInterfaceError::ITTypeNotFunction(
                    descriptor.adapter_function_type,
                )),
            }
        })
        .collect::<RIResult<Vec<IFunctionSignature>>>()
}
