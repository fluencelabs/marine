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
use crate::default_export_api_config::RELEASE_OBJECTS;
use crate::instructions_generator::ITResolver;

use marine_macro_impl::*;
use wasmer_it::interpreter::Instruction;
use wasmer_it::IType;

fn generate_export_fn(args: Vec<ParsedType>, output: Option<ParsedType>) -> FnType {
    let name = String::from("some_fn_name");

    let arguments = args
        .into_iter()
        .map(|ty| FnArgument {
            name: String::from("arg_name"),
            ty,
        })
        .collect::<Vec<_>>();

    let output_types = match output {
        Some(output) => vec![output],
        None => vec![],
    };

    let signature = FnSignature {
        name,
        arguments,
        output_types,
    };

    FnType { signature }
}

fn generate_import_mod(args: Vec<ParsedType>, output: Option<ParsedType>) -> ExternModType {
    let name = String::from("some_fn_name");

    let arguments = args
        .into_iter()
        .map(|ty| FnArgument {
            name: String::from("arg_name"),
            ty,
        })
        .collect::<Vec<_>>();

    let output_types = match output {
        Some(output) => vec![output],
        None => vec![],
    };

    let signature = FnSignature {
        name,
        arguments,
        output_types,
    };

    let extern_fn_type = ExternFnType {
        link_name: None,
        signature,
    };

    ExternModType {
        namespace: String::from("some namespace"),
        imports: vec![extern_fn_type],
    }
}

#[test]
fn simple_arg_types_in_export() {
    let args = vec![
        ParsedType::I8(PassingStyle::ByValue),
        ParsedType::I16(PassingStyle::ByValue),
        ParsedType::I32(PassingStyle::ByValue),
        ParsedType::I64(PassingStyle::ByValue),
        ParsedType::U8(PassingStyle::ByValue),
        ParsedType::U16(PassingStyle::ByValue),
        ParsedType::U32(PassingStyle::ByValue),
        ParsedType::U64(PassingStyle::ByValue),
        ParsedType::F32(PassingStyle::ByValue),
        ParsedType::F64(PassingStyle::ByValue),
    ];

    let outputs = Some(ParsedType::I32(PassingStyle::ByValue));
    let fn_type = generate_export_fn(args, outputs);

    let mut it_resolver = ITResolver::default();
    fn_type
        .generate_it(&mut it_resolver)
        .expect("IT generation succeeded");

    let interfaces = it_resolver.interfaces;

    let actual_instruction = &interfaces.adapters[0].instructions;

    let expected_instruction = vec![
        Instruction::ArgumentGet { index: 0 },
        Instruction::I32FromS8,
        Instruction::ArgumentGet { index: 1 },
        Instruction::I32FromS16,
        Instruction::ArgumentGet { index: 2 },
        Instruction::I32FromS32,
        Instruction::ArgumentGet { index: 3 },
        Instruction::I64FromS64,
        Instruction::ArgumentGet { index: 4 },
        Instruction::I32FromU8,
        Instruction::ArgumentGet { index: 5 },
        Instruction::I32FromU16,
        Instruction::ArgumentGet { index: 6 },
        Instruction::I32FromU32,
        Instruction::ArgumentGet { index: 7 },
        Instruction::I64FromU64,
        Instruction::ArgumentGet { index: 8 },
        Instruction::ArgumentGet { index: 9 },
        Instruction::CallCore { function_index: 0 },
        Instruction::S32FromI32,
    ];

    assert_eq!(actual_instruction, &expected_instruction);
}

#[test]
fn complex_arg_types_in_export() {
    let args = vec![
        ParsedType::I8(PassingStyle::ByValue),
        ParsedType::I16(PassingStyle::ByValue),
        ParsedType::I32(PassingStyle::ByValue),
        ParsedType::I64(PassingStyle::ByValue),
        ParsedType::U8(PassingStyle::ByValue),
        ParsedType::U16(PassingStyle::ByValue),
        ParsedType::U32(PassingStyle::ByValue),
        ParsedType::U64(PassingStyle::ByValue),
        ParsedType::F32(PassingStyle::ByValue),
        ParsedType::F64(PassingStyle::ByValue),
        ParsedType::Utf8String(PassingStyle::ByValue),
        ParsedType::Vector(
            Box::new(ParsedType::U8(PassingStyle::ByValue)),
            PassingStyle::ByValue,
        ),
    ];

    let outputs = Some(ParsedType::Utf8String(PassingStyle::ByValue));
    let fn_type = generate_export_fn(args, outputs);

    let mut it_resolver = ITResolver::default();
    fn_type
        .generate_it(&mut it_resolver)
        .expect("IT generation succeeded");

    let interfaces = it_resolver.interfaces;

    let actual_instruction = &interfaces.adapters[0].instructions;

    let expected_instruction = vec![
        Instruction::ArgumentGet { index: 0 },
        Instruction::I32FromS8,
        Instruction::ArgumentGet { index: 1 },
        Instruction::I32FromS16,
        Instruction::ArgumentGet { index: 2 },
        Instruction::I32FromS32,
        Instruction::ArgumentGet { index: 3 },
        Instruction::I64FromS64,
        Instruction::ArgumentGet { index: 4 },
        Instruction::I32FromU8,
        Instruction::ArgumentGet { index: 5 },
        Instruction::I32FromU16,
        Instruction::ArgumentGet { index: 6 },
        Instruction::I32FromU32,
        Instruction::ArgumentGet { index: 7 },
        Instruction::I64FromU64,
        Instruction::ArgumentGet { index: 8 },
        Instruction::ArgumentGet { index: 9 },
        Instruction::ArgumentGet { index: 10 },
        Instruction::StringSize,
        Instruction::PushI32 { value: 1 },
        Instruction::CallCore { function_index: 0 },
        Instruction::ArgumentGet { index: 10 },
        Instruction::StringLowerMemory,
        Instruction::ArgumentGet { index: 11 },
        Instruction::ArrayLowerMemory {
            value_type: IType::U8,
        },
        Instruction::CallCore { function_index: 0 },
        Instruction::CallCore { function_index: 3 },
        Instruction::CallCore { function_index: 2 },
        Instruction::StringLiftMemory,
        Instruction::CallCore {
            function_index: RELEASE_OBJECTS.id,
        },
    ];

    assert_eq!(actual_instruction, &expected_instruction);
}

