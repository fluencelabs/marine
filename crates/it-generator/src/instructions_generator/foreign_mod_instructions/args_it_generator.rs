/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use super::ITResolver;
use super::ptype_to_itype_checked;
use crate::Result;

use marine_macro_impl::ParsedType;
use wasmer_it::interpreter::Instruction;
use wasmer_it::IType;

/// Generate IT instructions for a foreign mod.
pub(super) trait ArgumentITGenerator {
    fn generate_instructions_for_arg(
        &self,
        arg_id: u32,
        it_resolver: &mut ITResolver<'_>,
    ) -> Result<(Vec<Instruction>, u32)>;
}

#[rustfmt::skip]
impl ArgumentITGenerator for ParsedType {
    fn generate_instructions_for_arg(
        &self,
        index: u32,
        it_resolver: &mut ITResolver<'_>,
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
