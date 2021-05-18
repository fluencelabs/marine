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
use crate::ITParserError;
use marine_it_interfaces::MITInterfaces;

use wasmer_it::IRecordType;
use wasmer_it::ast::FunctionArg as IFunctionArg;
use wasmer_it::IType;
use serde::Serialize;
use serde::Deserialize;

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Formatter;

pub type MRecordTypes = HashMap<u64, Rc<IRecordType>>;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct MFunctionSignature {
    pub name: Rc<String>,
    pub arguments: Rc<Vec<IFunctionArg>>,
    pub outputs: Rc<Vec<IType>>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct MModuleInterface {
    pub record_types: MRecordTypes,
    pub function_signatures: Vec<MFunctionSignature>,
}

pub fn get_interface(mit: &MITInterfaces<'_>) -> Result<ServiceInterface> {
    let marine_interface = get_raw_interface(mit)?;
    let service_interface = into_service_interface(marine_interface);

    Ok(service_interface)
}

pub fn get_raw_interface(mit: &MITInterfaces<'_>) -> Result<MModuleInterface> {
    let function_signatures = get_exports(mit)?;
    let record_types = extract_record_types(mit);

    let mm_interface = MModuleInterface {
        record_types,
        function_signatures,
    };

    Ok(mm_interface)
}

fn get_exports(it: &MITInterfaces<'_>) -> Result<Vec<MFunctionSignature>> {
    use marine_it_interfaces::ITAstType;

    it.implementations()
        .filter_map(|(adapter_function_type, core_function_type)| {
            it.exports_by_type(*core_function_type)
                .map(|export_function_name| (adapter_function_type, export_function_name))
        })
        .map(|(adapter_function_type, export_function_names)| {
            export_function_names
                .iter()
                .map(move |export_function_name| (*adapter_function_type, export_function_name))
        })
        .flatten()
        .map(|(adapter_function_type, export_function_name)| {
            let it_type = it.type_by_idx_r(adapter_function_type).unwrap();

            match it_type {
                ITAstType::Function {
                    arguments,
                    output_types,
                } => {
                    let signature = MFunctionSignature {
                        name: Rc::new(export_function_name.to_string()),
                        arguments: arguments.clone(),
                        outputs: output_types.clone(),
                    };
                    Ok(signature)
                }
                _ => Err(ITParserError::IncorrectITFormat(format!(
                    "type with idx = {} isn't a function type",
                    adapter_function_type
                ))),
            }
        })
        .collect::<Result<Vec<MFunctionSignature>>>()
}

fn extract_record_types(it: &MITInterfaces<'_>) -> MRecordTypes {
    use marine_it_interfaces::ITAstType;

    let (record_types_by_id, _) = it.types().fold(
        (HashMap::new(), 0u64),
        |(mut record_types_by_id, id), ty| {
            match ty {
                ITAstType::Record(record_type) => {
                    record_types_by_id.insert(id, record_type.clone());
                }
                ITAstType::Function { .. } => {}
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

use std::fmt;

impl fmt::Display for FunctionSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use itertools::Itertools;

        let output = match self.output_types.len() {
            0 => "()",
            1 => &self.output_types[0],
            _ => unimplemented!("more than 1 output type is unsupported"),
        };

        if self.arguments.is_empty() {
            writeln!(f, "{}: -> {}", self.name, output)
        } else {
            let args = self.arguments.iter().map(|(_, ty)| ty).format(",");
            writeln!(f, "{}: {} -> {}", self.name, args, output)
        }
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "data {}:", self.name)?;

        for (name, ty) in self.fields.iter() {
            writeln!(f, "  {}: {}", name, ty)?;
        }

        Ok(())
    }
}

#[derive(Serialize)]
pub struct ServiceInterface {
    pub function_signatures: Vec<FunctionSignature>,
    pub record_types: Vec<RecordType>,
}

pub(crate) fn into_service_interface(mm_interface: MModuleInterface) -> ServiceInterface {
    let record_types = mm_interface.record_types;

    let function_signatures = mm_interface
        .function_signatures
        .into_iter()
        .map(|sign| serialize_function_signature(sign, &record_types))
        .collect();

    let record_types = record_types
        .iter()
        .map(|(id, record)| serialize_record_type(*id, record.clone(), &record_types))
        .collect::<Vec<_>>();

    ServiceInterface {
        function_signatures,
        record_types,
    }
}

fn serialize_function_signature(
    signature: MFunctionSignature,
    record_types: &MRecordTypes,
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

fn serialize_record_type(
    id: u64,
    record: Rc<IRecordType>,
    record_types: &MRecordTypes,
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

// TODO: refactor it to avoid code deduplication
fn itype_text_view(arg_ty: &IType, record_types: &MRecordTypes) -> String {
    match arg_ty {
        IType::Record(record_type_id) => {
            // unwrap is safe because FaaSInterface here is well-formed
            // (it was checked on the module startup stage)
            let record = record_types.get(record_type_id).unwrap();
            record.name.clone()
        }
        IType::Array(array_ty) => format!("[]{}", itype_text_view(array_ty, record_types)),
        IType::Boolean => "bool".to_string(),
        IType::S8 => "i8".to_string(),
        IType::S16 => "i16".to_string(),
        IType::S32 => "i32".to_string(),
        IType::S64 => "i64".to_string(),
        IType::U8 => "u8".to_string(),
        IType::U16 => "u16".to_string(),
        IType::U32 => "u32".to_string(),
        IType::U64 => "u64".to_string(),
        IType::F32 => "f32".to_string(),
        IType::F64 => "f64".to_string(),
        IType::String => "string".to_string(),
        IType::ByteArray => "[]u8".to_string(),
        IType::I32 => "i32".to_string(),
        IType::I64 => "i64".to_string(),
    }
}
