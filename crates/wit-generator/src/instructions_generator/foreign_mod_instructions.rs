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
use crate::*;
use crate::default_export_api_config::*;

use fluence_sdk_wit::AstExternModItem;
use fluence_sdk_wit::AstExternFnItem;
use fluence_sdk_wit::ParsedType;
use wasmer_wit::interpreter::Instruction;

const HOST_NAMESPACE_NAME: &str = "host";

impl WITGenerator for AstExternModItem {
    fn generate_wit<'ast_type, 'resolver>(
        &'ast_type self,
        wit_resolver: &'resolver mut WITResolver<'ast_type>,
    ) -> Result<()> {
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
    let arguments = &import.signature.arguments;
    let output_type = &import.signature.output_type;

    let function_type_id = add_function_type(arguments, output_type, wit_resolver)?;

    let link_name = match &import.link_name {
        Some(link_name) => link_name,
        None => &import.signature.name,
    };
    let import_type = AstImport {
        name: link_name,
        namespace,
        function_type: function_type_id,
    };
    let import_function_id = wit_resolver.insert_import_type(import_type);

    let adapter_instructions = generate_import_adapter_instructions(
        arguments,
        output_type,
        wit_resolver,
        import_function_id,
    )?;

    let adapter = crate::AstAdapter {
        function_type: function_type_id,
        instructions: adapter_instructions,
    };
    let adapter_id = wit_resolver.insert_adapter(adapter);

    let implementation = crate::AstImplementation {
        core_function_id: import_function_id,
        adapter_function_id: adapter_id,
    };
    wit_resolver.insert_implementation(implementation);

    Ok(())
}

fn generate_import_adapter_instructions(
    arguments: &[(String, ParsedType)],
    output_type: &Option<ParsedType>,
    wit_resolver: &mut WITResolver<'_>,
    import_function_id: u32,
) -> Result<Vec<Instruction>> {
    let mut instructions = arguments
        .iter()
        .try_fold::<_, _, Result<_>>(
            (0, Vec::new()),
            |(arg_id, mut instructions), (_, input_type)| {
                let (mut new_instructions, shift) =
                    input_type.generate_instructions_for_input_type(arg_id as _, wit_resolver)?;

                instructions.append(&mut new_instructions);
                Ok((arg_id + shift, instructions))
            },
        )?
        .1;

    instructions.push(Instruction::CallCore {
        function_index: import_function_id,
    });

    instructions.extend(match output_type {
        Some(output_type) => output_type.generate_instructions_for_output_type(wit_resolver)?,
        None => vec![],
    });

    Ok(instructions)
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
            ParsedType::I32 => (vec![Instruction::ArgumentGet { index }, Instruction::S32FromI32], 1),
            ParsedType::I64 => (vec![Instruction::ArgumentGet { index }, Instruction::S64FromI64], 1),
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
            ParsedType::Vector(value_type) => {
                let value_type = ptype_to_itype_checked(value_type, wit_resolver)?;

                (vec![
                    Instruction::ArgumentGet { index },
                    Instruction::ArgumentGet { index: index + 1 },
                    Instruction::ArrayLiftMemory { value_type },
                ], 2)
            },
            ParsedType::Record(record_name) => {
                let record_type_id = wit_resolver.get_record_type_id(record_name)? as u32;

                (vec![
                    Instruction::ArgumentGet { index },
                    Instruction::RecordLiftMemory { record_type_id },
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
            ParsedType::I32 => vec![Instruction::I32FromS32],
            ParsedType::I64 => vec![Instruction::I64FromS64],
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
            ParsedType::Vector(value_type) => {
                let value_type = ptype_to_itype_checked(value_type, wit_resolver)?;

                vec![
                    Instruction::ArrayLowerMemory { value_type },
                    Instruction::CallCore { function_index: SET_RESULT_SIZE_FUNC.id },
                    Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
                ]
            },
            ParsedType::Record(record_name) => {
                let record_type_id = wit_resolver.get_record_type_id(record_name)? as u32;

                vec![
                    Instruction::RecordLowerMemory { record_type_id },
                    Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
                ]
            },
        };

        Ok(instructions)
    }
}
