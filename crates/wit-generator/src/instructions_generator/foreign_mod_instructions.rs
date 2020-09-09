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
use super::WITResolver;
use super::utils::ptype_to_itype;
use crate::default_export_api_config::*;
use crate::Result;

use fluence_sdk_wit::AstExternModItem;
use fluence_sdk_wit::AstExternFnItem;
use fluence_sdk_wit::ParsedType;
use wasmer_wit::interpreter::Instruction;
use crate::instructions_generator::utils::wtype_to_itype;

const HOST_NAMESPACE_NAME: &str = "host";

impl WITGenerator for AstExternModItem {
    fn generate_wit<'a>(&'a self, wit_resolver: &mut WITResolver<'a>) -> Result<()> {
        // host imports should be left as is
        if self.namespace == HOST_NAMESPACE_NAME {
            return Ok(());
        }

        for import in &self.imports {
            generate_wit_for_import(import, &self.namespace, wit_resolver)?;
        }

        Ok(())
    }
}

fn generate_wit_for_import<'a>(
    import: &'a AstExternFnItem,
    namespace: &'a str,
    wit_resolver: &mut WITResolver<'a>,
) -> Result<()> {
    use wasmer_wit::ast::Type;
    use wasmer_wit::ast::Adapter;

    let inputs = import
        .signature
        .input_types
        .iter()
        .map(|input_type| ptype_to_itype(input_type, wit_resolver))
        .collect::<Result<Vec<_>>>()?;

    let outputs = match import.signature.output_type {
        Some(ref output_type) => vec![ptype_to_itype(output_type, wit_resolver)?],
        None => vec![],
    };

    let interfaces = &mut wit_resolver.interfaces;
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

    let mut instructions = import
        .signature
        .input_types
        .iter()
        .try_fold::<_, _, Result<_>>((0, Vec::new()), |(arg_id, mut instructions), input_type| {
            let (mut new_instructions, shift) =
                input_type.generate_instructions_for_input_type(arg_id as _, wit_resolver)?;

            instructions.append(&mut new_instructions);
            Ok((arg_id + shift, instructions))
        })?
        .1;

    // TODO: refactor
    let import_function_index = (wit_resolver.interfaces.exports.len()
        + wit_resolver.interfaces.imports.len() / 2
        - 1) as u32;
    instructions.push(Instruction::CallCore {
        function_index: import_function_index,
    });

    instructions.extend(match &import.signature.output_type {
        Some(output_type) => output_type.generate_instructions_for_output_type(wit_resolver)?,
        None => vec![],
    });

    let adapter = Adapter {
        function_type: adapter_idx,
        instructions,
    };
    wit_resolver.interfaces.adapters.push(adapter);

    let implementation = wasmer_wit::ast::Implementation {
        core_function_type: raw_import_idx,
        adapter_function_type: adapter_idx,
    };
    wit_resolver.interfaces.implementations.push(implementation);

    Ok(())
}

/// Generate WIT instructions for a foreign mod.
trait ForeignModInstructionGenerator {
    fn generate_instructions_for_input_type<'a>(
        &self,
        arg_id: u32,
        wit_resolver: &mut WITResolver<'a>,
    ) -> Result<(Vec<Instruction>, u32)>;

    fn generate_instructions_for_output_type<'a>(
        &self,
        wit_resolver: &mut WITResolver<'a>,
    ) -> Result<Vec<Instruction>>;
}

#[rustfmt::skip]
impl ForeignModInstructionGenerator for ParsedType {
    fn generate_instructions_for_input_type<'a>(
        &self,
        index: u32,
        wit_resolver: &mut WITResolver<'a>,
    ) -> Result<(Vec<Instruction>, u32)> {
        let instructions = match self {
            ParsedType::Boolean => (vec![Instruction::ArgumentGet { index }], 1),
            ParsedType::I8 => (vec![Instruction::ArgumentGet { index }, Instruction::S8FromI32], 1),
            ParsedType::I16 => (vec![Instruction::ArgumentGet { index }, Instruction::S16FromI32], 1),
            ParsedType::I32 => (vec![Instruction::ArgumentGet { index }], 1),
            ParsedType::I64 => (vec![Instruction::ArgumentGet { index }], 1),
            ParsedType::U8 => (vec![Instruction::ArgumentGet { index }, Instruction::U8FromI32], 1),
            ParsedType::U16 => (vec![Instruction::ArgumentGet { index }, Instruction::U16FromI32], 1),
            ParsedType::U32 => (vec![Instruction::ArgumentGet { index }, Instruction::U32FromI32], 1),
            ParsedType::U64 => (vec![Instruction::ArgumentGet { index }, Instruction::U64FromI64], 1),
            ParsedType::F32 => (vec![Instruction::ArgumentGet { index }], 1),
            ParsedType::F64 => (vec![Instruction::ArgumentGet { index }], 1),
            ParsedType::Utf8String => (vec![
                Instruction::ArgumentGet { index },
                Instruction::ArgumentGet { index: index + 1 },
                Instruction::StringLiftMemory,
            ], 2),
            ParsedType::ByteVector => (vec![
                Instruction::ArgumentGet { index },
                Instruction::ArgumentGet { index: index + 1 },
                Instruction::ByteArrayLiftMemory,
            ], 2),
            ParsedType::Record(record_name) => {
                let type_index = wit_resolver.get_record_type_id(record_name)?;

                (vec![
                    Instruction::ArgumentGet { index },
                    Instruction::RecordLiftMemory { type_index },
                ], 1)
            }
        };

        Ok(instructions)
    }

    #[rustfmt::skip]
    fn generate_instructions_for_output_type<'a>(&self, wit_resolver: &mut WITResolver<'a>) -> Result<Vec<Instruction>> {
        let instructions = match self {
            ParsedType::Boolean => vec![],
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
            ParsedType::Record(record_name) => {
                let type_index = wit_resolver.get_record_type_id(record_name)?;

                vec![
                    Instruction::RecordLowerMemory {type_index},
                    Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
                ]
            },
        };

        Ok(instructions)
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
        | ParsedType::U32
        | ParsedType::Record(_) => vec![WasmType::I32],
        ParsedType::I64 | ParsedType::U64 => vec![WasmType::I64],
        ParsedType::F32 => vec![WasmType::F32],
        ParsedType::F64 => vec![WasmType::F64],
        ParsedType::Utf8String | ParsedType::ByteVector => vec![WasmType::I32, WasmType::I32],
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
