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
use super::{IType, IRecordType, IFunctionArg, IValue, WValue};
use crate::Result;
use crate::FCEModuleConfig;

use fce_wit_interfaces::FCEWITInterfaces;
use fce_wit_parser::extract_wit;
use wasmer_core::Instance as WasmerInstance;
use wasmer_core::import::Namespace;
use wasmer_runtime::compile;
use wasmer_runtime::ImportObject;
use wasmer_wit::interpreter::Interpreter;

use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::sync::Arc;

type WITInterpreter =
    Interpreter<WITInstance, WITExport, WITFunction, WITMemory, WITMemoryView<'static>>;

#[derive(Clone)]
pub(super) struct WITModuleFunc {
    interpreter: Arc<WITInterpreter>,
    pub(super) arguments: Vec<IFunctionArg>,
    pub(super) output_types: Vec<IType>,
}

#[derive(Clone)]
pub(super) struct Callable {
    pub(super) wit_instance: Arc<WITInstance>,
    pub(super) wit_module_func: WITModuleFunc,
}

impl Callable {
    pub fn call(&mut self, args: &[IValue]) -> Result<Vec<IValue>> {
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
    // that internally keep pointer to it.
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
    export_funcs: HashMap<String, Arc<Callable>>,

    // TODO: save refs instead of copies
    record_types: Vec<(u64, IRecordType)>,
}

impl FCEModule {
    pub(crate) fn new(
        wasm_bytes: &[u8],
        config: FCEModuleConfig,
        modules: &HashMap<String, FCEModule>,
    ) -> Result<Self> {
        let wasmer_module = compile(&wasm_bytes)?;
        let wit = extract_wit(&wasmer_module)?;
        let fce_wit = FCEWITInterfaces::new(wit);

        let mut wit_instance = Arc::new_uninit();
        let import_object = Self::adjust_wit_imports(&fce_wit, wit_instance.clone())?;

        let mut wasi_import_object = wasmer_wasi::generate_import_object_for_version(
            config.wasi_version,
            vec![],
            config.wasi_envs.clone(),
            config.wasi_preopened_files.clone(),
            config.wasi_mapped_dirs.clone(),
        );

        wasi_import_object.extend(import_object.clone());
        wasi_import_object.extend(config.imports.clone());

        let wasmer_instance = wasmer_module.instantiate(&wasi_import_object)?;
        let wit_instance = unsafe {
            // get_mut_unchecked here is safe because currently only this modules have reference to
            // it and the environment is single-threaded
            *Arc::get_mut_unchecked(&mut wit_instance) =
                MaybeUninit::new(WITInstance::new(&wasmer_instance, &fce_wit, modules)?);
            std::mem::transmute::<_, Arc<WITInstance>>(wit_instance)
        };

        let export_funcs = Self::instantiate_wit_exports(&wit_instance, &fce_wit)?;
        let record_types = Self::extract_export_record_types(&export_funcs, &wit_instance)?;

        // call _start to populate the WASI state of the module
        #[rustfmt::skip]
        if let Ok(start_func) = wasmer_instance.exports.get::<wasmer_runtime::Func<'_, (), ()>>("_start") {
            start_func.call()?;
        }

        Ok(Self {
            wasmer_instance: Box::new(wasmer_instance),
            import_object,
            host_import_object: config.imports,
            export_funcs,
            record_types,
        })
    }

    pub(crate) fn call(&mut self, function_name: &str, args: &[IValue]) -> Result<Vec<IValue>> {
        match self.export_funcs.get_mut(function_name) {
            Some(func) => Arc::make_mut(func).call(args),
            None => Err(FCEError::NoSuchFunction(format!(
                "{} hasn't been found while calling",
                function_name
            ))),
        }
    }

    pub(crate) fn get_exports_signatures(
        &self,
    ) -> impl Iterator<Item = (&String, &Vec<IFunctionArg>, &Vec<IType>)> {
        self.export_funcs.iter().map(|(func_name, func)| {
            (
                func_name,
                &func.wit_module_func.arguments,
                &func.wit_module_func.output_types,
            )
        })
    }

    pub(crate) fn get_export_record_types(&self) -> impl Iterator<Item = &(u64, IRecordType)> {
        self.record_types.iter()
    }

    pub(crate) fn get_wasi_state(&mut self) -> &wasmer_wasi::state::WasiState {
        unsafe { wasmer_wasi::state::get_wasi_state(self.wasmer_instance.context_mut()) }
    }

    // TODO: change the cloning Callable behaviour after changes of Wasmer API
    pub(super) fn get_callable(&self, function_name: &str) -> Result<Arc<Callable>> {
        match self.export_funcs.get(function_name) {
            Some(func) => Ok(func.clone()),
            None => Err(FCEError::NoSuchFunction(format!(
                "{} hasn't been found while calling",
                function_name
            ))),
        }
    }

