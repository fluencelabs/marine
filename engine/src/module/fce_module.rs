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
use crate::FCEModuleConfig;

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

        let result = self
            .wit_module_func
            .interpreter
            .run(args, Arc::make_mut(&mut self.wit_instance))?
            .as_slice()
            .to_owned();

        Ok(result)
    }
}

pub(crate) struct FCEModule {
    // wasmer_instance is needed because WITInstance contains dynamic functions
    // that internally keep pointer to Wasmer instance.
    #[allow(unused)]
    wasmer_instance: Box<WasmerInstance>,

    // import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    #[allow(unused)]
    import_object: ImportObject,

    // host_import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    #[allow(unused)]
    host_import_object: ImportObject,

    // TODO: replace with dyn Trait
    exports_funcs: HashMap<String, Arc<Callable>>,
}

impl FCEModule {
    pub(crate) fn new(
        wasm_bytes: &[u8],
        fce_module_config: FCEModuleConfig,
        modules: &HashMap<String, FCEModule>,
    ) -> Result<Self, FCEError> {
        let wasmer_module = compile(&wasm_bytes)?;
        let wit = extract_wit(&wasmer_module)?;
        let fce_wit = FCEWITInterfaces::new(wit);

        let mut wit_instance = Arc::new_uninit();
        let import_object = Self::adjust_wit_imports(&fce_wit, wit_instance.clone())?;

        let mut wasi_import_object = wasmer_wasi::generate_import_object_for_version(
            fce_module_config.wasi_version,
            vec![],
            fce_module_config.wasi_envs.clone(),
            fce_module_config.wasi_preopened_files.clone(),
            fce_module_config.wasi_mapped_dirs.clone(),
        );

        wasi_import_object.extend(import_object.clone());
        wasi_import_object.extend(fce_module_config.imports.clone());

        let wasmer_instance = wasmer_module.instantiate(&wasi_import_object)?;
        let wit_instance = unsafe {
            // get_mut_unchecked here is safe because currently only this modules have reference to
            // it and the environment is single-threaded
            *Arc::get_mut_unchecked(&mut wit_instance) =
                MaybeUninit::new(WITInstance::new(&wasmer_instance, &fce_wit, modules)?);
            std::mem::transmute::<_, Arc<WITInstance>>(wit_instance)
        };

        let exports_funcs = Self::instantiate_wit_exports(wit_instance, &fce_wit)?;

        // call _start to populate the WASI state of the module
        #[rustfmt::skip]
        if let Ok(start_func) = wasmer_instance.exports.get::<wasmer_runtime::Func<'_, (), ()>>("_start") {
            start_func.call()?;
        }

        Ok(Self {
            wasmer_instance: Box::new(wasmer_instance),
            import_object,
            host_import_object: fce_module_config.imports,
            exports_funcs,
        })
    }

    pub(crate) fn call(
        &mut self,
        function_name: &str,
        args: &[IValue],
    ) -> Result<Vec<IValue>, FCEError> {
        match self.exports_funcs.get_mut(function_name) {
            Some(func) => Arc::make_mut(func).call(args),
            None => Err(FCEError::NoSuchFunction(format!(
                "{} hasn't been found while calling",
                function_name
            ))),
        }
    }

    pub(crate) fn get_exports_signatures(
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

    // TODO: change the cloning Callable behaviour after changes of Wasmer API
    pub(super) fn get_callable(&self, function_name: &str) -> Result<Arc<Callable>, FCEError> {
        match self.exports_funcs.get(function_name) {
            Some(func) => Ok(func.clone()),
            None => Err(FCEError::NoSuchFunction(format!(
                "{} hasn't been found while calling",
                function_name
            ))),
        }
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

        // returns function that will be called from imports of Wasmer module
        fn dyn_func_from_raw_import<F>(
            inputs: Vec<IType>,
            outputs: Vec<IType>,
            raw_import: F,
        ) -> DynamicFunc<'static>
        where
            F: Fn(&mut Ctx, &[WValue]) -> Vec<WValue> + 'static,
        {
            use wasmer_core::types::FuncSig;
            use super::type_converters::itype_to_wtype;

            let inputs = inputs.iter().map(itype_to_wtype).collect::<Vec<_>>();
            let outputs = outputs.iter().map(itype_to_wtype).collect::<Vec<_>>();
            DynamicFunc::new(Arc::new(FuncSig::new(inputs, outputs)), raw_import)
        }

        // creates a closure that is represent a WIT module import
        fn create_raw_import(
            wit_instance: Arc<MaybeUninit<WITInstance>>,
            interpreter: WITInterpreter,
            import_namespace: String,
            import_name: String,
        ) -> impl Fn(&mut Ctx, &[WValue]) -> Vec<WValue> + 'static {
            move |_: &mut Ctx, inputs: &[WValue]| -> Vec<WValue> {
                use super::type_converters::wval_to_ival;

                log::info!(
                    "raw import for {}.{} called with {:?}\n",
                    import_namespace,
                    import_name,
                    inputs
                );

                // copy here because otherwise wit_instance will be consumed by the closure
                let wit_instance_callable = wit_instance.clone();
                let wit_inputs = inputs.iter().map(wval_to_ival).collect::<Vec<_>>();
                unsafe {
                    // error here will be propagated by the special error instruction
                    let _ = interpreter.run(
                        &wit_inputs,
                        Arc::make_mut(&mut wit_instance_callable.assume_init()),
                    );
                }

                log::info!(
                    "\nraw import for {}.{} finished",
                    import_namespace,
                    import_name
                );

                // wit import functions should only change the stack state -
                // the result will be returned by an export function
                vec![]
            }
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
                    WITAstType::Function { inputs, outputs } => {
                        let interpreter: WITInterpreter = adapter_instructions.try_into()?;

                        let raw_import = create_raw_import(
                            wit_instance.clone(),
                            interpreter,
                            import_namespace.to_string(),
                            import_name.to_string(),
                        );
                        let wit_import =
                            dyn_func_from_raw_import(inputs.clone(), outputs.clone(), raw_import);

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

        // TODO: refactor this
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
