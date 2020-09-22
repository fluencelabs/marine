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

use fluence_sdk_wit::AstExternModItem;
use fluence_sdk_wit::AstExternFnItem;
use fluence_sdk_wit::ParsedType;
use wasmer_wit::ast::FunctionArg as IFunctionArg;
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

    let arguments = import
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

    let output_types = match import.signature.output_type {
        Some(ref output_type) => vec![ptype_to_itype_checked(output_type, wit_resolver)?],
        None => vec![],
    };

    let interfaces = &mut wit_resolver.interfaces;
    interfaces.types.push(Type::Function {
        arguments,
        output_types,
    });

    let raw_inputs = import
        .signature
        .arguments
        .iter()
        .map(to_raw_input_types)
        .flatten()
        .collect::<Vec<_>>();

    let raw_outputs = match import.signature.output_type {
        Some(ref output_type) => to_raw_output_type(output_type)
            .iter()
            .map(wtype_to_itype)
            .collect(),
        None => vec![],
    };

    interfaces.types.push(Type::Function {
        arguments: raw_inputs.clone(),
        output_types: raw_outputs.clone(),
    });

    interfaces.types.push(Type::Function {
        arguments: raw_inputs,
        output_types: raw_outputs,
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
        .arguments
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

use fluence_sdk_wit::RustType;
use wasmer_wit::types::InterfaceType as IType;

pub fn to_raw_input_types(arg: &(String, ParsedType)) -> Vec<IFunctionArg> {
    match arg.1 {
        ParsedType::Boolean
        | ParsedType::I8
        | ParsedType::I16
        | ParsedType::I32
        | ParsedType::U8
        | ParsedType::U16
        | ParsedType::U32
        | ParsedType::Record(_) => vec![IFunctionArg {
            name: arg.0.clone(),
            ty: IType::I32,
        }],
        ParsedType::I64 | ParsedType::U64 => vec![IFunctionArg {
            name: arg.0.clone(),
            ty: IType::I64,
        }],
        ParsedType::F32 => vec![IFunctionArg {
            name: arg.0.clone(),
            ty: IType::F32,
        }],
        ParsedType::F64 => vec![IFunctionArg {
            name: arg.0.clone(),
            ty: IType::F64,
        }],
        ParsedType::Utf8String | ParsedType::Vector(_) => vec![
            IFunctionArg {
                name: format!("{}_ptr", arg.0),
                ty: IType::I32,
            },
            IFunctionArg {
                name: format!("{}_ptr", arg.0),
                ty: IType::I32,
            },
        ],
    }
}

pub fn to_raw_output_type(ty: &ParsedType) -> Vec<RustType> {
    match ty {
        ParsedType::Boolean
        | ParsedType::I8
        | ParsedType::I16
        | ParsedType::I32
        | ParsedType::U8
        | ParsedType::U16
        | ParsedType::U32 => vec![RustType::I32],
        ParsedType::I64 | ParsedType::U64 => vec![RustType::I64],
        ParsedType::F32 => vec![RustType::F32],
        ParsedType::F64 => vec![RustType::F64],
        ParsedType::Utf8String | ParsedType::Vector(_) | ParsedType::Record(_) => vec![],
    }
}
