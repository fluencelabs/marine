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
use crate::default_export_api_config::*;
use crate::Result;

use marine_macro_impl::ParsedType;
use wasmer_it::interpreter::Instruction;
use wasmer_it::IType;

/// Generates IT instructions for a output type of an export function.
pub(super) trait OutputITGenerator {
    fn generate_instructions_for_output_type(
        &self,
        it_resolver: &mut ITResolver<'_>,
    ) -> Result<Vec<Instruction>>;
}

impl OutputITGenerator for ParsedType {
    #[rustfmt::skip]
    fn generate_instructions_for_output_type(&self, it_resolver: &mut ITResolver<'_>) -> Result<Vec<Instruction>> {
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
            ],
            ParsedType::Vector(value_type, _) => {
                let value_type = ptype_to_itype_checked(value_type, it_resolver)?;
                if let IType::U8 = value_type {
                    vec![
                        Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                        Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                        Instruction::ByteArrayLiftMemory,
                    ]
                } else {
                    vec![
                        Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                        Instruction::CallCore { function_index: GET_RESULT_SIZE_FUNC.id },
                        Instruction::ArrayLiftMemory { value_type },
                    ]
                }
            },
            ParsedType::Record(record_name, _) => {
                let record_type_id = it_resolver.get_record_type_id(record_name)? as u32;

                vec! [
                    Instruction::CallCore { function_index: GET_RESULT_PTR_FUNC.id },
                    Instruction::RecordLiftMemory { record_type_id },
                ]
            },
        };

        Ok(instructions)
    }
}
