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

use super::ModuleInterface;
use super::InterfaceResult;
use super::FunctionSignature;
use super::records_transformer::RecordsTransformer;
use crate::it_interface::IModuleInterface;
use crate::it_interface::MFunctionSignature;
use crate::it_interface::MRecordTypes;

pub fn it_to_module_interface(mm_interface: IModuleInterface) -> InterfaceResult<ModuleInterface> {
    let record_types = mm_interface.export_record_types;

    let function_signatures = mm_interface
        .function_signatures
        .into_iter()
        .map(|sign| serialize_function_signature(sign, &record_types))
        .collect();

    let record_types = RecordsTransformer::transform(&record_types)?;

    let interface = ModuleInterface {
        function_signatures,
        record_types,
    };

    Ok(interface)
}

fn serialize_function_signature(
    signature: MFunctionSignature,
    record_types: &MRecordTypes,
) -> FunctionSignature {
    use super::itype_text_view;

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
