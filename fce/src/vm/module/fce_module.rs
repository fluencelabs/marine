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
use super::{IType, IValue, WValue};

use fce_wit_interfaces::FCEWITInterfaces;
use wasmer_wit::interpreter::Interpreter;
use wasmer_runtime::{compile, ImportObject};
use wasmer_core::Instance as WasmerInstance;
use wasmer_core::import::Namespace;
use wit_parser::extract_wit;

use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::borrow::BorrowMut;

type WITInterpreter =
    Interpreter<WITInstance, WITExport, WITFunction, WITMemory, WITMemoryView<'static>>;

#[derive(Clone)]
pub(super) struct WITModuleFunc {
    interpreter: Arc<WITInterpreter>,
    pub(super) inputs: Vec<IType>,
    pub(super) outputs: Vec<IType>,
}

#[derive(Clone)]
pub(super) struct Callable {
    pub(super) wit_instance: Arc<WITInstance>,
    pub(super) wit_module_func: WITModuleFunc,
}

impl Callable {
    pub fn call(&mut self, args: &[IValue]) -> Result<Vec<IValue>, FCEError> {
        use wasmer_wit::interpreter::stack::Stackable;

        let result = self.wit_module_func
            .interpreter
            .run(args, Arc::make_mut(&mut self.wit_instance))?
            .as_slice()
            .to_owned();

        Ok(result)
    }
}

pub struct FCEModule {
    // it is needed because of WITInstance contains dynamic functions
    // that internally keep pointer to Wasmer instance.
    #[allow(unused)]
    wamser_instance: WasmerInstance,
    import_object: ImportObject,

    // TODO: replace with dyn Trait
    pub(super) exports_funcs: HashMap<String, Arc<Callable>>,
}

impl Drop for FCEModule {
    fn drop(&mut self) {
        println!("FCEModule dropped: {:?}", self.exports_funcs.keys());
    }
}

impl FCEModule {
    pub fn new(
        wasm_bytes: &[u8],
        fce_imports: ImportObject,
        modules: &HashMap<String, FCEModule>,
    ) -> Result<Self, FCEError> {
        let wasmer_module = compile(&wasm_bytes)?;
        let wit = extract_wit(&wasmer_module)?;
        let fce_wit = FCEWITInterfaces::new(wit);

        let mut wit_instance = Arc::new_uninit();
        let mut import_object = Self::adjust_wit_imports(&fce_wit, wit_instance.clone())?;
        let mut fce_imports = fce_imports;

        fce_imports.extend(import_object.clone());


        let wasmer_instance = wasmer_module.instantiate(&fce_imports)?;

        let wit_instance = unsafe {
            // get_mut_unchecked here is safe because currently only this modules have reference to
            // it and the environment is single-threaded
            *Arc::get_mut_unchecked(&mut wit_instance) =
                MaybeUninit::new(WITInstance::new(&wasmer_instance, &fce_wit, modules)?);
            std::mem::transmute::<_, Arc<WITInstance>>(wit_instance)
        };

        let exports_funcs = Self::instantiate_wit_exports(wit_instance.clone(), &fce_wit)?;

        Ok(Self {
            wamser_instance: wasmer_instance,
            import_object,
            exports_funcs,
        })
    }

    pub fn call(&mut self, function_name: &str, args: &[IValue]) -> Result<Vec<IValue>, FCEError> {
        use wasmer_wit::interpreter::stack::Stackable;

        match self.exports_funcs.get_mut(function_name) {
            Some(func) => {
                Arc::make_mut(func).call(args)
            }
            None => Err(FCEError::NoSuchFunction(format!(
                "{} hasn't been found while calling",
                function_name
            ))),
        }
    }

    pub fn get_func_signature(
        &self,
        function_name: &str,
    ) -> Result<(&Vec<IType>, &Vec<IType>), FCEError> {
        match self.exports_funcs.get(function_name) {
            Some(func) => Ok((&func.wit_module_func.inputs, &func.wit_module_func.outputs)),
            None => {
                for func in self.exports_funcs.iter() {
                    println!("{}", func.0);
                }

                Err(FCEError::NoSuchFunction(format!(
                    "{} has't been found during its signature looking up",
                    function_name
                )))
            }
        }
    }

    pub fn get_exports_signatures(
        &self,
    ) -> impl Iterator<Item = (&String, &Vec<IType>, &Vec<IType>)> {
        self.exports_funcs.iter().map(|(func_name, func)| {
            (
                func_name,
                &func.wit_module_func.inputs,
                &func.wit_module_func.outputs,
            )
        })
    }

