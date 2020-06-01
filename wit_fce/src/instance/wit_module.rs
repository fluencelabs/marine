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
use crate::instance::memory::{WITMemory, WITMemoryView};
use crate::instance::wit_function::WITFunction;
use crate::instance::wit_instance::WITInstance;

use wasmer_interface_types as wit;
use wasmer_interface_types::ast::{Interfaces, Type};
use wasmer_interface_types::interpreter::Interpreter;
use wasmer_interface_types::values::InterfaceValue;
use wasmer_runtime::{compile, ImportObject};
use wasmer_runtime_core::Instance as WasmerInstance;

use multimap::MultiMap;
use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::sync::Arc;
use wasmer_interface_types::interpreter::stack::Stackable;
use wasmer_interface_types::types::InterfaceType;
use wasmer_runtime_core::import::Namespace;

const WIT_SECTION_NAME: &str = "interface-types";
type WITInterpreter =
    Interpreter<WITInstance, WITExport, WITFunction, WITMemory, WITMemoryView<'static>>;
type WITModuleFunc = (WITInterpreter, Vec<InterfaceType>, Vec<InterfaceType>);

pub struct WITModule {
    #[allow(unused)]
    instance: WasmerInstance,
    wit_instance: Arc<WITInstance>,
    funcs: HashMap<String, WITModuleFunc>,
}

impl WITModule {
    pub fn new(
        wasm_bytes: &[u8],
        imports: ImportObject,
        modules: &HashMap<String, Arc<WITModule>>,
    ) -> Result<Self, WITFCEError> {
        let wasmer_instance = compile(&wasm_bytes)?;

        let wit_sections = wasmer_instance
            .custom_sections(WIT_SECTION_NAME)
            .ok_or_else(|| WITFCEError::NoWITSection)?;

        if wit_sections.len() > 1 {
            return Err(WITFCEError::MultipleWITSections);
        }

        let (remainder, interfaces) = wit::decoders::binary::parse::<()>(&wit_sections[0])
            .map_err(|_e| WITFCEError::WITParseError)?;
        if remainder.len() > 1 {
            return Err(WITFCEError::WITRemainderNotEmpty);
        }

        let mut wit_instance = Arc::new_uninit();

        let callable_exports = Self::extract_wit_exports(&interfaces)?;
        let mut import_object = Self::adjust_imports(&interfaces, wit_instance.clone())?;
        import_object.extend(imports);

        let wasmer_instance = wasmer_instance.instantiate(&import_object)?;

        let wit_instance = unsafe {
            // get_mut_unchecked here is safe because currently only this modules have reference to
            // it and the environment is single-threaded
            *Arc::get_mut_unchecked(&mut wit_instance) =
                MaybeUninit::new(WITInstance::new(&wasmer_instance, &interfaces, modules)?);
            std::mem::transmute::<_, Arc<WITInstance>>(wit_instance)
        };

        Ok(Self {
            instance: wasmer_instance,
            wit_instance,
            funcs: callable_exports,
        })
    }

    pub fn call(
        &mut self,
        function_name: &str,
        args: &[InterfaceValue],
    ) -> Result<Vec<InterfaceValue>, WITFCEError> {
        match self.funcs.get(function_name) {
            Some(func) => {
                let result = func
                    .0
                    .run(args, Arc::make_mut(&mut self.wit_instance))?
                    .as_slice()
                    .to_owned();
                Ok(result)
            }
            None => Err(WITFCEError::NoSuchFunction(format!(
                "{} hasn't been found while calling",
                function_name
            ))),
        }
    }

    pub fn get_func_signature(
        &self,
        function_name: &str,
    ) -> Result<(&Vec<InterfaceType>, &Vec<InterfaceType>), WITFCEError> {
        match self.funcs.get(function_name) {
            Some((_, inputs, outputs)) => Ok((inputs, outputs)),
            None => Err(WITFCEError::NoSuchFunction(format!(
                "{} has't been found during its signature looking up",
                function_name
            ))),
        }
    }

