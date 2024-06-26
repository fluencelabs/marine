/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
use marine_macro_impl::FnSignature;
use wasmer_it::ast::FunctionArg as IFunctionArg;
use wasmer_it::interpreter::Instruction;
use wasmer_it::IType;

use std::sync::Arc;

// TODO: create a common place for these consts to use in both marine and marine-rs-sdk to use in both marine and marine-rs-sdk
const HOST_NAMESPACE_V0: &str = "host";
const HOST_NAMESPACE_PREFIX: &str = "__marine_host_api_v";

impl ITGenerator for ExternModType {
    fn generate_it<'a>(&'a self, it_resolver: &mut ITResolver<'a>) -> Result<()> {
        // host imports should be left as is
        if is_host_import(&self.namespace) {
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
    let arguments = generate_it_args(&fn_type.signature, it_resolver)?;
    let output_types = generate_it_output_type(&fn_type.signature, it_resolver)?;
    it_resolver.add_fn_type(arguments, output_types);

    let raw_arguments = generate_raw_args(&fn_type.signature);
    let raw_output_types = generate_raw_output_type(&fn_type.signature);
    it_resolver.add_fn_type(raw_arguments.clone(), raw_output_types.clone());
    it_resolver.add_fn_type(raw_arguments, raw_output_types);

    let types_count = it_resolver.interfaces.types.len() as u32;
    let import_idx = types_count - 3;
    let raw_import_idx = types_count - 1;

    let link_name = match &fn_type.link_name {
        Some(link_name) => link_name,
        None => &fn_type.signature.name,
    };

    it_resolver.add_import(namespace, link_name, import_idx);
    it_resolver.add_import(namespace, link_name, raw_import_idx);

    Ok(())
}

fn generate_it_instructions<'f>(
    fn_type: &'f ExternFnType,
    it_resolver: &mut ITResolver<'f>,
) -> Result<()> {
    use args_it_generator::ArgumentITGenerator;
    use output_type_it_generator::OutputITGenerator;

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

    it_resolver.add_adapter(adapter_idx, instructions);
    it_resolver.add_implementation(raw_import_idx, adapter_idx);

    Ok(())
}

pub(crate) fn generate_raw_args(signature: &FnSignature) -> Arc<Vec<IFunctionArg>> {
    let raw_inputs = signature
        .arguments
        .iter()
        .flat_map(to_raw_input_types)
        .collect::<Vec<_>>();

    Arc::new(raw_inputs)
}

pub(crate) fn generate_raw_output_type(signature: &FnSignature) -> Arc<Vec<IType>> {
    let raw_outputs = signature
        .output_types
        .iter()
        .flat_map(|ty| {
            to_raw_output_type(ty)
                .iter()
                .map(wtype_to_itype)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Arc::new(raw_outputs)
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

fn is_host_import(namespace: &str) -> bool {
    namespace == HOST_NAMESPACE_V0 || namespace.starts_with(HOST_NAMESPACE_PREFIX)
}
