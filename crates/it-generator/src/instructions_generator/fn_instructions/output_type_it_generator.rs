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
