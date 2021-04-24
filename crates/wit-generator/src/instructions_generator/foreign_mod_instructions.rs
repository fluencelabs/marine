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
use crate::Result;
use crate::default_export_api_config::*;
use crate::instructions_generator::utils::wtype_to_itype;

use fluence_sdk_wit::ExternModType;
use fluence_sdk_wit::ExternFnType;
use fluence_sdk_wit::ParsedType;
use fluence_sdk_wit::FnArgument;
use wasmer_wit::ast::FunctionArg as IFunctionArg;
use wasmer_wit::interpreter::Instruction;
use wasmer_wit::IType;

use std::rc::Rc;

const HOST_NAMESPACE_NAME: &str = "host";

impl WITGenerator for ExternModType {
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
    import: &'a ExternFnType,
    namespace: &'a str,
    wit_resolver: &mut WITResolver<'a>,
) -> Result<()> {
    use wasmer_wit::ast::Type;
    use wasmer_wit::ast::Adapter;

    let arguments = import
        .signature
        .arguments
        .iter()
        .map(|arg| -> Result<IFunctionArg> {
            Ok(IFunctionArg {
                name: arg.name.clone(),
                ty: ptype_to_itype_checked(&arg.ty, wit_resolver)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let arguments = Rc::new(arguments);

    let output_types = import
        .signature
        .output_types
        .iter()
        .map(|ty| ptype_to_itype_checked(ty, wit_resolver))
        .collect::<Result<Vec<_>>>()?;
    let output_types = Rc::new(output_types);

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
    let raw_inputs = Rc::new(raw_inputs);

    let raw_outputs = import
        .signature
        .output_types
        .iter()
        .map(|ty| {
            to_raw_output_type(ty)
                .iter()
                .map(wtype_to_itype)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let raw_outputs = Rc::new(raw_outputs);

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
        .try_fold::<_, _, Result<_>>((0, Vec::new()), |(arg_id, mut instructions), arg| {
            let (new_instructions, shift) = arg
                .ty
                .generate_instructions_for_input_type(arg_id as _, wit_resolver)?;

            instructions.extend(new_instructions);
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

    let instructions = import
        .signature
        .output_types
        .iter()
        .try_fold::<_, _, Result<_>>(instructions, |mut instructions, ty| {
            let new_instructions = ty.generate_instructions_for_output_type(wit_resolver)?;

            instructions.extend(new_instructions);
            Ok(instructions)
        })?;

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
            ParsedType::Boolean(_) => (vec![Instruction::ArgumentGet { index }, Instruction::BoolFromI32], 1),
            ParsedType::I8(_) => (vec![Instruction::ArgumentGet { index }, Instruction::S8FromI32], 1),
            ParsedType::I16(_) => (vec![Instruction::ArgumentGet { index }, Instruction::S16FromI32], 1),
            ParsedType::I32(_) => (vec![Instruction::ArgumentGet { index }, Instruction::S32FromI32], 1),
            ParsedType::I64(_) => (vec![Instruction::ArgumentGet { index }, Instruction::S64FromI64], 1),
            ParsedType::U8(_) => (vec![Instruction::ArgumentGet { index }, Instruction::U8FromI32], 1),
            ParsedType::U16(_) => (vec![Instruction::ArgumentGet { index }, Instruction::U16FromI32], 1),
            ParsedType::U32(_) => (vec![Instruction::ArgumentGet { index }, Instruction::U32FromI32], 1),
            ParsedType::U64(_) => (vec![Instruction::ArgumentGet { index }, Instruction::U64FromI64], 1),
            ParsedType::F32(_) => (vec![Instruction::ArgumentGet { index }], 1),
            ParsedType::F64(_) => (vec![Instruction::ArgumentGet { index }], 1),
            ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => (vec![
                Instruction::ArgumentGet { index },
                Instruction::ArgumentGet { index: index + 1 },
                Instruction::StringLiftMemory,
            ], 2),
            ParsedType::Vector(value_type, _) => {
                let value_type = ptype_to_itype_checked(value_type, wit_resolver)?;

                (vec![
                    Instruction::ArgumentGet { index },
                    Instruction::ArgumentGet { index: index + 1 },
                    Instruction::ArrayLiftMemory { value_type },
                ], 2)
            },
            ParsedType::Record(record_name, _) => {
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
            ParsedType::Boolean(_) => vec![Instruction::I32FromBool],
            ParsedType::I8(_) => vec![Instruction::I32FromS8],
            ParsedType::I16(_) => vec![Instruction::I32FromS16],
            ParsedType::I32(_) => vec![Instruction::I32FromS32],
            ParsedType::I64(_) => vec![Instruction::I64FromS64],
            ParsedType::U8(_) => vec![Instruction::I32FromU8],
            ParsedType::U16(_) => vec![Instruction::I32FromU16],
            ParsedType::U32(_) => vec![Instruction::I32FromU32],
            ParsedType::U64(_) => vec![Instruction::I64FromU64],
            ParsedType::F32(_) => vec![],
            ParsedType::F64(_) => vec![],
            ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => {
                let type_tag = it_lilo::utils::ser_type_size(&IType::U8) as i32;

                vec![
                    Instruction::Dup,
                    Instruction::StringSize,
                    Instruction::PushI32 { value: type_tag },
                    Instruction::CallCore { function_index: ALLOCATE_FUNC.id },
                    Instruction::Swap2,
                    Instruction::StringLowerMemory,
                    Instruction::CallCore { function_index: SET_RESULT_SIZE_FUNC.id },
                    Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
                ]
            },
            ParsedType::Vector(value_type, _) => {
                let value_type = ptype_to_itype_checked(value_type, wit_resolver)?;

                vec![
                    Instruction::ArrayLowerMemory { value_type },
                    Instruction::CallCore { function_index: SET_RESULT_SIZE_FUNC.id },
                    Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
                ]
            },
            ParsedType::Record(record_name, _) => {
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

pub fn to_raw_input_types(arg: &FnArgument) -> Vec<IFunctionArg> {
    match arg.ty {
        ParsedType::Boolean(_)
        | ParsedType::I8(_)
        | ParsedType::I16(_)
        | ParsedType::I32(_)
        | ParsedType::U8(_)
        | ParsedType::U16(_)
        | ParsedType::U32(_)
        | ParsedType::Record(..) => vec![IFunctionArg {
            name: arg.name.clone(),
            ty: IType::I32,
        }],
        ParsedType::I64(_) | ParsedType::U64(_) => vec![IFunctionArg {
            name: arg.name.clone(),
            ty: IType::I64,
        }],
        ParsedType::F32(_) => vec![IFunctionArg {
            name: arg.name.clone(),
            ty: IType::F32,
        }],
        ParsedType::F64(_) => vec![IFunctionArg {
            name: arg.name.clone(),
            ty: IType::F64,
        }],
        ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) | ParsedType::Vector(..) => vec![
            IFunctionArg {
                name: format!("{}_ptr", arg.name),
                ty: IType::I32,
            },
            IFunctionArg {
                name: format!("{}_ptr", arg.name),
                ty: IType::I32,
            },
        ],
    }
}

pub fn to_raw_output_type(ty: &ParsedType) -> Vec<RustType> {
    match ty {
        ParsedType::Boolean(_)
        | ParsedType::I8(_)
        | ParsedType::I16(_)
        | ParsedType::I32(_)
        | ParsedType::U8(_)
        | ParsedType::U16(_)
        | ParsedType::U32(_) => vec![RustType::I32],
        ParsedType::I64(_) | ParsedType::U64(_) => vec![RustType::I64],
        ParsedType::F32(_) => vec![RustType::F32],
        ParsedType::F64(_) => vec![RustType::F64],
        ParsedType::Utf8Str(_)
        | ParsedType::Utf8String(_)
        | ParsedType::Vector(..)
        | ParsedType::Record(..) => vec![],
    }
}