    fn instantiate_wit_exports(
        wit_instance: Arc<WITInstance>,
        wit: &FCEWITInterfaces<'_>,
    ) -> Result<HashMap<String, Arc<Callable>>, FCEError> {
        use fce_wit_interfaces::WITAstType;

        wit.implementations()
            .filter_map(|(adapter_function_type, core_function_type)| {
                match wit.exports_by_type(*core_function_type) {
                    Some(export_function_name) => {
                        Some((adapter_function_type, export_function_name))
                    }
                    // pass functions that aren't export
                    None => None,
                }
            })
            .map(|(adapter_function_type, export_function_names)| {
                export_function_names
                    .iter()
                    .map(move |export_function_name| (*adapter_function_type, export_function_name))
            })
            .flatten()
            .map(|(adapter_function_type, export_function_name)| {
                let adapter_instructions = wit.adapter_by_type_r(adapter_function_type)?;
                let wit_type = wit.type_by_idx_r(adapter_function_type)?;

                match wit_type {
                    WITAstType::Function {
                        inputs, outputs, ..
                    } => {
                        let interpreter: WITInterpreter = adapter_instructions.try_into()?;
                        let wit_module_func = WITModuleFunc {
                            interpreter: Arc::new(interpreter),
                            inputs: inputs.clone(),
                            outputs: outputs.clone(),
                        };

                        Ok((
                            export_function_name.to_string(),
                            Arc::new(Callable {
                                wit_instance: wit_instance.clone(),
                                wit_module_func,
                            }),
                        ))
                    }
                    _ => Err(FCEError::IncorrectWIT(format!(
                        "type with idx = {} isn't a function type",
                        adapter_function_type
                    ))),
                }
            })
            .collect::<Result<HashMap<String, Arc<Callable>>, FCEError>>()
    }

    // this function deals only with import functions that have an adaptor implementation
    fn adjust_wit_imports(
        wit: &FCEWITInterfaces<'_>,
        wit_instance: Arc<MaybeUninit<WITInstance>>,
    ) -> Result<ImportObject, FCEError> {
        use fce_wit_interfaces::WITAstType;
        use wasmer_core::typed_func::DynamicFunc;
        use wasmer_core::vm::Ctx;

        #[derive(Clone)]
        struct T {}

        impl Drop for T {
            fn drop(&mut self) {
                println!("fce_module imports: drop T");
            }
        }

        // returns function that will be called from imports of Wasmer module
        fn dyn_func_from_raw_import(
            inputs: Vec<IType>,
            wit_instance: Arc<MaybeUninit<WITInstance>>,
            interpreter: WITInterpreter,
        ) -> DynamicFunc<'static> {
            use wasmer_core::types::FuncSig;
            use super::type_converters::itype_to_wtype;
            let t = T {};

            let signature = inputs.iter().map(itype_to_wtype).collect::<Vec<_>>();
            DynamicFunc::new(
                Arc::new(FuncSig::new(signature, vec![])),
                move |_: &mut Ctx, inputs: &[WValue]| -> Vec<WValue> {
                    use super::type_converters::wval_to_ival;
                    let t_copied = t.clone();

                    println!("dyn_func_from_raw_import: {:?}", inputs);

                    // copy here because otherwise wit_instance will be consumed by the closure
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
                },
            )
        }

        // creates a closure that is represent a WIT module import
        fn create_raw_import(
            wit_instance: Arc<MaybeUninit<WITInstance>>,
            interpreter: WITInterpreter,
        ) -> Box<dyn for<'a, 'b> Fn(&'a mut Ctx, &'b [WValue]) -> Vec<WValue> + 'static> {
            Box::new(move |_: &mut Ctx, inputs: &[WValue]| -> Vec<WValue> {
                use super::type_converters::wval_to_ival;

                // copy here because otherwise wit_instance will be consumed by the closure
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
            })
        }

        let wit_import_funcs = wit
            .implementations()
            .filter_map(|(adapter_function_type, core_function_type)| {
                match wit.imports_by_type(*core_function_type) {
                    Some(import) => Some((adapter_function_type, import)),
                    // skip functions that aren't import
                    None => None,
                }
            })
            .map(|(adapter_function_type, import_function_names)| {
                import_function_names
                    .iter()
                    .map(move |import_function_name| (*adapter_function_type, import_function_name))
            })
            .flatten()
            .map(|(adapter_function_type, (import_namespace, import_name))| {
                let adapter_instructions = wit.adapter_by_type_r(adapter_function_type)?;
                let wit_type = wit.type_by_idx_r(adapter_function_type)?;

                match wit_type {
                    WITAstType::Function { inputs, .. } => {
                        let interpreter: WITInterpreter = adapter_instructions.try_into()?;

                        let wit_import = dyn_func_from_raw_import(
                            inputs.clone(),
                            wit_instance.clone(),
                            interpreter,
                        );

                        Ok((import_namespace.to_string(), (*import_name, wit_import)))
                    }
                    _ => Err(FCEError::IncorrectWIT(format!(
                        "type with idx = {} isn't a function type",
                        adapter_function_type
                    ))),
                }
            })
            .collect::<Result<multimap::MultiMap<_, _>, FCEError>>()?;

        let mut import_object = ImportObject::new();

        // TODO: refactor it
        for (namespace_name, funcs) in wit_import_funcs.into_iter() {
            let mut namespace = Namespace::new();
            for (import_name, import_func) in funcs.into_iter() {
                namespace.insert(import_name.to_string(), import_func);
            }
            import_object.register(namespace_name, namespace);
        }

        Ok(import_object)
    }
}
