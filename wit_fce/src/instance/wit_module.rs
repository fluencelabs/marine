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

use crate::instance::errors::WITFCEError;
use crate::instance::exports::WITExport;
use crate::instance::locals_imports::WITLocalImport;
use crate::instance::memory::{WITMemory, WITMemoryView};
use crate::instance::wit_instance::WITInstance;

use wasmer_interface_types as wit;
use wasmer_interface_types::interpreter::Interpreter;
use wasmer_interface_types::values::InterfaceValue;
use wasmer_runtime::{compile, ImportObject};
use wasmer_runtime_core::Instance as WasmerInstance;

use std::collections::HashMap;
use std::convert::TryInto;
use wasmer_interface_types::interpreter::stack::Stackable;

const WIT_SECTION_NAME: &str = "interface-types";

pub struct WITModule {
    instance: WasmerInstance,
    wit_instance: WITInstance,
    exports: HashMap<
        String,
        Interpreter<WITInstance, WITExport, WITLocalImport, WITMemory, WITMemoryView<'static>>,
    >,
    import_object: ImportObject,
}

impl WITModule {
    pub fn new(wasm_bytes: &[u8], imports: &ImportObject) -> Result<Self, WITFCEError> {
        let wasmer_instance = compile(&wasm_bytes)?.instantiate(imports)?;

        let wit_sections = wasmer_instance
            .module
            .info
            .custom_sections
            .get(WIT_SECTION_NAME)
            .ok_or_else(|| WITFCEError::NoWITSection)?;

        if wit_sections.len() > 1 {
            return Err(WITFCEError::MultipleWITSections);
        }

        let (remainder, interfaces) = wit::decoders::binary::parse::<()>(&wit_sections[0])
            .map_err(|_e| WITFCEError::WITParseError)?;
        if remainder.len() > 1 {
            return Err(WITFCEError::WITRemainderNotEmpty);
        }

        let wit_instance = WITInstance::new(&wasmer_instance, &interfaces)?;

        let wit_export_names = interfaces
            .imports
            .iter()
            .map(|export| (export.function_type, export.name.to_string()))
            .collect::<HashMap<u32, String>>();

        let callable_exports = interfaces
            .adapters
            .iter()
            .map(|adapter| {
                let export_func_name = wit_export_names
                    .get(&adapter.function_type)
                    .ok_or_else(|| WITFCEError::NoSuchFunction)?;
                let instructions = &adapter.instructions;

                let interpreter: Interpreter<
                    WITInstance,
                    WITExport,
                    WITLocalImport,
                    WITMemory,
                    WITMemoryView<'static>,
                > = instructions.try_into().unwrap();

                Ok((export_func_name.to_owned(), interpreter))
            })
            .collect::<Result<HashMap<_, _>, WITFCEError>>()?;


        let callable_imports = interfaces
            .adapters
            .iter()
            .map(|adapter| {
                let import_func_name = wit_export_names
                    .get(&adapter.function_type)
                    .ok_or_else(|| WITFCEError::NoSuchFunction)?;
                let instructions = &adapter.instructions;

                let interpreter: Interpreter<
                    WITInstance,
                    WITExport,
                    WITLocalImport,
                    WITMemory,
                    WITMemoryView<'static>,
                > = instructions.try_into().unwrap();

                Ok((export_func_name.to_owned(), interpreter))
            })
            .collect::<Result<HashMap<_, _>, WITFCEError>>()?;

        Ok(Self {
            instance: wasmer_instance,
            wit_instance,
            exports: callable_exports,
        })
    }

    pub fn call(
        &mut self,
        function_name: &str,
        args: &[InterfaceValue],
    ) -> Result<Vec<InterfaceValue>, WITFCEError> {
        match self.exports.get(function_name) {
            Some(func) => {
                let result = func
                    .run(args, &mut self.wit_instance)?
                    .as_slice()
                    .to_owned();
                Ok(result)
            }
            None => Err(WITFCEError::NoSuchFunction),
        }
    }
}
