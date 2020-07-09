use super::WITGenerator;
use super::Interfaces;
use super::utils::ptype_to_itype;
use super::ForeignModInstructionGenerator;

use fluence_sdk_wit::AstExternModItem;
use fluence_sdk_wit::AstExternFnItem;
use fluence_sdk_wit::ParsedType;
use wasmer_wit::interpreter::Instruction;

impl WITGenerator for AstExternModItem {
    fn generate_wit<'a>(&'a self, interfaces: &mut Interfaces<'a>) {
        for import in &self.imports {
            generate_wit_for_import(import, &self.namespace, interfaces);
        }
    }
}

fn generate_wit_for_import<'a>(
    import: &'a AstExternFnItem,
    namespace: &'a String,
    interfaces: &mut Interfaces<'a>,
) {
    use wasmer_wit::ast::Type;
    use wasmer_wit::ast::Adapter;

    let inputs = import
        .signature
        .input_types
        .iter()
        .map(ptype_to_itype)
        .collect::<Vec<_>>();

    let outputs = match import.signature.output_type {
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
    let import_idx = (interfaces.types.len() - 1) as u32;

    interfaces.imports.push(wasmer_wit::ast::Import {
        namespace: &namespace,
        name: &import.signature.name,
        function_type: import_idx,
    });

    let mut instructions: Vec<Instruction> = import
        .signature
        .input_types
        .iter()
        .rev()
        .enumerate()
        .map(|(id, input_type)| input_type.generate_instructions_for_input_type(id as _))
        .flatten()
        .collect();

    instructions.push(Instruction::CallCore {
        function_index: import_idx,
    });

    instructions.extend(match &import.signature.output_type {
        Some(output_type) => output_type.generate_instructions_for_output_type(),
        None => vec![],
    });

    let adapter = Adapter {
        function_type: adapter_idx,
        instructions,
    };
    interfaces.adapters.push(adapter);

    let implementation = wasmer_wit::ast::Implementation {
        core_function_type: import_idx,
        adapter_function_type: adapter_idx,
    };
    interfaces.implementations.push(implementation);
}

impl ForeignModInstructionGenerator for ParsedType {
    fn generate_instructions_for_input_type(&self, index: u32) -> Vec<Instruction> {
        match self {
            ParsedType::I8 => vec![Instruction::ArgumentGet { index }, Instruction::S8FromI32],
            ParsedType::I16 => vec![Instruction::ArgumentGet { index }, Instruction::S16FromI32],
            ParsedType::I32 => vec![Instruction::ArgumentGet { index }],
            ParsedType::I64 => vec![Instruction::ArgumentGet { index }],
            ParsedType::U8 => vec![Instruction::ArgumentGet { index }, Instruction::U8FromI32],
            ParsedType::U16 => vec![Instruction::ArgumentGet { index }, Instruction::U16FromI32],
            ParsedType::U32 => vec![Instruction::ArgumentGet { index }, Instruction::U32FromI32],
            ParsedType::U64 => vec![Instruction::ArgumentGet { index }, Instruction::U64FromI64],
            ParsedType::F32 => vec![Instruction::ArgumentGet { index }],
            ParsedType::F64 => vec![Instruction::ArgumentGet { index }],
            ParsedType::Utf8String => vec![
                Instruction::ArgumentGet { index },
                Instruction::ArgumentGet { index: index + 1 },
                Instruction::StringLiftMemory,
            ],
            ParsedType::ByteVector => vec![
                Instruction::ArgumentGet { index },
                Instruction::ArgumentGet { index: index + 1 },
                Instruction::StringLiftMemory,
            ],
            _ => unimplemented!(),
        }
    }

    fn generate_instructions_for_output_type(&self) -> Vec<Instruction> {
        match self {
            ParsedType::I8 => vec![Instruction::I32FromS8],
            ParsedType::I16 => vec![Instruction::I32FromS16],
            ParsedType::I32 => vec![],
            ParsedType::I64 => vec![],
            ParsedType::U8 => vec![Instruction::I32FromU8],
            ParsedType::U16 => vec![Instruction::I32FromU16],
            ParsedType::U32 => vec![Instruction::I32FromU32],
            ParsedType::U64 => vec![Instruction::I64FromU64],
            ParsedType::F32 => vec![],
            ParsedType::F64 => vec![],
            ParsedType::Utf8String => vec![
                Instruction::Dup,
                Instruction::StringSize,
                Instruction::CallCore { function_index: 0 },
                Instruction::Swap2,
                Instruction::StringLowerMemory,
                Instruction::CallCore { function_index: 4 },
                Instruction::CallCore { function_index: 5 },
            ],
            ParsedType::ByteVector => vec![
                Instruction::Dup,
                Instruction::StringSize,
                Instruction::CallCore { function_index: 0 },
                Instruction::Swap2,
                Instruction::StringLowerMemory,
                Instruction::CallCore { function_index: 4 },
                Instruction::CallCore { function_index: 5 },
            ],
            _ => unimplemented!(),
        }
    }
}
