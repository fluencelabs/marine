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
use super::RecordTypes;
use crate::FCEResult;
use crate::FCEModuleConfig;

use fce_wit_interfaces::FCEWITInterfaces;
use fce_wit_parser::extract_wit_from_module;
use fce_utils::SharedString;
use wasmer_core::Instance as WasmerInstance;
use wasmer_core::import::Namespace;
use wasmer_runtime::compile;
use wasmer_runtime::ImportObject;
use wasmer_wit::interpreter::Interpreter;

use serde::Serialize;
use serde::Deserialize;

use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::rc::Rc;

type WITInterpreter =
    Interpreter<WITInstance, WITExport, WITFunction, WITMemory, WITMemoryView<'static>>;

#[derive(Clone)]
pub(super) struct WITModuleFunc {
    interpreter: Arc<WITInterpreter>,
    pub(super) arguments: Rc<Vec<IFunctionArg>>,
    pub(super) output_types: Rc<Vec<IType>>,
}

/// Represent a function type inside FCE module.
#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct FCEFunctionSignature {
    pub name: Rc<String>,
    pub arguments: Rc<Vec<IFunctionArg>>,
    pub outputs: Rc<Vec<IType>>,
}

#[derive(Clone)]
pub(super) struct Callable {
    pub(super) wit_instance: Arc<WITInstance>,
    pub(super) wit_module_func: WITModuleFunc,
}

impl Callable {
    pub fn call(&mut self, args: &[IValue]) -> FCEResult<Vec<IValue>> {
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

type ExportFunctions = HashMap<SharedString, Rc<Callable>>;

pub(crate) struct FCEModule {
    // wasmer_instance is needed because WITInstance contains dynamic functions
    // that internally keep pointer to it.
    #[allow(unused)]
    wasmer_instance: Box<WasmerInstance>,

    // import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    #[allow(unused)]
    wit_import_object: ImportObject,

    // host_import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    #[allow(unused)]
    host_import_object: ImportObject,

    // host_closures_import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    #[allow(unused)]
    host_closures_import_object: ImportObject,

    // TODO: replace with dyn Trait
    export_funcs: ExportFunctions,

    // TODO: save refs instead copying of a record types HashMap.
    /// Record types used in exported functions as arguments or return values.
    export_record_types: RecordTypes,
}

impl FCEModule {
    pub(crate) fn new(
        wasm_bytes: &[u8],
        config: FCEModuleConfig,
        modules: &HashMap<String, FCEModule>,
    ) -> FCEResult<Self> {
        let wasmer_module = compile(&wasm_bytes)?;
        crate::misc::check_sdk_version(&wasmer_module)?;

        let wit = extract_wit_from_module(&wasmer_module)?;
        let fce_wit = FCEWITInterfaces::new(wit);

        let mut wit_instance = Arc::new_uninit();
        let wit_import_object = Self::adjust_wit_imports(&fce_wit, wit_instance.clone())?;
        let raw_imports = config.raw_imports.clone();
        let (wasi_import_object, host_closures_import_object) =
            Self::create_import_objects(config, &fce_wit, wit_import_object.clone())?;

        let wasmer_instance = wasmer_module.instantiate(&wasi_import_object)?;
        let wit_instance = unsafe {
            // get_mut_unchecked here is safe because currently only this modules have reference to
            // it and the environment is single-threaded
            *Arc::get_mut_unchecked(&mut wit_instance) =
                MaybeUninit::new(WITInstance::new(&wasmer_instance, &fce_wit, modules)?);
            std::mem::transmute::<_, Arc<WITInstance>>(wit_instance)
        };

        let export_funcs = Self::instantiate_wit_exports(&wit_instance, &fce_wit)?;
        let export_record_types = Self::extract_export_record_types(&export_funcs, &wit_instance)?;

        // call _start to populate the WASI state of the module
        #[rustfmt::skip]
        if let Ok(start_func) = wasmer_instance.exports.get::<wasmer_runtime::Func<'_, (), ()>>("_start") {
            start_func.call()?;
        }

        Ok(Self {
            wasmer_instance: Box::new(wasmer_instance),
            wit_import_object,
            host_import_object: raw_imports,
            host_closures_import_object,
            export_funcs,
            export_record_types,
        })
    }

    pub(crate) fn call(&mut self, function_name: &str, args: &[IValue]) -> FCEResult<Vec<IValue>> {
        self.export_funcs.get_mut(function_name).map_or_else(
            || {
                Err(FCEError::NoSuchFunction(format!(
                    "{} hasn't been found while calling",
                    function_name
                )))
            },
            |func| Rc::make_mut(func).call(args),
        )
    }

    pub(crate) fn get_exports_signatures(&self) -> impl Iterator<Item = FCEFunctionSignature> + '_ {
        self.export_funcs
            .iter()
            .map(|(func_name, func)| FCEFunctionSignature {
                name: func_name.0.clone(),
                arguments: func.wit_module_func.arguments.clone(),
                outputs: func.wit_module_func.output_types.clone(),
            })
    }

    pub(crate) fn export_record_types(&self) -> &RecordTypes {
        &self.export_record_types
    }

    pub(crate) fn export_record_type_by_id(&self, record_type: u64) -> Option<&Rc<IRecordType>> {
        self.export_record_types.get(&record_type)
    }

    pub(crate) fn get_wasi_state(&mut self) -> &wasmer_wasi::state::WasiState {
        unsafe { wasmer_wasi::state::get_wasi_state(self.wasmer_instance.context_mut()) }
    }

    // TODO: change the cloning Callable behaviour after changes of Wasmer API
    pub(super) fn get_callable(&self, function_name: &str) -> FCEResult<Rc<Callable>> {
        match self.export_funcs.get(function_name) {
            Some(func) => Ok(func.clone()),
            None => Err(FCEError::NoSuchFunction(format!(
                "{} hasn't been found while calling",
                function_name
            ))),
        }
    }

