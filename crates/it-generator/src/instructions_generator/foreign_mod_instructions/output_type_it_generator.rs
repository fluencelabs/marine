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
use crate::default_export_api_config::*;

use marine_macro_impl::ParsedType;
use wasmer_it::interpreter::Instruction;
use wasmer_it::IType;

/// Generate IT instructions for a foreign mod.
pub(super) trait OutputITGenerator {
    fn generate_instructions_for_output_type(
        &self,
        it_resolver: &mut ITResolver<'_>,
    ) -> Result<Vec<Instruction>>;
}

#[rustfmt::skip]
impl OutputITGenerator for ParsedType {
    #[rustfmt::skip]
    fn generate_instructions_for_output_type(&self, it_resolver: &mut ITResolver<'_>) -> Result<Vec<Instruction>> {
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
                let value_type = ptype_to_itype_checked(value_type, it_resolver)?;

                vec![
                    Instruction::ArrayLowerMemory { value_type },
                    Instruction::CallCore { function_index: SET_RESULT_SIZE_FUNC.id },
                    Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
                ]
            },
            ParsedType::Record(record_name, _) => {
                let record_type_id = it_resolver.get_record_type_id(record_name)? as u32;

                vec![
                    Instruction::RecordLowerMemory { record_type_id },
                    Instruction::CallCore { function_index: SET_RESULT_PTR_FUNC.id },
                ]
            },
        };

        Ok(instructions)
    }
}