    fn instantiate_wit_exports(
        wit_instance: &Arc<WITInstance>,
        wit: &FCEWITInterfaces<'_>,
    ) -> Result<HashMap<String, Arc<Callable>>> {
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
                        arguments,
                        output_types,
                    } => {
                        let interpreter: WITInterpreter = adapter_instructions.try_into()?;
                        let wit_module_func = WITModuleFunc {
                            interpreter: Arc::new(interpreter),
                            arguments: arguments.clone(),
                            output_types: output_types.clone(),
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
            .collect::<Result<HashMap<String, Arc<Callable>>>>()
    }

    // this function deals only with import functions that have an adaptor implementation
    fn adjust_wit_imports(
        wit: &FCEWITInterfaces<'_>,
        wit_instance: Arc<MaybeUninit<WITInstance>>,
    ) -> Result<ImportObject> {
        use fce_wit_interfaces::WITAstType;
        use wasmer_core::typed_func::DynamicFunc;
        use wasmer_core::vm::Ctx;

        // returns function that will be called from imports of Wasmer module
        fn dyn_func_from_raw_import<'a, 'b, F>(
            inputs: impl Iterator<Item = &'a IType>,
            outputs: impl Iterator<Item = &'b IType>,
            raw_import: F,
        ) -> DynamicFunc<'static>
        where
            F: Fn(&mut Ctx, &[WValue]) -> Vec<WValue> + 'static,
        {
            use wasmer_core::types::FuncSig;
            use super::type_converters::itype_to_wtype;

            let inputs = inputs.map(itype_to_wtype).collect::<Vec<_>>();
            let outputs = outputs.map(itype_to_wtype).collect::<Vec<_>>();
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
                use wasmer_wit::interpreter::stack::Stackable;

                use super::type_converters::wval_to_ival;
                use super::type_converters::ival_to_wval;

                log::trace!(
                    "raw import for {}.{} called with {:?}\n",
                    import_namespace,
                    import_name,
                    inputs
                );

                // copy here because otherwise wit_instance will be consumed by the closure
                let wit_instance_callable = wit_instance.clone();
                let wit_inputs = inputs.iter().map(wval_to_ival).collect::<Vec<_>>();
                let outputs = unsafe {
                    // error here will be propagated by the special error instruction
                    interpreter.run(
                        &wit_inputs,
                        Arc::make_mut(&mut wit_instance_callable.assume_init()),
                    )
                };

                log::trace!(
                    "\nraw import for {}.{} finished",
                    import_namespace,
                    import_name
                );

                // TODO: optimize by prevent copying stack values
                outputs
                    .unwrap_or_default()
                    .as_slice()
                    .iter()
                    .map(ival_to_wval)
                    .collect::<Vec<_>>()
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
                    WITAstType::Function {
                        arguments,
                        output_types,
                    } => {
                        let interpreter: WITInterpreter = adapter_instructions.try_into()?;

                        let raw_import = create_raw_import(
                            wit_instance.clone(),
                            interpreter,
                            import_namespace.to_string(),
                            import_name.to_string(),
                        );

                        let wit_import = dyn_func_from_raw_import(
                            arguments.iter().map(|IFunctionArg { ty, .. }| ty),
                            output_types.iter(),
                            raw_import,
                        );

                        Ok((import_namespace.to_string(), (*import_name, wit_import)))
                    }
                    _ => Err(FCEError::IncorrectWIT(format!(
                        "type with idx = {} isn't a function type",
                        adapter_function_type
                    ))),
                }
            })
            .collect::<Result<multimap::MultiMap<_, _>>>()?;

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

    fn extract_export_record_types(
        export_funcs: &HashMap<String, Arc<Callable>>,
        wit_instance: &Arc<WITInstance>,
    ) -> Result<Vec<(u64, IRecordType)>> {
        fn handle_record_type(
            record_type_id: u64,
            wit_instance: &Arc<WITInstance>,
            export_record_types: &mut Vec<(u64, IRecordType)>,
        ) -> Result<()> {
            use wasmer_wit::interpreter::wasm::structures::Instance;

            let record_type = wit_instance
                .wit_record_by_id(record_type_id)
                .ok_or_else(|| {
                    FCEError::WasmerResolveError(format!(
                        "record type with type id {} not found",
                        record_type_id
                    ))
                })?;
            export_record_types.push((record_type_id, record_type.clone()));

            for field in record_type.fields.iter() {
                if let IType::Record(record_type_id) = &field.ty {
                    handle_record_type(*record_type_id, wit_instance, export_record_types)?;
                }
            }

            Ok(())
        }

        let export_record_ids = export_funcs
            .iter()
            .flat_map(|(_, ref mut callable)| {
                callable
                    .wit_module_func
                    .arguments
                    .iter()
                    .map(|arg| &arg.ty)
                    .chain(callable.wit_module_func.output_types.iter())
            })
            .filter_map(|itype| match itype {
                IType::Record(record_type_id) => Some(record_type_id),
                _ => None,
            });

        let mut export_record_types = Vec::new();
        for record_type_id in export_record_ids {
            handle_record_type(*record_type_id, wit_instance, &mut export_record_types)?;
        }

        Ok(export_record_types)
    }
}
