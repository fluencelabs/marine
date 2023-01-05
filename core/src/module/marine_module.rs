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

use std::borrow::{BorrowMut};
use super::wit_prelude::*;
use super::MFunctionSignature;
use super::MRecordTypes;
use super::{IType, IRecordType, IFunctionArg, IValue, WValue};
use crate::{HostImportDescriptor, MResult};
use crate::MModuleConfig;

use marine_wasm_backend_traits::{
    AsContextMut, DelayedContextLifetime, Function, WasiState, WasiVersion, WasmBackend,
};
use marine_wasm_backend_traits::Module;
use marine_wasm_backend_traits::Instance;
use marine_wasm_backend_traits::Imports;
use marine_wasm_backend_traits::WasiImplementation;

use marine_it_interfaces::MITInterfaces;
use marine_it_parser::extract_it_from_module;
use marine_utils::SharedString;
//use wasmer_core::Instance as WasmerInstance;
//use wasmer_core::import::Namespace;
//use wasmer_runtime::compile;
//use wasmer_runtime::ImportObject;
use wasmer_it::interpreter::Interpreter;

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::sync::Arc;
//use wasmer_core::types::FuncSig;
use marine_wasm_backend_traits::FuncSig;

const START_FUNC: &str = "_start";
const INITIALIZE_FUNC: &str = "_initialize";

type ITInterpreter<WB> = Interpreter<
    ITInstance<WB>,
    ITExport,
    WITFunction<WB>,
    <WB as WasmBackend>::Memory,
    <WB as WasmBackend>::MemoryView,
    DelayedContextLifetime<WB>,
>;

#[derive(Clone)]
pub(super) struct ITModuleFunc<WB: WasmBackend> {
    interpreter: Arc<ITInterpreter<WB>>,
    pub(super) arguments: Arc<Vec<IFunctionArg>>,
    pub(super) output_types: Arc<Vec<IType>>,
}

#[derive(Clone)]
pub(super) struct Callable<WB: WasmBackend> {
    pub(super) it_instance: Arc<ITInstance<WB>>,
    pub(super) it_module_func: ITModuleFunc<WB>,
}

impl<WB: WasmBackend> Callable<WB> {
    pub fn call(
        &mut self,
        store: &mut <WB as WasmBackend>::ContextMut<'_>,
        args: &[IValue],
    ) -> MResult<Vec<IValue>> {
        use wasmer_it::interpreter::stack::Stackable;

        let result = self
            .it_module_func
            .interpreter
            .run(args, Arc::make_mut(&mut self.it_instance), store)?
            .as_slice()
            .to_owned();

        Ok(result)
    }
}

type ExportFunctions<WB> = HashMap<SharedString, Arc<Callable<WB>>>;

pub(crate) struct MModule<WB: WasmBackend> {
    // wasmer_instance is needed because WITInstance contains dynamic functions
    // that internally keep pointer to it.
    #[allow(unused)]
    wasmer_instance: Box<<WB as WasmBackend>::Instance>,

    // import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    //#[allow(unused)]
    //it_import_object: <WB as WasmBackend>::ImportObject,

    // host_import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    // #[allow(unused)]
    //host_import_object: <WB as WasmBackend>::ImportObject,

    // host_closures_import_object is needed because ImportObject::extend doesn't really deep copy
    // imports, so we need to store imports of this module to prevent their removing.
    //#[allow(unused)]
    //host_closures_import_object: <WB as WasmBackend>::ImportObject,

    // TODO: replace with dyn Trait
    export_funcs: ExportFunctions<WB>,

    // TODO: save refs instead copying of a record types HashMap.
    /// Record types used in exported functions as arguments or return values.
    export_record_types: MRecordTypes,
}

