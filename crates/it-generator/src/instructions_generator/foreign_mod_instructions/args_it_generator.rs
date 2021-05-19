/*
 * Copyright 2021 Fluence Labs Limited
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

use super::ITResolver;
use super::ptype_to_itype_checked;
use crate::Result;

use marine_macro_impl::ParsedType;
use wasmer_it::interpreter::Instruction;
use wasmer_it::IType;

/// Generate IT instructions for a foreign mod.
pub(super) trait ArgumentITGenerator {
    fn generate_instructions_for_arg<'a>(
        &self,
        arg_id: u32,
        it_resolver: &mut ITResolver<'a>,
    ) -> Result<(Vec<Instruction>, u32)>;
}

#[rustfmt::skip]
impl ArgumentITGenerator for ParsedType {
    fn generate_instructions_for_arg<'a>(
        &self,
        index: u32,
        it_resolver: &mut ITResolver<'a>,
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
                let value_type = ptype_to_itype_checked(value_type, it_resolver)?;
                if let IType::U8 = value_type {
                    (vec![
                        Instruction::ArgumentGet { index },
                        Instruction::ArgumentGet { index: index + 1 },
                        Instruction::ByteArrayLiftMemory,
                    ], 2)
                } else {
                    (vec![
                        Instruction::ArgumentGet { index },
                        Instruction::ArgumentGet { index: index + 1 },
                        Instruction::ArrayLiftMemory { value_type },
                    ], 2)
                }
            },
            ParsedType::Record(record_name, _) => {
                let record_type_id = it_resolver.get_record_type_id(record_name)? as u32;

                (vec![
                    Instruction::ArgumentGet { index },
                    Instruction::RecordLiftMemory { record_type_id },
                ], 1)
            }
        };

        Ok(instructions)
    }
}
