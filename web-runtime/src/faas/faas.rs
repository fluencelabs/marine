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

use crate::faas::faas_interface::FaaSInterface;
use crate::faas::FaaSError;
use crate::faas::Result;
use crate::IValue;
use crate::IType;

use crate::Marine;
use crate::IFunctionArg;
//use marine_utils::SharedString;
use crate::MRecordTypes;
use marine_rs_sdk::CallParameters;

use serde_json::Value as JValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type MFunctionSignature = (Rc<Vec<IFunctionArg>>, Rc<Vec<IType>>);
type MModuleInterface = (Rc<Vec<IFunctionArg>>, Rc<Vec<IType>>, Rc<MRecordTypes>);

struct ModuleInterface {
    function_signatures: HashMap<String, MFunctionSignature>,
    record_types: Rc<MRecordTypes>,
}

// TODO: remove and use mutex instead
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for FluenceFaaS {}

pub struct FluenceFaaS {
    /// Marine instance.
    marine: Marine,

    /// Parameters of call accessible by Wasm modules.
    call_parameters: Rc<RefCell<CallParameters>>,

    /// Cached module interfaces by names.
    module_interfaces_cache: HashMap<String, ModuleInterface>,
}

#[allow(unused)]
impl FluenceFaaS {
    /// Creates FaaS with given modules.
    pub fn with_modules(modules: HashMap<String, Vec<u8>>) -> Result<Self> {
        let mut marine = Marine::new();
        let call_parameters = Rc::new(RefCell::new(CallParameters::default()));

        for (name, wit_section_bytes) in modules {
            marine.load_module(name, &wit_section_bytes)?;
        }

        Ok(Self {
            marine,
            call_parameters,
            module_interfaces_cache: HashMap::new(),
        })
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call_with_ivalues<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        args: &[IValue],
        call_parameters: marine_rs_sdk::CallParameters,
    ) -> Result<Vec<IValue>> {
        self.call_parameters.replace(call_parameters);

        self.marine
            .call(module_name, func_name, args)
            .map_err(Into::into)
    }

    /// Call a specified function of loaded on a startup module by its name.
    pub fn call_with_json<MN: AsRef<str>, FN: AsRef<str>>(
        &mut self,
        module_name: MN,
        func_name: FN,
        json_args: JValue,
        call_parameters: marine_rs_sdk::CallParameters,
    ) -> Result<JValue> {
        use crate::faas::json::json_to_ivalues;
        use crate::faas::json::ivalues_to_json;

        let module_name = module_name.as_ref();
        let func_name = func_name.as_ref();

        let (func_signature, output_types, record_types) =
            self.lookup_module_interface(module_name, func_name)?;
        let iargs = json_to_ivalues(
            json_args,
            func_signature.iter().map(|arg| (&arg.name, &arg.ty)),
            &record_types,
        )?;

        self.call_parameters.replace(call_parameters);
        let result = self.marine.call(module_name, func_name, &iargs)?;

        ivalues_to_json(result, &output_types, &record_types)
    }

    /// Return all export functions (name and signatures) of loaded modules.
    pub fn get_interface(&self) -> FaaSInterface<'_> {
        let modules = self.marine.interface().collect();

        FaaSInterface { modules }
    }

    /// At first, tries to find function signature and record types in module_interface_cache,
    /// if there is no them, tries to look
    fn lookup_module_interface<'faas>(
        &'faas mut self,
        module_name: &str,
        func_name: &str,
    ) -> Result<MModuleInterface> {
        use FaaSError::NoSuchModule;
        use FaaSError::MissingFunctionError;

        if let Some(module_interface) = self.module_interfaces_cache.get(module_name) {
            if let Some(function) = module_interface.function_signatures.get(func_name) {
                return Ok((
                    function.0.clone(),
                    function.1.clone(),
                    module_interface.record_types.clone(),
                ));
            }

            return Err(MissingFunctionError(func_name.to_string()));
        }

        let module_interface = self
            .marine
            .module_interface(module_name)
            .ok_or_else(|| NoSuchModule(module_name.to_string()))?;

        let function_signatures = module_interface
            .function_signatures
            .iter()
            .cloned()
            .map(|f| (f.name.to_string(), (f.arguments, f.outputs)))
            .collect::<HashMap<_, _>>();

        let (arg_types, output_types) = function_signatures
            .get(func_name)
            .ok_or_else(|| MissingFunctionError(func_name.to_string()))?;

        let arg_types = arg_types.clone();
        let output_types = output_types.clone();
        let record_types = Rc::new(module_interface.record_types.clone());

        let module_interface = ModuleInterface {
            function_signatures,
            record_types: record_types.clone(),
        };

        self.module_interfaces_cache
            .insert(func_name.to_string(), module_interface);

        Ok((arg_types, output_types, record_types))
    }
}
