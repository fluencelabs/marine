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

use super::add_function_type;
use super::WITGenerator;
use super::WITResolver;
use super::utils::ptype_to_itype_checked;
use crate::default_export_api_config::*;
use crate::AstExport;
use crate::Result;

use fluence_sdk_wit::AstFunctionItem;
use fluence_sdk_wit::ParsedType;
use wasmer_wit::interpreter::Instruction;

impl WITGenerator for AstFunctionItem {
    fn generate_wit<'ast_type, 'resolver>(&'ast_type self, wit_resolver: &'resolver mut WITResolver<'ast_type>) -> Result<()> {
        let arguments = &self.signature.arguments;
        let output_type = &self.signature.output_type;

        let function_type_id = add_function_type(arguments, output_type, wit_resolver)?;
        let export_type = AstExport {
            name: &self.signature.name,
            function_type: function_type_id,
        };
        let export_function_id = wit_resolver.insert_export_type(export_type);

        let adapter_instructions = generate_export_adapter_instructions(
            arguments,
            output_type,
            wit_resolver,
            export_function_id,
        )?;

        let adapter = crate::AstAdapter {
            function_type: function_type_id,
            instructions: adapter_instructions,
        };
        let adapter_id = wit_resolver.insert_adapter(adapter);

        let implementation = crate::AstImplementation {
            core_function_id: export_function_id,
            adapter_function_id: adapter_id,
        };
        wit_resolver.insert_implementation(implementation);

        Ok(())
    }
}

fn generate_export_adapter_instructions(
    arguments: &[(String, ParsedType)],
    output_type: &Option<ParsedType>,
    wit_resolver: &mut WITResolver<'_>,
    export_function_id: u32,
) -> Result<Vec<Instruction>> {
    let mut instructions = arguments.iter().enumerate().try_fold::<_, _, Result<_>>(
        Vec::with_capacity(arguments.len()),
        |mut instructions, (arg_id, (_, input_type))| {
            let mut new_instructions =
                input_type.generate_instructions_for_input_type(arg_id as _, wit_resolver)?;

            instructions.append(&mut new_instructions);
            Ok(instructions)
        },
    )?;

    instructions.push(Instruction::CallCore {
        function_index: export_function_id,
    });

    instructions.extend(match output_type {
        Some(output_type) => output_type.generate_instructions_for_output_type(wit_resolver)?,
        None => vec![],
    });

    Ok(instructions)
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
            ParsedType::I32 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromS32],
            ParsedType::I64 => vec![Instruction::ArgumentGet { index }, Instruction::I64FromS64],
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
            ParsedType::Vector(value_type) => {
                let value_type = ptype_to_itype_checked(value_type, wit_resolver)?;
                vec![
                    Instruction::ArgumentGet { index },
                    Instruction::ArrayLowerMemory {
                        value_type
                    },
                ]
            },
            ParsedType::Record(record_name) => {
                let record_type_id = wit_resolver.get_record_type_id(record_name)? as u32;

                vec! [
                    Instruction::ArgumentGet { index },
                    Instruction::RecordLowerMemory { record_type_id },
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
            ParsedType::I32 => vec![Instruction::S32FromI32],
            ParsedType::I64 => vec![Instruction::S64FromI64],
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
            ParsedType::Vector(value_type) => {
                let value_type = ptype_to_itype_checked(value_type, wit_resolver)?;

                vec![
                    Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                    Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                    Instruction::ArrayLiftMemory { value_type },
                ]
            },
            ParsedType::Record(record_name) => {
                let record_type_id = wit_resolver.get_record_type_id(record_name)? as u32;

                vec! [
                    Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                    Instruction::RecordLiftMemory { record_type_id },
                ]
            },
        };

        Ok(instructions)
    }
}
