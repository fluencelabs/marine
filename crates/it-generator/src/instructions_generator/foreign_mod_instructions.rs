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

mod args_it_generator;
mod output_type_it_generator;

use super::ITGenerator;
use super::ITResolver;
use super::utils::*;
use crate::Result;
use crate::default_export_api_config::RELEASE_OBJECTS;
use crate::instructions_generator::utils::wtype_to_itype;

use marine_macro_impl::ExternModType;
use marine_macro_impl::ExternFnType;
use marine_macro_impl::ParsedType;
use marine_macro_impl::FnArgument;
use wasmer_it::ast::FunctionArg as IFunctionArg;
use wasmer_it::interpreter::Instruction;
use wasmer_it::IType;

use std::rc::Rc;

const HOST_NAMESPACE_NAME: &str = "host";

impl ITGenerator for ExternModType {
    fn generate_it<'a>(&'a self, it_resolver: &mut ITResolver<'a>) -> Result<()> {
        // host imports should be left as is
        if self.namespace == HOST_NAMESPACE_NAME {
            return Ok(());
        }

        for import in &self.imports {
            generate_it_for_import(import, &self.namespace, it_resolver)?;
        }

        Ok(())
    }
}

fn generate_it_for_import<'a>(
    fn_type: &'a ExternFnType,
    namespace: &'a str,
    it_resolver: &mut ITResolver<'a>,
) -> Result<()> {
    generate_it_types(fn_type, namespace, it_resolver)?;
    generate_it_instructions(fn_type, it_resolver)
}

fn generate_it_types<'f>(
    fn_type: &'f ExternFnType,
    namespace: &'f str,
    it_resolver: &mut ITResolver<'f>,
) -> Result<()> {
    use wasmer_it::ast::Type;

    let arguments = generate_it_args(&fn_type.signature, it_resolver)?;
    let output_types = generate_it_output_type(&fn_type.signature, it_resolver)?;

    let interfaces = &mut it_resolver.interfaces;
    interfaces.types.push(Type::Function {
        arguments,
        output_types,
    });

    let raw_inputs = fn_type
        .signature
        .arguments
        .iter()
        .map(to_raw_input_types)
        .flatten()
        .collect::<Vec<_>>();
    let raw_inputs = Rc::new(raw_inputs);

    let raw_outputs = fn_type
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

    let import_idx = (interfaces.types.len() - 3) as u32;
    let raw_import_idx = (interfaces.types.len() - 1) as u32;

    let link_name = match &fn_type.link_name {
        Some(link_name) => link_name,
        None => &fn_type.signature.name,
    };

    interfaces.imports.push(wasmer_it::ast::Import {
        namespace: &namespace,
        name: link_name,
        function_type: import_idx,
    });

    interfaces.imports.push(wasmer_it::ast::Import {
        namespace: &namespace,
        name: link_name,
        function_type: raw_import_idx,
    });

    Ok(())
}

fn generate_it_instructions<'f>(
    fn_type: &'f ExternFnType,
    it_resolver: &mut ITResolver<'f>,
) -> Result<()> {
    use args_it_generator::ArgumentITGenerator;
    use output_type_it_generator::OutputITGenerator;
    use wasmer_it::ast::Adapter;

    let adapter_idx = (it_resolver.interfaces.types.len() - 2) as u32;
    let raw_import_idx = (it_resolver.interfaces.types.len() - 1) as u32;

    let mut should_generate_release = false;
    let mut instructions = fn_type
        .signature
        .arguments
        .iter()
        .try_fold::<_, _, Result<_>>((0, Vec::new()), |(arg_id, mut instructions), arg| {
            let (new_instructions, shift) = arg
                .ty
                .generate_instructions_for_arg(arg_id as _, it_resolver)?;

            should_generate_release |= arg.ty.is_complex_type();

            instructions.extend(new_instructions);
            Ok((arg_id + shift, instructions))
        })?
        .1;

    if should_generate_release {
        instructions.push(Instruction::CallCore {
            function_index: RELEASE_OBJECTS.id,
        });
    }

    // TODO: refactor
    let import_function_index = (it_resolver.interfaces.exports.len()
        + it_resolver.interfaces.imports.len() / 2
        - 1) as u32;
    instructions.push(Instruction::CallCore {
        function_index: import_function_index,
    });

    let instructions = fn_type
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
        core_function_type: raw_import_idx,
        adapter_function_type: adapter_idx,
    };
    it_resolver.interfaces.implementations.push(implementation);

    Ok(())
}

use marine_macro_impl::RustType;

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