#[test]
fn simple_arg_types_in_import() {
    let args = vec![
        ParsedType::I8(PassingStyle::ByValue),
        ParsedType::I16(PassingStyle::ByValue),
        ParsedType::I32(PassingStyle::ByValue),
        ParsedType::I64(PassingStyle::ByValue),
        ParsedType::U8(PassingStyle::ByValue),
        ParsedType::U16(PassingStyle::ByValue),
        ParsedType::U32(PassingStyle::ByValue),
        ParsedType::U64(PassingStyle::ByValue),
        ParsedType::F32(PassingStyle::ByValue),
        ParsedType::F64(PassingStyle::ByValue),
    ];

    let outputs = Some(ParsedType::I32(PassingStyle::ByValue));
    let import_fn_type = generate_import_mod(args, outputs);

    let mut it_resolver = ITResolver::default();
    import_fn_type
        .generate_it(&mut it_resolver)
        .expect("IT generation succeeded");

    let interfaces = it_resolver.interfaces;

    let actual_instruction = &interfaces.adapters[0].instructions;

    let expected_instruction = vec![
        Instruction::ArgumentGet { index: 0 },
        Instruction::S8FromI32,
        Instruction::ArgumentGet { index: 1 },
        Instruction::S16FromI32,
        Instruction::ArgumentGet { index: 2 },
        Instruction::S32FromI32,
        Instruction::ArgumentGet { index: 3 },
        Instruction::S64FromI64,
        Instruction::ArgumentGet { index: 4 },
        Instruction::U8FromI32,
        Instruction::ArgumentGet { index: 5 },
        Instruction::U16FromI32,
        Instruction::ArgumentGet { index: 6 },
        Instruction::U32FromI32,
        Instruction::ArgumentGet { index: 7 },
        Instruction::U64FromI64,
        Instruction::ArgumentGet { index: 8 },
        Instruction::ArgumentGet { index: 9 },
        Instruction::CallCore { function_index: 0 },
        Instruction::I32FromS32,
    ];

    assert_eq!(actual_instruction, &expected_instruction);
}

#[test]
fn complex_arg_types_in_import() {
    let args = vec![
        ParsedType::I8(PassingStyle::ByValue),
        ParsedType::I16(PassingStyle::ByValue),
        ParsedType::I32(PassingStyle::ByValue),
        ParsedType::I64(PassingStyle::ByValue),
        ParsedType::U8(PassingStyle::ByValue),
        ParsedType::U16(PassingStyle::ByValue),
        ParsedType::U32(PassingStyle::ByValue),
        ParsedType::U64(PassingStyle::ByValue),
        ParsedType::F32(PassingStyle::ByValue),
        ParsedType::F64(PassingStyle::ByValue),
        ParsedType::Utf8String(PassingStyle::ByValue),
        ParsedType::Vector(
            Box::new(ParsedType::U8(PassingStyle::ByValue)),
            PassingStyle::ByValue,
        ),
    ];

    let outputs = Some(ParsedType::Utf8String(PassingStyle::ByValue));
    let fn_type = generate_import_mod(args, outputs);

    let mut it_resolver = ITResolver::default();
    fn_type
        .generate_it(&mut it_resolver)
        .expect("IT generation succeeded");

    let interfaces = it_resolver.interfaces;

    let actual_instruction = &interfaces.adapters[0].instructions;

    let expected_instruction = vec![
        Instruction::ArgumentGet { index: 0 },
        Instruction::S8FromI32,
        Instruction::ArgumentGet { index: 1 },
        Instruction::S16FromI32,
        Instruction::ArgumentGet { index: 2 },
        Instruction::S32FromI32,
        Instruction::ArgumentGet { index: 3 },
        Instruction::S64FromI64,
        Instruction::ArgumentGet { index: 4 },
        Instruction::U8FromI32,
        Instruction::ArgumentGet { index: 5 },
        Instruction::U16FromI32,
        Instruction::ArgumentGet { index: 6 },
        Instruction::U32FromI32,
        Instruction::ArgumentGet { index: 7 },
        Instruction::U64FromI64,
        Instruction::ArgumentGet { index: 8 },
        Instruction::ArgumentGet { index: 9 },
        Instruction::ArgumentGet { index: 10 },
        Instruction::ArgumentGet { index: 11 },
        Instruction::StringLiftMemory,
        Instruction::ArgumentGet { index: 12 },
        Instruction::ArgumentGet { index: 13 },
        Instruction::ByteArrayLiftMemory,
        Instruction::CallCore {
            function_index: RELEASE_OBJECTS.id,
        },
        Instruction::CallCore { function_index: 0 },
        Instruction::Dup,
        Instruction::StringSize,
        Instruction::PushI32 { value: 1 },
        Instruction::CallCore { function_index: 0 },
        Instruction::Swap2,
        Instruction::StringLowerMemory,
        Instruction::CallCore { function_index: 4 },
        Instruction::CallCore { function_index: 5 },
    ];

    assert_eq!(actual_instruction, &expected_instruction);
}