impl<WB: WasmBackend> MModule<WB> {
    pub(crate) fn new(
        name: &str,
        store: &mut <WB as WasmBackend>::Store,
        wasm_bytes: &[u8],
        config: MModuleConfig<WB>,
        modules: &HashMap<String, MModule<WB>>,
    ) -> MResult<Self> {
        //let store = Rc::new(RefCell::new(<WB as WasmBackend>::Store::new(backend)));
        //let mut store = store_container.deref().borrow_mut();

        let wasmer_module = WB::compile(store, wasm_bytes)?;
        crate::misc::check_sdk_version::<WB>(name.to_string(), &wasmer_module)?;

        let it = extract_it_from_module::<WB>(&wasmer_module)?;
        crate::misc::check_it_version(name, &it.version)?;

        let mit = MITInterfaces::new(it);

        let mut wit_instance = Arc::new_uninit();
        let mut linker = <WB as WasmBackend>::Imports::new(store);

        let MModuleConfig {
            raw_imports,
            host_imports,
            wasi_version,
            wasi_envs,
            wasi_preopened_files,
            wasi_mapped_dirs,
            ..
        } = config;

        Self::add_wit_imports(store, &mut linker, &mit, wit_instance.clone())?;
        Self::add_wasi_imports(
            store,
            &mut linker,
            wasi_version,
            wasi_envs,
            wasi_preopened_files,
            wasi_mapped_dirs,
        )?;
        Self::add_host_imports(store, &mut linker, raw_imports, host_imports, &mit)?;

        let wasmer_instance = wasmer_module.instantiate(store, &linker)?;
        let it_instance = unsafe {
            // get_mut_unchecked here is safe because currently only this modules have reference to
            // it and the environment is single-threaded
            *Arc::get_mut_unchecked(&mut wit_instance) = MaybeUninit::new(ITInstance::new(
                &wasmer_instance,
                store,
                name,
                &mit,
                modules,
            )?); // todo why is deref_mut needed instead of &mut?
            std::mem::transmute::<_, Arc<ITInstance<WB>>>(wit_instance)
        };

        let (export_funcs, export_record_types) = Self::instantiate_exports(&it_instance, &mit)?;

        // call _initialize to populate the WASI state of the module
        #[rustfmt::skip]
        if let Ok(initialize_func) = wasmer_instance.get_function(store, INITIALIZE_FUNC) {
            initialize_func.call(store, &[])?;
        }
        // call _start to call module's main function
        #[rustfmt::skip]
        if let Ok(start_func) = wasmer_instance.get_function(store, START_FUNC) {
            start_func.call(store, &[])?;
        }

        Ok(Self {
            wasmer_instance: Box::new(wasmer_instance),
            //it_import_object: wit_import_object,
            //host_import_object: raw_imports,
            //host_closures_import_object,
            export_funcs,
            export_record_types,
        })
    }

