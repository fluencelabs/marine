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

use super::ITResolver;
use super::ptype_to_itype_checked;
use crate::default_export_api_config::*;
use crate::Result;

use marine_macro_impl::ParsedType;
use wasmer_it::interpreter::Instruction;
use wasmer_it::IType;

/// Generates IT instructions for a argument of an export function.
pub(super) trait ArgumentITGenerator {
    fn generate_instructions_for_arg(
        &self,
        arg_id: u32,
        it_resolver: &mut ITResolver<'_>,
    ) -> Result<Vec<Instruction>>;
}

impl ArgumentITGenerator for ParsedType {
    #[rustfmt::skip]
    fn generate_instructions_for_arg(&self, index: u32, it_resolver: &mut ITResolver<'_>) -> Result<Vec<Instruction>> {
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
}
