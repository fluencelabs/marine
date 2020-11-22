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

use fluence_faas::FaaSModuleInterface;
use fluence_faas::FaaSFunctionSignature;
use fluence_faas::IRecordType;
use fluence_faas::RecordTypes;
use fluence_faas::itype_text_view;

use serde::Serialize;

use std::rc::Rc;

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

pub(crate) fn into_service_interface(faas_interface: FaaSModuleInterface<'_>) -> ServiceInterface {
    let record_types = faas_interface.record_types;

    let function_signatures = faas_interface
        .function_signatures
        .into_iter()
        .map(|sign| serialize_function_signature(sign, record_types))
        .collect();

    let record_types = record_types
        .iter()
        .map(|(id, record)| serialize_record_type(*id, record.clone(), record_types))
        .collect::<Vec<_>>();

    ServiceInterface {
        record_types,
        function_signatures,
    }
}

fn serialize_function_signature(
    signature: FaaSFunctionSignature,
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
