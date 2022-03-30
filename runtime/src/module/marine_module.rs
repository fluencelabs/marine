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
use super::{IType, IRecordType, IFunctionArg, IValue, WValue};
use crate::MResult;
use crate::MModuleConfig;

use marine_it_interfaces::MITInterfaces;
use marine_it_parser::extract_it_from_module;
use marine_utils::SharedString;
use wasmer_core::Instance as WasmerInstance;
use wasmer_core::import::Namespace;
use wasmer_runtime::compile;
use wasmer_runtime::ImportObject;
use wasmer_it::interpreter::Interpreter;

use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::rc::Rc;

type ITInterpreter =
    Interpreter<ITInstance, ITExport, WITFunction, WITMemory, WITMemoryView<'static>>;

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

    // import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    #[allow(unused)]
    it_import_object: ImportObject,

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
    export_record_types: MRecordTypes,
}

impl MModule {
    pub(crate) fn new(
        name: &str,
        wasm_bytes: &[u8],
        config: MModuleConfig,
        modules: &HashMap<String, MModule>,
    ) -> MResult<Self> {
        let wasmer_module = compile(wasm_bytes)?;
        crate::misc::check_sdk_version(name, &wasmer_module)?;

        let it = extract_it_from_module(&wasmer_module)?;
        crate::misc::check_it_version(name, &it.version)?;

        let mit = MITInterfaces::new(it);

        let mut wit_instance = Arc::new_uninit();
        let wit_import_object = Self::adjust_wit_imports(&mit, wit_instance.clone())?;
        let raw_imports = config.raw_imports.clone();
        let (wasi_import_object, host_closures_import_object) =
            Self::create_import_objects(config, &mit, wit_import_object.clone())?;

        let wasmer_instance = wasmer_module.instantiate(&wasi_import_object)?;
        let it_instance = unsafe {
            // get_mut_unchecked here is safe because currently only this modules have reference to
            // it and the environment is single-threaded
            *Arc::get_mut_unchecked(&mut wit_instance) =
                MaybeUninit::new(ITInstance::new(&wasmer_instance, name, &mit, modules)?);
            std::mem::transmute::<_, Arc<ITInstance>>(wit_instance)
        };

        let (export_funcs, export_record_types) = Self::instantiate_exports(&it_instance, &mit)?;

        // call _start to populate the WASI state of the module
        #[rustfmt::skip]
        if let Ok(start_func) = wasmer_instance.exports.get::<wasmer_runtime::Func<'_, (), ()>>("_start") {
            start_func.call()?;
        }

        Ok(Self {
            wasmer_instance: Box::new(wasmer_instance),
            it_import_object: wit_import_object,
            host_import_object: raw_imports,
            host_closures_import_object,
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

    pub(crate) fn get_wasi_state(&mut self) -> &wasmer_wasi::state::WasiState {
        unsafe { wasmer_wasi::state::get_wasi_state(self.wasmer_instance.context_mut()) }
    }

    /// Returns Wasm linear memory size that this module consumes in bytes.
    pub(crate) fn memory_size(&self) -> usize {
        // Wasmer 0.17.1 supports only one memory
        const MEMORY_INDEX: u32 = 0;

        let pages = self.wasmer_instance.context().memory(MEMORY_INDEX).size();
        pages.bytes().0
    }

    /// Returns max Wasm linear memory size that this module consumes in bytes.
    pub(crate) fn max_memory_size(&self) -> Option<usize> {
        // Wasmer 0.17.1 supports only one memory
        const MEMORY_INDEX: u32 = 0;

        let maybe_pages = self
            .wasmer_instance
            .context()
            .memory(MEMORY_INDEX)
            .descriptor()
            .maximum;
        maybe_pages.map(|pages| pages.bytes().0)
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

    fn create_import_objects(
        config: MModuleConfig,
        mit: &MITInterfaces<'_>,
        wit_import_object: ImportObject,
    ) -> MResult<(ImportObject, ImportObject)> {
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
        .map_err(MError::WASIPrepareError)?;

        let mut host_closures_namespace = Namespace::new();
        let record_types = mit
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

    // this function deals only with import functions that have an adaptor implementation
    fn adjust_wit_imports(
        wit: &MITInterfaces<'_>,
        wit_instance: Arc<MaybeUninit<ITInstance>>,
    ) -> MResult<ImportObject> {
        use marine_it_interfaces::ITAstType;
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

        // creates a closure that is represent a IT module import
        fn create_raw_import(
            wit_instance: Arc<MaybeUninit<ITInstance>>,
            interpreter: ITInterpreter,
            import_namespace: String,
            import_name: String,
        ) -> impl Fn(&mut Ctx, &[WValue]) -> Vec<WValue> + 'static {
            move |_: &mut Ctx, inputs: &[WValue]| -> Vec<WValue> {
                use wasmer_it::interpreter::stack::Stackable;

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
                wit.imports_by_type(*core_function_type)
                    .map(|import| (adapter_function_type, import))
            })
            .flat_map(|(adapter_function_type, import_function_names)| {
                import_function_names
                    .iter()
                    .map(move |import_function_name| (*adapter_function_type, import_function_name))
            })
            .map(|(adapter_function_type, (import_namespace, import_name))| {
                let adapter_instructions = wit.adapter_by_type_r(adapter_function_type)?;
                let wit_type = wit.type_by_idx_r(adapter_function_type)?;

                match wit_type {
                    ITAstType::Function {
                        arguments,
                        output_types,
                    } => {
                        let interpreter: ITInterpreter = adapter_instructions.clone().try_into()?;

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
                    _ => Err(MError::IncorrectWIT(format!(
                        "type with idx = {} isn't a function type",
                        adapter_function_type
                    ))),
                }
            })
            .collect::<MResult<multimap::MultiMap<_, _>>>()?;

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
