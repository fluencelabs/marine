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

use crate::Result;
use crate::WITParserError;
use fce_wit_interfaces::FCEWITInterfaces;

use wasmer_wit::IRecordType;
use wasmer_wit::ast::FunctionArg as IFunctionArg;
use wasmer_wit::IType;
use serde::Serialize;
use serde::Deserialize;

use std::collections::HashMap;
use std::rc::Rc;

pub type RecordTypes = HashMap<u64, Rc<IRecordType>>;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct FCEFunctionSignature {
    pub name: Rc<String>,
    pub arguments: Rc<Vec<IFunctionArg>>,
    pub outputs: Rc<Vec<IType>>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct FCEModuleInterface {
    pub record_types: RecordTypes,
    pub function_signatures: Vec<FCEFunctionSignature>,
}

pub fn get_interface(wit: &FCEWITInterfaces<'_>) -> Result<ServiceInterface> {
    let function_signatures = get_exports(wit)?;
    let record_types = extract_record_types(wit);

    let fce_interface = FCEModuleInterface {
        record_types,
        function_signatures,
    };

    let service_interface = into_service_interface(fce_interface);

    Ok(service_interface)
}

fn get_exports(wit: &FCEWITInterfaces<'_>) -> Result<Vec<FCEFunctionSignature>> {
    use fce_wit_interfaces::WITAstType;

    wit.implementations()
        .filter_map(|(adapter_function_type, core_function_type)| {
            match wit.exports_by_type(*core_function_type) {
                Some(export_function_name) => Some((adapter_function_type, export_function_name)),
                // pass functions that aren't export
                None => None,
            }
        })
        .map(|(adapter_function_type, export_function_names)| {
            export_function_names
                .iter()
                .map(move |export_function_name| (*adapter_function_type, export_function_name))
        })
        .flatten()
        .map(|(adapter_function_type, export_function_name)| {
            let wit_type = wit.type_by_idx_r(adapter_function_type).unwrap();

            match wit_type {
                WITAstType::Function {
                    arguments,
                    output_types,
                } => {
                    let signature = FCEFunctionSignature {
                        name: Rc::new(export_function_name.to_string()),
                        arguments: arguments.clone(),
                        outputs: output_types.clone(),
                    };
                    Ok(signature)
                }
                _ => Err(WITParserError::IncorrectWIT(format!(
                    "type with idx = {} isn't a function type",
                    adapter_function_type
                ))),
            }
        })
        .collect::<Result<Vec<FCEFunctionSignature>>>()
}

fn extract_record_types(wit: &FCEWITInterfaces<'_>) -> RecordTypes {
    use fce_wit_interfaces::WITAstType;

    let (record_types_by_id, _) = wit.types().fold(
        (HashMap::new(), 0u64),
        |(mut record_types_by_id, id), ty| {
            match ty {
                WITAstType::Record(record_type) => {
                    record_types_by_id.insert(id, record_type.clone());
                }
                WITAstType::Function { .. } => {}
            };
            (record_types_by_id, id + 1)
        },
    );

    record_types_by_id
}

#[derive(Serialize)]
pub struct FunctionSignature {
    pub name: String,
    pub arguments: Vec<(String, String)>,
    pub output_types: Vec<String>,
}

#[derive(Serialize)]
pub struct RecordType {
    pub name: String,
    pub id: u64,
    pub fields: Vec<(String, String)>,
}

#[derive(Serialize)]
pub struct ServiceInterface {
    pub function_signatures: Vec<FunctionSignature>,
    pub record_types: Vec<RecordType>,
}

pub(crate) fn into_service_interface(fce_interface: FCEModuleInterface) -> ServiceInterface {
    let record_types = fce_interface.record_types;

    let function_signatures = fce_interface
        .function_signatures
        .into_iter()
        .map(|sign| serialize_function_signature(sign, &record_types))
        .collect();

    let record_types = record_types
        .iter()
        .map(|(id, record)| serialize_record_type(*id, record.clone(), &record_types))
        .collect::<Vec<_>>();

    ServiceInterface {
        record_types,
        function_signatures,
    }
}

fn serialize_function_signature(
    signature: FCEFunctionSignature,
    record_types: &RecordTypes,
) -> FunctionSignature {
    let arguments = signature
        .arguments
        .iter()
        .map(|arg| (arg.name.clone(), itype_text_view(&arg.ty, record_types)))
        .collect();

    let output_types = signature
        .outputs
        .iter()
        .map(|itype| itype_text_view(itype, record_types))
        .collect();

    FunctionSignature {
        name: signature.name.to_string(),
        arguments,
        output_types,
    }
}

fn serialize_record_type<'a, 'b>(
    id: u64,
    record: Rc<IRecordType>,
    record_types: &RecordTypes,
) -> RecordType {
    let fields = record
        .fields
        .iter()
        .map(|field| (field.name.clone(), itype_text_view(&field.ty, record_types)))
        .collect::<Vec<_>>();

    RecordType {
        name: record.name.clone(),
        id,
        fields,
    }
}

fn itype_text_view(arg_ty: &IType, record_types: &RecordTypes) -> String {
    match arg_ty {
        IType::Record(record_type_id) => {
            // unwrap is safe because FaaSInterface here is well-formed
            // (it was checked on the module startup stage)
            let record = record_types.get(record_type_id).unwrap();
            record.name.clone()
        }
        IType::Array(array_ty) => format!("Array<{}>", itype_text_view(array_ty, record_types)),
        t => format!("{:?}", t),
    }
}
