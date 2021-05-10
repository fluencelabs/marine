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

use super::ITGenerator;
use super::ITResolver;
use super::utils::ptype_to_itype_checked;
use crate::default_export_api_config::*;
use crate::Result;

use marine_macro_impl::FnType;
use marine_macro_impl::ParsedType;
use wasmer_it::interpreter::Instruction;
use wasmer_it::ast::FunctionArg as IFunctionArg;
use wasmer_it::IType;

use std::rc::Rc;

impl ITGenerator for FnType {
    fn generate_it<'a>(&'a self, it_resolver: &mut ITResolver<'a>) -> Result<()> {
        use wasmer_it::ast::Type;
        use wasmer_it::ast::Adapter;

        let arguments = self
            .signature
            .arguments
            .iter()
            .map(|arg| -> Result<IFunctionArg> {
                Ok(IFunctionArg {
                    name: arg.name.clone(),
                    ty: ptype_to_itype_checked(&arg.ty, it_resolver)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let arguments = Rc::new(arguments);

        let output_types = self
            .signature
            .output_types
            .iter()
            .map(|ty| ptype_to_itype_checked(ty, it_resolver))
            .collect::<Result<Vec<_>>>()?;
        let output_types = Rc::new(output_types);

        let interfaces = &mut it_resolver.interfaces;
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

        interfaces.exports.push(wasmer_it::ast::Export {
            name: &self.signature.name,
            function_type: export_idx,
        });

        let mut instructions = self
            .signature
            .arguments
            .iter()
            .enumerate()
            .try_fold::<_, _, Result<_>>(Vec::new(), |mut instructions, (arg_id, arg)| {
                let new_instructions = arg
                    .ty
                    .generate_instructions_for_input_type(arg_id as _, it_resolver)?;

                instructions.extend(new_instructions);
                Ok(instructions)
            })?;

        let export_function_index = (it_resolver.interfaces.exports.len() - 1) as u32;
        instructions.push(Instruction::CallCore {
            function_index: export_function_index,
        });

        let instructions = self
            .signature
            .output_types
            .iter()
            .try_fold::<_, _, Result<_>>(instructions, |mut instructions, ty| {
                let new_instructions = ty.generate_instructions_for_output_type(it_resolver)?;

                instructions.extend(new_instructions);
                Ok(instructions)
            })?;

        let adapter = Adapter {
            function_type: adapter_idx,
            instructions,
        };

        it_resolver.interfaces.adapters.push(adapter);

        let implementation = wasmer_it::ast::Implementation {
            core_function_type: export_idx,
            adapter_function_type: adapter_idx,
        };
        it_resolver.interfaces.implementations.push(implementation);

        Ok(())
    }
}

/// Generate IT instructions for a function.
trait FnInstructionGenerator {
    fn generate_instructions_for_input_type<'a>(
        &self,
        arg_id: u32,
        it_resolver: &mut ITResolver<'a>,
    ) -> Result<Vec<Instruction>>;

    fn generate_instructions_for_output_type<'a>(
        &self,
        it_resolver: &mut ITResolver<'a>,
    ) -> Result<Vec<Instruction>>;
}

impl FnInstructionGenerator for ParsedType {
    #[rustfmt::skip]
    fn generate_instructions_for_input_type<'a>(&self, index: u32, it_resolver: &mut ITResolver<'a>) -> Result<Vec<Instruction>> {
        let instructions = match self {
            ParsedType::Boolean(_) => vec![Instruction::ArgumentGet { index }, Instruction::I32FromBool],
            ParsedType::I8(_) => vec![Instruction::ArgumentGet { index }, Instruction::I32FromS8],
            ParsedType::I16(_) => vec![Instruction::ArgumentGet { index }, Instruction::I32FromS16],
            ParsedType::I32(_) => vec![Instruction::ArgumentGet { index }, Instruction::I32FromS32],
            ParsedType::I64(_) => vec![Instruction::ArgumentGet { index }, Instruction::I64FromS64],
            ParsedType::U8(_) => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU8],
            ParsedType::U16(_) => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU16],
            ParsedType::U32(_) => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU32],
            ParsedType::U64(_) => vec![Instruction::ArgumentGet { index }, Instruction::I64FromU64],
            ParsedType::F32(_) => vec![Instruction::ArgumentGet { index }],
            ParsedType::F64(_) => vec![Instruction::ArgumentGet { index }],
            ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => {
                let type_tag = it_lilo::utils::ser_type_size(&IType::U8) as i32;
                vec![
                    Instruction::ArgumentGet { index },
                    Instruction::StringSize,
                    Instruction::PushI32 { value: type_tag },
                    Instruction::CallCore { function_index: ALLOCATE_FUNC.id },
                    Instruction::ArgumentGet { index },
                    Instruction::StringLowerMemory,
                ]
            },
            ParsedType::Vector(value_type, _) => {
                let value_type = ptype_to_itype_checked(value_type, it_resolver)?;
                vec![
                    Instruction::ArgumentGet { index },
                    Instruction::ArrayLowerMemory {
                        value_type
                    },
                ]
            },
            ParsedType::Record(record_name, _) => {
                let record_type_id = it_resolver.get_record_type_id(record_name)? as u32;

                vec! [
                    Instruction::ArgumentGet { index },
                    Instruction::RecordLowerMemory { record_type_id },
                ]
            },
        };

        Ok(instructions)
    }

    #[rustfmt::skip]
    fn generate_instructions_for_output_type<'a>(&self, it_resolver: &mut ITResolver<'a>) -> Result<Vec<Instruction>> {
        let instructions = match self {
            ParsedType::Boolean(_) => vec![Instruction::BoolFromI32],
            ParsedType::I8(_) => vec![Instruction::S8FromI32],
            ParsedType::I16(_) => vec![Instruction::S16FromI32],
            ParsedType::I32(_) => vec![Instruction::S32FromI32],
            ParsedType::I64(_) => vec![Instruction::S64FromI64],
            ParsedType::U8(_) => vec![Instruction::U8FromI32],
            ParsedType::U16(_) => vec![Instruction::U16FromI32],
            ParsedType::U32(_) => vec![Instruction::U32FromI32],
            ParsedType::U64(_) => vec![Instruction::U64FromI64],
            ParsedType::F32(_) => vec![],
            ParsedType::F64(_) => vec![],
            ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => vec![
                Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                Instruction::StringLiftMemory,
                Instruction::CallCore { function_index: RELEASE_OBJECTS.id },
            ],
            ParsedType::Vector(value_type, _) => {
                let value_type = ptype_to_itype_checked(value_type, it_resolver)?;
                if let IType::U8 = value_type {
                   vec![
                       Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                       Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                       Instruction::ByteArrayLiftMemory,
                       Instruction::CallCore { function_index: RELEASE_OBJECTS.id },
                   ]
                } else {
                    vec![
                        Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                        Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                        Instruction::ArrayLiftMemory { value_type },
                        Instruction::CallCore { function_index: RELEASE_OBJECTS.id },
                    ]
                }
            },
            ParsedType::Record(record_name, _) => {
                let record_type_id = it_resolver.get_record_type_id(record_name)? as u32;

                vec! [
                    Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                    Instruction::RecordLiftMemory { record_type_id },
                    Instruction::CallCore { function_index: RELEASE_OBJECTS.id },
                ]
            },
        };

        Ok(instructions)
    }
}
