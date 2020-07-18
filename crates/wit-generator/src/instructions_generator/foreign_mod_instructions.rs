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

use super::WITGenerator;
use super::Interfaces;
use super::utils::ptype_to_itype;
use super::ForeignModInstructionGenerator;
use crate::default_export_api_config::*;

use fluence_sdk_wit::AstExternModItem;
use fluence_sdk_wit::AstExternFnItem;
use fluence_sdk_wit::ParsedType;
use wasmer_wit::interpreter::Instruction;
use crate::instructions_generator::utils::wtype_to_itype;

const HOST_NAMESPACE_NAME: &str = "host";

impl WITGenerator for AstExternModItem {
    fn generate_wit<'a>(&'a self, interfaces: &mut Interfaces<'a>) {
        // host imports should be left as is
        if self.namespace == HOST_NAMESPACE_NAME {
            return;
        }

        for import in &self.imports {
            generate_wit_for_import(import, &self.namespace, interfaces);
        }
    }
}

fn generate_wit_for_import<'a>(
    import: &'a AstExternFnItem,
    namespace: &'a str,
    interfaces: &mut Interfaces<'a>,
) {
    use wasmer_wit::ast::Type;
    use wasmer_wit::ast::Adapter;

    let inputs = import
        .signature
        .input_types
        .iter()
        .map(ptype_to_itype)
        .collect::<Vec<_>>();

    let outputs = match import.signature.output_type {
        Some(ref output_type) => vec![ptype_to_itype(output_type)],
        None => vec![],
    };

    interfaces.types.push(Type::Function { inputs, outputs });

    let raw_inputs = import
        .signature
        .input_types
        .iter()
        .map(to_raw_input_types)
        .flatten()
        .map(|wt| wtype_to_itype(&wt))
        .collect::<Vec<_>>();

    let raw_outputs = match import.signature.output_type {
        Some(ref output_type) => to_raw_output_type(output_type)
            .iter()
            .map(wtype_to_itype)
            .collect(),
        None => vec![],
    };

    interfaces.types.push(Type::Function {
        inputs: raw_inputs.clone(),
        outputs: raw_outputs.clone(),
    });

    interfaces.types.push(Type::Function {
        inputs: raw_inputs,
        outputs: raw_outputs,
    });

    let adapter_idx = (interfaces.types.len() - 2) as u32;
    let import_idx = (interfaces.types.len() - 3) as u32;
    let raw_import_idx = (interfaces.types.len() - 1) as u32;

    let link_name = match &import.link_name {
        Some(link_name) => link_name,
        None => &import.signature.name,
    };

    interfaces.imports.push(wasmer_wit::ast::Import {
        namespace: &namespace,
        name: link_name,
        function_type: import_idx,
    });

    interfaces.imports.push(wasmer_wit::ast::Import {
        namespace: &namespace,
        name: link_name,
        function_type: raw_import_idx,
    });

    let mut instructions: Vec<Instruction> = import
        .signature
        .input_types
        .iter()
        .enumerate()
        .map(|(id, input_type)| input_type.generate_instructions_for_input_type(id as _))
        .flatten()
        .collect();

    // TODO: refactor
    let import_function_index =
        (interfaces.exports.len() + interfaces.imports.len() / 2 - 1) as u32;
    instructions.push(Instruction::CallCore {
        function_index: import_function_index,
    });

    instructions.extend(match &import.signature.output_type {
        Some(output_type) => output_type.generate_instructions_for_output_type(),
        None => vec![],
    });

    let adapter = Adapter {
        function_type: adapter_idx,
        instructions,
    };
    interfaces.adapters.push(adapter);

    let implementation = wasmer_wit::ast::Implementation {
        core_function_type: raw_import_idx,
        adapter_function_type: adapter_idx,
    };
    interfaces.implementations.push(implementation);
}