    pub(crate) fn call(
        &mut self,
        store: &mut <WB as WasmBackend>::ContextMut<'_>,
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
            |func| Arc::make_mut(func).call(store, args),
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

    pub(crate) fn export_record_type_by_id(&self, record_type: u64) -> Option<&Arc<IRecordType>> {
        self.export_record_types.get(&record_type)
    }

    pub(crate) fn get_wasi_state<'s>(&'s mut self) -> Box<dyn WasiState + 's> {
        <WB as WasmBackend>::Wasi::get_wasi_state(self.wasmer_instance.borrow_mut())
    }

    /// Returns Wasm linear memory size that this module consumes in bytes.
    pub(crate) fn memory_size(&self) -> usize {
        // TODO: Add a method to the trait
        1024 * 1014
    }

    /// Returns max Wasm linear memory size that this module could consume in bytes.
    pub(crate) fn max_memory_size(&self) -> Option<usize> {
        // TODO: Add a method to the trait
        Some(1024 * 1014)
    }

    // TODO: change the cloning Callable behaviour after changes of Wasmer API
    pub(super) fn get_callable(
        &self,
        module_name: &str,
        function_name: &str,
    ) -> MResult<Arc<Callable<WB>>> {
        match self.export_funcs.get(function_name) {
            Some(func) => Ok(func.clone()),
            None => Err(MError::NoSuchFunction(
                module_name.to_string(),
                function_name.to_string(),
            )),
        }
    }

    fn add_wasi_imports(
        store: &mut <WB as WasmBackend>::Store,
        linker: &mut <WB as WasmBackend>::Imports,
        wasi_version: WasiVersion,
        wasi_envs: HashMap<Vec<u8>, Vec<u8>>,
        wasi_preopened_files: HashSet<PathBuf>,
        wasi_mapped_dirs: HashMap<String, PathBuf>,
    ) -> MResult<()> {
        let wasi_envs = wasi_envs
            .into_iter()
            .map(|(mut left, right)| {
                left.push(61); // 61 is ASCII code of '=' // todo remove or move to backend impl
                left.extend(right);
                left
            })
            .collect::<Vec<_>>();
        let wasi_preopened_files = wasi_preopened_files.into_iter().collect::<Vec<_>>();
        let wasi_mapped_dirs = wasi_mapped_dirs.into_iter().collect::<Vec<_>>();

        <WB as WasmBackend>::Wasi::register_in_linker(
            &mut store.as_context_mut(),
            linker,
            wasi_version,
            vec![],
            wasi_envs,
            wasi_preopened_files,
            wasi_mapped_dirs,
        )
        .map_err(MError::WASIPrepareError)?;

        Ok(())
    }

    fn add_host_imports(
        store: &mut <WB as WasmBackend>::Store,
        linker: &mut <WB as WasmBackend>::Imports,
        raw_imports: HashMap<String, <WB as WasmBackend>::Function>,
        host_imports: HashMap<String, HostImportDescriptor<WB>>,
        mit: &MITInterfaces<'_>,
    ) -> MResult<()> {
        use crate::host_imports::create_host_import_func;

        let record_types = mit
            .record_types()
            .map(|(id, r)| (id, r.clone()))
            .collect::<HashMap<_, _>>();
        let record_types = Arc::new(record_types);

        let host_imports = host_imports.into_iter().map(|(import_name, descriptor)| {
            let func = create_host_import_func::<WB>(store, descriptor, record_types.clone());
            (import_name, func)
        });

        linker.register("host", raw_imports.into_iter().chain(host_imports));

        Ok(())
    }

    // this function deals only with import functions that have an adaptor implementation
    fn add_wit_imports(
        store: &mut <WB as WasmBackend>::Store,
        linker: &mut <WB as WasmBackend>::Imports,
        wit: &MITInterfaces<'_>,
        wit_instance: Arc<MaybeUninit<ITInstance<WB>>>,
    ) -> MResult<()> {
        use marine_it_interfaces::ITAstType;

        // returns function that will be called from imports of Wasmer module
        fn func_from_raw_import<'a, 'b, F, WB, I1, I2>(
            store: &mut <WB as WasmBackend>::Store,
            inputs: I1,
            outputs: I2,
            raw_import: F,
        ) -> <WB as WasmBackend>::Function
        where
            F: for<'c> Fn(<WB as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
                + Sync
                + Send
                + 'static,
            WB: WasmBackend,
            I1: Iterator<Item = &'a IType>,
            I2: Iterator<Item = &'b IType>,
        {
            //use wasmer_core::types::FuncSig;
            use super::type_converters::itype_to_wtype;

            let inputs = inputs.map(itype_to_wtype).collect::<Vec<_>>();
            let outputs = outputs.map(itype_to_wtype).collect::<Vec<_>>();
            <WB as WasmBackend>::Function::new_with_ctx(
                &mut store.as_context_mut(),
                FuncSig::new(inputs, outputs),
                raw_import,
            )
        }

        // creates a closure that is represent a IT module import
        fn create_raw_import<WB: WasmBackend>(
            wit_instance: Arc<MaybeUninit<ITInstance<WB>>>,
            interpreter: ITInterpreter<WB>,
            import_namespace: String,
            import_name: String,
        ) -> impl for<'c> Fn(<WB as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
               + Sync
               + Send
               + 'static {
            move |mut ctx: <WB as WasmBackend>::Caller<'_>, inputs: &[WValue]| -> Vec<WValue> {
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
                        &mut ctx.as_context_mut(),
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
                        let interpreter: ITInterpreter<WB> =
                            adapter_instructions.clone().try_into()?;

                        let raw_import = create_raw_import(
                            wit_instance.clone(),
                            interpreter,
                            import_namespace.to_string(),
                            import_name.to_string(),
                        );

                        let wit_import = func_from_raw_import::<_, WB, _, _>(
                            store,
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

        for (namespace_name, funcs) in wit_import_funcs.into_iter() {
            let funcs = funcs.into_iter().map(|(name, f)| (name.to_string(), f));
            linker.register(namespace_name, funcs);
        }

        Ok(())
    }

    fn instantiate_exports(
        it_instance: &Arc<ITInstance<WB>>,
        mit: &MITInterfaces<'_>,
    ) -> MResult<(ExportFunctions<WB>, MRecordTypes)> {
        let module_interface = marine_module_interface::it_interface::get_interface(mit)?;

        let export_funcs = module_interface
            .function_signatures
            .into_iter()
            .map(|sign| {
                let adapter_instructions = mit.adapter_by_type_r(sign.adapter_function_type)?;

                let interpreter: ITInterpreter<WB> = adapter_instructions.clone().try_into()?;
                let it_module_func = ITModuleFunc {
                    interpreter: Arc::new(interpreter),
                    arguments: sign.arguments.clone(),
                    output_types: sign.outputs.clone(),
                };

                let shared_string = SharedString(sign.name);
                let callable = Arc::new(Callable {
                    it_instance: it_instance.clone(),
                    it_module_func,
                });

                Ok((shared_string, callable))
            })
            .collect::<MResult<ExportFunctions<WB>>>()?;

        Ok((export_funcs, module_interface.export_record_types))
    }
}