    fn extract_wit_exports(
        interfaces: &Interfaces,
    ) -> Result<HashMap<String, WITModuleFunc>, WITFCEError> {
        let exports_type_to_names = interfaces
            .exports
            .iter()
            .map(|export| (export.function_type, export.name.to_string()))
            .collect::<MultiMap<_, _>>();

        let adapter_type_to_instructions = interfaces
            .adapters
            .iter()
            .map(|adapter| (adapter.function_type, &adapter.instructions))
            .collect::<HashMap<_, _>>();

        let mut wit_callable_exports = HashMap::new();
        for i in interfaces.implementations.iter() {
            let export_function_names = match exports_type_to_names.get_vec(&i.core_function_type) {
                Some(export_function_names) => export_function_names,
                None => continue,
            };

            // * just to remove reference
            let adapter_instructions = *adapter_type_to_instructions
                .get(&i.adapter_function_type)
                .ok_or_else(|| WITFCEError::NoSuchFunction(
                    format!("adapter function with idx = {} hasn't been found during extracting exports by implementations", i.adapter_function_type)
                ))?;

            if i.adapter_function_type >= interfaces.types.len() as u32 {
                // TODO: change error type
                return Err(WITFCEError::NoSuchFunction(format!(
                    "{} function id is bigger than WIT interface types count",
                    i.adapter_function_type
                )));
            };

            if let Type::Function { inputs, outputs } =
                &interfaces.types[i.adapter_function_type as usize]
            {
                for export_function_name in export_function_names.iter() {
                    // TODO: handle errors
                    let interpreter: WITInterpreter = adapter_instructions.try_into().unwrap();
                    wit_callable_exports.insert(
                        export_function_name.to_owned(),
                        (interpreter, inputs.clone(), outputs.clone()),
                    );
                }
            } else {
                return Err(WITFCEError::NoSuchFunction(format!(
                    "type with idx = {} isn't a function type",
                    i.adapter_function_type
                )));
            }
        }

        Ok(wit_callable_exports)
    }

    // this function deals only with import functions that have an adaptor implementation
    fn adjust_imports(
        interfaces: &Interfaces,
        wit_instance: Arc<MaybeUninit<WITInstance>>,
    ) -> Result<ImportObject, WITFCEError> {
        use crate::instance::{itype_to_wtype, wval_to_ival};
        use wasmer_interface_types::ast::Type as IType;
        use wasmer_runtime_core::typed_func::DynamicFunc;
        use wasmer_runtime_core::types::{FuncSig, Value};
        use wasmer_runtime_core::vm::Ctx;

        // returns function that will be called from imports of Wasmer module
        fn dyn_func_from_imports<F>(inputs: Vec<InterfaceType>, func: F) -> DynamicFunc<'static>
        where
            F: Fn(&mut Ctx, &[Value]) -> Vec<Value> + 'static,
        {
            let signature = inputs.iter().map(itype_to_wtype).collect::<Vec<_>>();
            DynamicFunc::new(Arc::new(FuncSig::new(signature, vec![])), func)
        }

        // uses to filter out import functions that have an adapter implementation
        let adapter_to_core = interfaces
            .implementations
            .iter()
            .map(|i| (i.adapter_function_type, i.core_function_type))
            .collect::<HashMap<_, _>>();

        // all wit imports
        let mut export_type_to_name = interfaces
            .imports
            .iter()
            .map(|import| {
                (
                    import.function_type,
                    (import.namespace.to_string(), import.name.to_string()),
                )
            })
            .collect::<HashMap<_, _>>();

        let mut import_namespaces: HashMap<String, Namespace> = HashMap::new();

        for adapter in interfaces.adapters.iter() {
            let core_function_idx = adapter_to_core
                .get(&adapter.function_type)
                .ok_or_else(|| WITFCEError::NoSuchFunction(format!("function with idx = {} hasn't been found during adjusting imports in WIT implementation", adapter.function_type)))?;

            let (namespace, func_name) = match export_type_to_name.remove(core_function_idx) {
                Some(v) => (v.0, v.1),
                None => continue,
            };

            if adapter.function_type >= interfaces.types.len() as u32 {
                // TODO: change error type
                return Err(WITFCEError::NoSuchFunction(format!(
                    "{} function id is bigger than WIT interface types count",
                    adapter.function_type
                )));
            }

            if let IType::Function { inputs, .. } =
                &interfaces.types[adapter.function_type as usize]
            {
                let instructions = &adapter.instructions;
                let interpreter: WITInterpreter = instructions.try_into().unwrap();

                let wit_instance = wit_instance.clone();
                let wit_inner_import =
                    Box::new(move |_: &mut Ctx, inputs: &[Value]| -> Vec<Value> {
                        // copy here to because otherwise wit_instance will be consumed by the closure
                        let wit_instance_callable = wit_instance.clone();
                        let converted_inputs = inputs.iter().map(wval_to_ival).collect::<Vec<_>>();
                        unsafe {
                            // error here will be propagated by the special error instruction
                            let _ = interpreter.run(
                                &converted_inputs,
                                Arc::make_mut(&mut wit_instance_callable.assume_init()),
                            );
                        }

                        // wit import functions should only change the stack state -
                        // the result will be returned by an export function
                        vec![]
                    });

                let wit_import = dyn_func_from_imports(inputs.clone(), wit_inner_import);

                // TODO: refactor this
                let mut module_namespace = Namespace::new();
                module_namespace.insert(func_name.clone(), wit_import);

                import_namespaces.insert(namespace, module_namespace);
            } else {
                // TODO: change error type
                return Err(WITFCEError::WasmerResolveError(format!(
                    "WIT type with idx = {} doesn't refer to function",
                    adapter.function_type
                )));
            }
        }

        let mut import_object = ImportObject::new();

        for (namespace_name, namespace) in import_namespaces.into_iter() {
            import_object.register(namespace_name, namespace);
        }

        Ok(import_object)
    }
}
