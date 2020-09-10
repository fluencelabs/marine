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
use super::utils::ptype_to_itype_checked;
use crate::default_export_api_config::*;
use crate::Result;

use fluence_sdk_wit::AstFunctionItem;
use fluence_sdk_wit::ParsedType;
use wasmer_wit::interpreter::Instruction;
use wasmer_wit::ast::FunctionArg as IFunctionArg;

impl WITGenerator for AstFunctionItem {
    fn generate_wit<'a>(&'a self, wit_resolver: &mut WITResolver<'a>) -> Result<()> {
        use wasmer_wit::ast::Type;
        use wasmer_wit::ast::Adapter;

        let arguments = self
            .signature
            .arguments
            .iter()
            .map(|(arg_name, arg_type)| -> Result<IFunctionArg> {
                Ok(IFunctionArg {
                    name: arg_name.clone(),
                    ty: ptype_to_itype_checked(arg_type, wit_resolver)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let output_types = match self.signature.output_type {
            Some(ref output_type) => vec![ptype_to_itype_checked(output_type, wit_resolver)?],
            None => vec![],
        };

        let interfaces = &mut wit_resolver.interfaces;
        interfaces.types.push(Type::Function {
            arguments: arguments.clone(),
            output_types: output_types.clone(),
        });

        // TODO: replace with Wasm types
        interfaces.types.push(Type::Function {
            arguments,
            output_types,
        });

        let adapter_idx = (interfaces.types.len() - 2) as u32;
        let export_idx = (interfaces.types.len() - 1) as u32;

        interfaces.exports.push(wasmer_wit::ast::Export {
            name: &self.signature.name,
            function_type: export_idx,
        });

        let mut instructions = self
            .signature
            .arguments
            .iter()
            .enumerate()
            .try_fold::<_, _, Result<_>>(
                Vec::new(),
                |mut instructions, (arg_id, (_, input_type))| {
                    let mut new_instructions = input_type
                        .generate_instructions_for_input_type(arg_id as _, wit_resolver)?;

                    instructions.append(&mut new_instructions);
                    Ok(instructions)
                },
            )?;

        let export_function_index = (wit_resolver.interfaces.exports.len() - 1) as u32;
        instructions.push(Instruction::CallCore {
            function_index: export_function_index,
        });

        instructions.extend(match &self.signature.output_type {
            Some(output_type) => output_type.generate_instructions_for_output_type(wit_resolver)?,
            None => vec![],
        });

        let adapter = Adapter {
            function_type: adapter_idx,
            instructions,
        };

        wit_resolver.interfaces.adapters.push(adapter);

        let implementation = wasmer_wit::ast::Implementation {
            core_function_type: export_idx,
            adapter_function_type: adapter_idx,
        };
        wit_resolver.interfaces.implementations.push(implementation);

        Ok(())
    }
}

/// Generate WIT instructions for a function.
trait FnInstructionGenerator {
    fn generate_instructions_for_input_type<'a>(
        &self,
        arg_id: u32,
        wit_resolver: &mut WITResolver<'a>,
    ) -> Result<Vec<Instruction>>;

    fn generate_instructions_for_output_type<'a>(
        &self,
        wit_resolver: &mut WITResolver<'a>,
    ) -> Result<Vec<Instruction>>;
}

impl FnInstructionGenerator for ParsedType {
    #[rustfmt::skip]
    fn generate_instructions_for_input_type<'a>(&self, index: u32, wit_resolver: &mut WITResolver<'a>) -> Result<Vec<Instruction>> {
        let instructions = match self {
            ParsedType::Boolean => vec![Instruction::ArgumentGet { index }],
            ParsedType::I8 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromS8],
            ParsedType::I16 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromS16],
            ParsedType::I32 => vec![Instruction::ArgumentGet { index }],
            ParsedType::I64 => vec![Instruction::ArgumentGet { index }],
            ParsedType::U8 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU8],
            ParsedType::U16 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU16],
            ParsedType::U32 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU32],
            ParsedType::U64 => vec![Instruction::ArgumentGet { index }, Instruction::I64FromU64],
            ParsedType::F32 => vec![Instruction::ArgumentGet { index }],
            ParsedType::F64 => vec![Instruction::ArgumentGet { index }],
            ParsedType::Utf8String => vec![
                Instruction::ArgumentGet { index },
                Instruction::StringSize,
                Instruction::CallCore { function_index: ALLOCATE_FUNC.id },
                Instruction::ArgumentGet { index },
                Instruction::StringLowerMemory,
            ],
            ParsedType::ByteVector => vec![
                Instruction::ArgumentGet { index },
                Instruction::ByteArraySize,
                Instruction::CallCore { function_index: ALLOCATE_FUNC.id },
                Instruction::ArgumentGet { index },
                Instruction::ByteArrayLowerMemory,
            ],
            ParsedType::Record(record_name) => {
                let type_index = wit_resolver.get_record_type_id(record_name)?;

                vec! [
                    Instruction::ArgumentGet { index },
                    Instruction::RecordLowerMemory { type_index },
                ]
            },
        };

        Ok(instructions)
    }

    #[rustfmt::skip]
    fn generate_instructions_for_output_type<'a>(&self, wit_resolver: &mut WITResolver<'a>) -> Result<Vec<Instruction>> {
        let instructions = match self {
            ParsedType::Boolean => vec![],
            ParsedType::I8 => vec![Instruction::S8FromI32],
            ParsedType::I16 => vec![Instruction::S16FromI32],
            ParsedType::I32 => vec![],
            ParsedType::I64 => vec![],
            ParsedType::U8 => vec![Instruction::U8FromI32],
            ParsedType::U16 => vec![Instruction::U16FromI32],
            ParsedType::U32 => vec![Instruction::U32FromI32],
            ParsedType::U64 => vec![Instruction::U64FromI64],
            ParsedType::F32 => vec![],
            ParsedType::F64 => vec![],
            ParsedType::Utf8String => vec![
                Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                Instruction::StringLiftMemory,
                Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                Instruction::CallCore { function_index: DEALLOCATE_FUNC.id },
            ],
            ParsedType::ByteVector => vec![
                Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                Instruction::ByteArrayLiftMemory,
                Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                Instruction::CallCore { function_index: DEALLOCATE_FUNC.id },
            ],
            ParsedType::Record(record_name) => {
                let type_index = wit_resolver.get_record_type_id(record_name)?;

                vec! [
                    Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                    Instruction::RecordLiftMemory { type_index },
                ]
            },
        };

        Ok(instructions)
    }
}
