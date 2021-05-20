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

use super::MFunctionSignature;
use super::ITInterfaceError;
use super::RIResult;

use marine_it_interfaces::MITInterfaces;

use std::rc::Rc;

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
        .map(|(&adapter_function_type, export_function_names)| {
            export_function_names
                .iter()
                .map(move |name| ITExportFuncDescriptor {
                    adapter_function_type,
                    name,
                })
        })
        .flatten()
        .collect::<Vec<_>>()
}

/// Returns all exported IT functions.
pub fn get_export_funcs(mit: &MITInterfaces<'_>) -> RIResult<Vec<MFunctionSignature>> {
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
                    let signature = MFunctionSignature {
                        name: Rc::new(descriptor.name.to_string()),
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
        .collect::<RIResult<Vec<MFunctionSignature>>>()
}