impl ForeignModInstructionGenerator for ParsedType {
    fn generate_instructions_for_input_type(&self, index: u32) -> Vec<Instruction> {
        match self {
            ParsedType::I8 => vec![Instruction::ArgumentGet { index }, Instruction::S8FromI32],
            ParsedType::I16 => vec![Instruction::ArgumentGet { index }, Instruction::S16FromI32],
            ParsedType::I32 => vec![Instruction::ArgumentGet { index }],
            ParsedType::I64 => vec![Instruction::ArgumentGet { index }],
            ParsedType::U8 => vec![Instruction::ArgumentGet { index }, Instruction::U8FromI32],
            ParsedType::U16 => vec![Instruction::ArgumentGet { index }, Instruction::U16FromI32],
            ParsedType::U32 => vec![Instruction::ArgumentGet { index }, Instruction::U32FromI32],
            ParsedType::U64 => vec![Instruction::ArgumentGet { index }, Instruction::U64FromI64],
            ParsedType::F32 => vec![Instruction::ArgumentGet { index }],
            ParsedType::F64 => vec![Instruction::ArgumentGet { index }],
            ParsedType::Utf8String => vec![
                Instruction::ArgumentGet { index },
                Instruction::ArgumentGet { index: index + 1 },
                Instruction::StringLiftMemory,
            ],
            ParsedType::ByteVector => vec![
                Instruction::ArgumentGet { index },
                Instruction::ArgumentGet { index: index + 1 },
                Instruction::StringLiftMemory,
            ],
            _ => unimplemented!(),
        }
    }

    #[rustfmt::skip]
    fn generate_instructions_for_output_type(&self) -> Vec<Instruction> {
        match self {
            ParsedType::I8 => vec![Instruction::I32FromS8],
            ParsedType::I16 => vec![Instruction::I32FromS16],
            ParsedType::I32 => vec![],
            ParsedType::I64 => vec![],
            ParsedType::U8 => vec![Instruction::I32FromU8],
            ParsedType::U16 => vec![Instruction::I32FromU16],
            ParsedType::U32 => vec![Instruction::I32FromU32],
            ParsedType::U64 => vec![Instruction::I64FromU64],
            ParsedType::F32 => vec![],
            ParsedType::F64 => vec![],
            ParsedType::Utf8String => vec![
                Instruction::Dup,
                Instruction::StringSize,
                Instruction::CallCore { function_index: ALLOCATE_FUNC.id },
                Instruction::Swap2,
                Instruction::StringLowerMemory,
                Instruction::CallCore { function_index: SET_RESULT_SIZE_FUNC.id },
                Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
            ],
            ParsedType::ByteVector => vec![
                Instruction::Dup,
                Instruction::ByteArraySize,
                Instruction::CallCore { function_index: ALLOCATE_FUNC.id },
                Instruction::Swap2,
                Instruction::ByteArrayLowerMemory,
                Instruction::CallCore { function_index: SET_RESULT_SIZE_FUNC.id },
                Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
            ],
            _ => unimplemented!(),
        }
    }
}

use fluence_sdk_wit::WasmType;

pub fn to_raw_input_types(ty: &ParsedType) -> Vec<WasmType> {
    match ty {
        ParsedType::Boolean
        | ParsedType::I8
        | ParsedType::I16
        | ParsedType::I32
        | ParsedType::U8
        | ParsedType::U16
        | ParsedType::U32 => vec![WasmType::I32],
        ParsedType::I64 | ParsedType::U64 => vec![WasmType::I64],
        ParsedType::F32 => vec![WasmType::F32],
        ParsedType::F64 => vec![WasmType::F64],
        ParsedType::Utf8String | ParsedType::ByteVector | ParsedType::Record(_) => {
            vec![WasmType::I32, WasmType::I32]
        }
    }
}

pub fn to_raw_output_type(ty: &ParsedType) -> Vec<WasmType> {
    match ty {
        ParsedType::Boolean
        | ParsedType::I8
        | ParsedType::I16
        | ParsedType::I32
        | ParsedType::U8
        | ParsedType::U16
        | ParsedType::U32 => vec![WasmType::I32],
        ParsedType::I64 | ParsedType::U64 => vec![WasmType::I64],
        ParsedType::F32 => vec![WasmType::F32],
        ParsedType::F64 => vec![WasmType::F64],
        ParsedType::Utf8String | ParsedType::ByteVector | ParsedType::Record(_) => vec![],
    }
}