    fn create_import_objects(
        config: FCEModuleConfig,
        fce_wit: &FCEWITInterfaces<'_>,
        wit_import_object: ImportObject,
    ) -> FCEResult<(ImportObject, ImportObject)> {
        use crate::host_imports::create_host_import_func;

        let wasi_envs = config
            .wasi_envs
            .into_iter()
            .map(|(mut left, right)| {
                left.push(61); // 61 is ASCII code of '='
                left.extend(right);
                left
            })
            .collect::<Vec<_>>();
        let wasi_preopened_files = config.wasi_preopened_files.into_iter().collect::<Vec<_>>();
        let wasi_mapped_dirs = config.wasi_mapped_dirs.into_iter().collect::<Vec<_>>();

        let mut wasi_import_object = wasmer_wasi::generate_import_object_for_version(
            config.wasi_version,
            vec![],
            wasi_envs,
            wasi_preopened_files,
            wasi_mapped_dirs,
        )
        .map_err(|e| FCEError::PrepareError(e))?;

        let mut host_closures_namespace = Namespace::new();
        let record_types = fce_wit
            .record_types()
            .map(|(id, r)| (id, r.clone()))
            .collect::<HashMap<_, _>>();
        let record_types = Rc::new(record_types);

        for (import_name, descriptor) in config.host_imports {
            let host_import = create_host_import_func(descriptor, record_types.clone());
            host_closures_namespace.insert(import_name, host_import);
        }
        let mut host_closures_import_object = ImportObject::new();
        host_closures_import_object.register("host", host_closures_namespace);

        wasi_import_object.extend(wit_import_object);
        wasi_import_object.extend(config.raw_imports);
        wasi_import_object.extend(host_closures_import_object.clone());

        Ok((wasi_import_object, host_closures_import_object))
    }

    fn instantiate_wit_exports(
        wit_instance: &Arc<WITInstance>,
        wit: &FCEWITInterfaces<'_>,
    ) -> FCEResult<ExportFunctions> {
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
                        let interpreter: WITInterpreter =
                            adapter_instructions.clone().try_into()?;
                        let wit_module_func = WITModuleFunc {
                            interpreter: Arc::new(interpreter),
                            arguments: arguments.clone(),
                            output_types: output_types.clone(),
                        };

                        let shared_string = SharedString(Rc::new(export_function_name.to_string()));
                        let callable = Rc::new(Callable {
                            wit_instance: wit_instance.clone(),
                            wit_module_func,
                        });

                        Ok((shared_string, callable))
                    }
                    _ => Err(FCEError::IncorrectWIT(format!(
                        "type with idx = {} isn't a function type",
                        adapter_function_type
                    ))),
                }
            })
            .collect::<FCEResult<ExportFunctions>>()
    }

    // this function deals only with import functions that have an adaptor implementation
    fn adjust_wit_imports(
        wit: &FCEWITInterfaces<'_>,
        wit_instance: Arc<MaybeUninit<WITInstance>>,
    ) -> FCEResult<ImportObject> {
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
                        let interpreter: WITInterpreter =
                            adapter_instructions.clone().try_into()?;

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
            .collect::<FCEResult<multimap::MultiMap<_, _>>>()?;

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

    // TODO : move it to a separate crate
    fn extract_export_record_types(
        export_funcs: &ExportFunctions,
        wit_instance: &Arc<WITInstance>,
    ) -> FCEResult<RecordTypes> {
        use fce_wit_generator::TYPE_RESOLVE_RECURSION_LIMIT;
        use FCEError::WasmerResolveError;

        fn handle_itype(
            itype: &IType,
            wit_instance: &Arc<WITInstance>,
            export_record_types: &mut RecordTypes,
            recursion_level: u32,
        ) -> FCEResult<()> {
            use wasmer_wit::interpreter::wasm::structures::Instance;

            if recursion_level > TYPE_RESOLVE_RECURSION_LIMIT {
                return Err(WasmerResolveError(String::from(
                    "mailformed module: a record contains more recursion level then allowed",
                )));
            }

            fn handle_record_type(
                record_type_id: u64,
                wit_instance: &Arc<WITInstance>,
                export_record_types: &mut RecordTypes,
                recursion_level: u32,
            ) -> FCEResult<()> {
                let record_type =
                    wit_instance
                        .wit_record_by_id(record_type_id)
                        .ok_or_else(|| {
                            WasmerResolveError(format!(
                                "record type with type id {} not found",
                                record_type_id
                            ))
                        })?;
                export_record_types.insert(record_type_id, record_type.clone());

                for field in record_type.fields.iter() {
                    handle_itype(
                        &field.ty,
                        wit_instance,
                        export_record_types,
                        recursion_level + 1,
                    )?;
                }

                Ok(())
            }

            match itype {
                IType::Record(record_type_id) => handle_record_type(
                    *record_type_id,
                    wit_instance,
                    export_record_types,
                    recursion_level + 1,
                )?,
                IType::Array(array_ty) => handle_itype(
                    array_ty,
                    wit_instance,
                    export_record_types,
                    recursion_level + 1,
                )?,
                _ => {}
            }

            Ok(())
        }

        let mut export_record_types = HashMap::new();

        let itypes = export_funcs.iter().flat_map(|(_, ref mut callable)| {
            callable
                .wit_module_func
                .arguments
                .iter()
                .map(|arg| &arg.ty)
                .chain(callable.wit_module_func.output_types.iter())
        });

        for itype in itypes {
            handle_itype(itype, wit_instance, &mut export_record_types, 0)?;
        }

        Ok(export_record_types)
    }
}
