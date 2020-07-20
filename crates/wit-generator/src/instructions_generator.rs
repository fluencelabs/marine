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

mod fn_instructions;
mod foreign_mod_instructions;
mod record_instructions;
mod utils;

use fluence_sdk_wit::FCEAst;

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
