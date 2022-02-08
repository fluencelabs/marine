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

use super::wit_prelude::*;
use super::MFunctionSignature;
use super::MRecordTypes;
use super::{IType, IRecordType, IFunctionArg, IValue};
use crate::MResult;

use marine_it_interfaces::MITInterfaces;
use marine_utils::SharedString;
use crate::marine_js::{Instance as WasmerInstance};
use wasmer_it::interpreter::Interpreter;
use wasmer_it::ast::Interfaces;

use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;
use std::rc::Rc;
use crate::module::wit_function::WITFunction;

type ITInterpreter = Interpreter<ITInstance, ITExport, WITFunction, WITMemory, WITMemoryView>;

#[derive(Clone)]
pub(super) struct ITModuleFunc {
    interpreter: Arc<ITInterpreter>,
    pub(super) arguments: Rc<Vec<IFunctionArg>>,
    pub(super) output_types: Rc<Vec<IType>>,
}

#[derive(Clone)]
pub(super) struct Callable {
    pub(super) it_instance: Arc<ITInstance>,
    pub(super) it_module_func: ITModuleFunc,
}

impl Callable {
    pub fn call(&mut self, args: &[IValue]) -> MResult<Vec<IValue>> {
        use wasmer_it::interpreter::stack::Stackable;
        let result = self
            .it_module_func
            .interpreter
            .run(args, Arc::make_mut(&mut self.it_instance))?
            .as_slice()
            .to_owned();
        Ok(result)
    }
}

type ExportFunctions = HashMap<SharedString, Rc<Callable>>;

pub(crate) struct MModule {
    // wasmer_instance is needed because WITInstance contains dynamic functions
    // that internally keep pointer to it.
    #[allow(unused)]
    wasmer_instance: Box<WasmerInstance>,

    // TODO: replace with dyn Trait
    export_funcs: ExportFunctions,

    // TODO: save refs instead copying of a record types HashMap.
    /// Record types used in exported functions as arguments or return values.
    export_record_types: MRecordTypes,
}

pub(crate) fn extract_it_from_bytes(wit_section_bytes: &[u8]) -> Result<Interfaces<'_>, String> {
    match wasmer_it::decoders::binary::parse::<(&[u8], nom::error::ErrorKind)>(wit_section_bytes) {
        Ok((remainder, it)) if remainder.is_empty() => Ok(it),
        Ok(_) => Err("ITParserError::ITRemainderNotEmpty".to_string()),
        Err(e) => Err(format!("ITParserError::CorruptedITSection({})", e)),
    }
}

#[allow(unused)]
impl MModule {
    pub(crate) fn new(name: &str, wit_section_bytes: &[u8]) -> MResult<Self> {
        let it = extract_it_from_bytes(&wit_section_bytes)?;
        crate::misc::check_it_version(name, &it.version)?;

        let mit = MITInterfaces::new(it);
        let wasmer_instance = WasmerInstance::new(&mit, Rc::new(name.to_string()));
        let it_instance = Arc::new(ITInstance::new(&wasmer_instance, &mit)?);
        let (export_funcs, export_record_types) = Self::instantiate_exports(&it_instance, &mit)?;
        Ok(Self {
            wasmer_instance: Box::new(wasmer_instance),
            export_funcs,
            export_record_types,
        })
    }

    pub(crate) fn call(
        &mut self,
        module_name: &str,
        function_name: &str,
        args: &[IValue],
    ) -> MResult<Vec<IValue>> {
        self.export_funcs.get_mut(function_name).map_or_else(
            || {
                Err(MError::NoSuchFunction(
                    module_name.to_string(),
                    function_name.to_string(),
                ))
            },
            |func| Rc::make_mut(func).call(args),
        )
    }

    pub(crate) fn get_exports_signatures(&self) -> impl Iterator<Item = MFunctionSignature> + '_ {
        self.export_funcs
            .iter()
            .map(|(func_name, func)| MFunctionSignature {
                name: func_name.0.clone(),
                arguments: func.it_module_func.arguments.clone(),
                outputs: func.it_module_func.output_types.clone(),
            })
    }

    pub(crate) fn export_record_types(&self) -> &MRecordTypes {
        &self.export_record_types
    }

    pub(crate) fn export_record_type_by_id(&self, record_type: u64) -> Option<&Rc<IRecordType>> {
        self.export_record_types.get(&record_type)
    }

    // TODO: change the cloning Callable behaviour after changes of Wasmer API
    pub(super) fn get_callable(
        &self,
        module_name: &str,
        function_name: &str,
    ) -> MResult<Rc<Callable>> {
        match self.export_funcs.get(function_name) {
            Some(func) => Ok(func.clone()),
            None => Err(MError::NoSuchFunction(
                module_name.to_string(),
                function_name.to_string(),
            )),
        }
    }

    fn instantiate_exports(
        it_instance: &Arc<ITInstance>,
        mit: &MITInterfaces<'_>,
    ) -> MResult<(ExportFunctions, MRecordTypes)> {
        let module_interface = marine_module_interface::it_interface::get_interface(mit)?;

        let export_funcs = module_interface
            .function_signatures
            .into_iter()
            .map(|sign| {
                let adapter_instructions = mit.adapter_by_type_r(sign.adapter_function_type)?;

                let interpreter: ITInterpreter = adapter_instructions.clone().try_into()?;
                let it_module_func = ITModuleFunc {
                    interpreter: Arc::new(interpreter),
                    arguments: sign.arguments.clone(),
                    output_types: sign.outputs.clone(),
                };

                let shared_string = SharedString(sign.name);
                let callable = Rc::new(Callable {
                    it_instance: it_instance.clone(),
                    it_module_func,
                });

                Ok((shared_string, callable))
            })
            .collect::<MResult<ExportFunctions>>()?;

        Ok((export_funcs, module_interface.export_record_types))
    }
}
