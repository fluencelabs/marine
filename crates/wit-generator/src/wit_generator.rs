mod fn_wit_generator;
mod foreign_mod_wit_generator;
mod record_wit_generator;
mod utils;

use super::FCEAst;

use wasmer_wit::types::InterfaceType as IType;
use wasmer_wit::ast::Interfaces;
use wasmer_wit::interpreter::Instruction;

pub trait WITGenerator {
    fn generate_wit<'a>(&'a self, interfaces: &mut Interfaces<'a>);
}

trait FnInstructionGenerator {
    fn generate_instructions_for_input_type(&self, arg_id: u32) -> Vec<Instruction>;

    fn generate_instructions_for_output_type(&self) -> Vec<Instruction>;
}

trait ForeignModInstructionGenerator {
    fn generate_instructions_for_input_type(&self, arg_id: u32) -> Vec<Instruction>;

    fn generate_instructions_for_output_type(&self) -> Vec<Instruction>;
}

impl WITGenerator for FCEAst {
    fn generate_wit<'a>(&'a self, interfaces: &mut Interfaces<'a>) {
        match self {
            FCEAst::Function(func) => func.generate_wit(interfaces),
            FCEAst::ExternMod(extern_mod) => extern_mod.generate_wit(interfaces),
            FCEAst::Record(record) => record.generate_wit(interfaces),
        }
    }
}
