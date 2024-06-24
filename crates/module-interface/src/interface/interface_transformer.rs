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

use super::ModuleInterface;
use super::InterfaceResult;
use super::FunctionSignature;
use super::records_transformer::RecordsTransformer;
use crate::it_interface::IModuleInterface;
use crate::it_interface::IFunctionSignature;
use crate::it_interface::IRecordTypes;

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
    signature: IFunctionSignature,
    record_types: &IRecordTypes,
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
