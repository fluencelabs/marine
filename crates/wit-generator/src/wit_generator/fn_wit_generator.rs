use super::WITGenerator;
use super::Interfaces;
use super::utils::ptype_to_itype;
use super::FnInstructionGenerator;

use fluence_sdk_wit::AstFunctionItem;
use fluence_sdk_wit::ParsedType;
use wasmer_wit::interpreter::Instruction;

impl WITGenerator for AstFunctionItem {
    fn generate_wit<'a>(&'a self, interfaces: &mut Interfaces<'a>) {
        use wasmer_wit::ast::Type;
        use wasmer_wit::ast::Adapter;

        let inputs = self
            .signature
            .input_types
            .iter()
            .map(ptype_to_itype)
            .collect::<Vec<_>>();

        let outputs = match self.signature.output_type {
            Some(ref output_type) => vec![ptype_to_itype(output_type)],
            None => vec![],
        };

        interfaces.types.push(Type::Function {
            inputs: inputs.clone(),
            outputs: outputs.clone(),
        });

        // TODO: replace with Wasm types
        interfaces.types.push(Type::Function { inputs, outputs });

        let adapter_idx = (interfaces.types.len() - 2) as u32;
        let export_idx = (interfaces.types.len() - 1) as u32;

        interfaces.exports.push(wasmer_wit::ast::Export {
            name: &self.signature.name,
            function_type: export_idx,
        });

        let mut instructions: Vec<Instruction> = self
            .signature
            .input_types
            .iter()
            .rev()
            .enumerate()
            .map(|(id, input_type)| input_type.generate_instructions_for_input_type(id as _))
            .flatten()
            .collect();

        instructions.push(Instruction::CallCore {
            function_index: export_idx,
        });

        instructions.extend(match &self.signature.output_type {
            Some(output_type) => output_type.generate_instructions_for_output_type(),
            None => vec![],
        });

        let adapter = Adapter {
            function_type: adapter_idx,
            instructions,
        };

        interfaces.adapters.push(adapter);

        let implementation = wasmer_wit::ast::Implementation {
            core_function_type: export_idx,
            adapter_function_type: adapter_idx,
        };
        interfaces.implementations.push(implementation);
    }
}

impl FnInstructionGenerator for ParsedType {
    fn generate_instructions_for_input_type(&self, index: u32) -> Vec<Instruction> {
        match self {
            ParsedType::I8 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromS8],
            ParsedType::I16 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromS16],
            ParsedType::I32 => vec![Instruction::ArgumentGet { index }],
            ParsedType::I64 => vec![Instruction::ArgumentGet { index }],
            ParsedType::U8 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU8],
            ParsedType::U16 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU16],
            ParsedType::U32 => vec![Instruction::ArgumentGet { index }, Instruction::I32FromU32],
            ParsedType::U64 => vec![Instruction::ArgumentGet { index }, Instruction::I64FromU64],
            ParsedType::F32 => vec![Instruction::ArgumentGet { index }],
            ParsedType::F64 => vec![Instruction::ArgumentGet { index }],
            ParsedType::Utf8String => vec![
                Instruction::ArgumentGet { index },
                Instruction::StringSize,
                Instruction::CallCore { function_index: 0 },
                Instruction::ArgumentGet { index },
                Instruction::StringLowerMemory,
            ],
            ParsedType::ByteVector => vec![
                Instruction::ArgumentGet { index },
                Instruction::StringSize,
                Instruction::CallCore { function_index: 0 },
                Instruction::ArgumentGet { index },
                Instruction::StringLowerMemory,
            ],
            _ => unimplemented!(),
        }
    }

    fn generate_instructions_for_output_type(&self) -> Vec<Instruction> {
        match self {
            ParsedType::I8 => vec![Instruction::S8FromI32],
            ParsedType::I16 => vec![Instruction::S16FromI32],
            ParsedType::I32 => vec![],
            ParsedType::I64 => vec![],
            ParsedType::U8 => vec![Instruction::U8FromI32],
            ParsedType::U16 => vec![Instruction::U16FromI32],
            ParsedType::U32 => vec![Instruction::U32FromI32],
            ParsedType::U64 => vec![Instruction::U64FromI64],
            ParsedType::F32 => vec![],
            ParsedType::F64 => vec![],
            ParsedType::Utf8String => vec![
                Instruction::CallCore { function_index: 3 },
                Instruction::CallCore { function_index: 2 },
                Instruction::StringLiftMemory,
                Instruction::CallCore { function_index: 3 },
                Instruction::CallCore { function_index: 2 },
                Instruction::CallCore { function_index: 1 },
            ],
            ParsedType::ByteVector => vec![
                Instruction::CallCore { function_index: 3 },
                Instruction::CallCore { function_index: 2 },
                Instruction::StringLiftMemory,
                Instruction::CallCore { function_index: 3 },
                Instruction::CallCore { function_index: 2 },
                Instruction::CallCore { function_index: 1 },
            ],
            _ => unimplemented!(),
        }
    }
}
