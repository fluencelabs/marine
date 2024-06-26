/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::wit_prelude::*;
use super::MFunctionSignature;
use super::MRecordTypes;
use super::IType;
use super::IRecordType;
use super::IFunctionArg;
use super::IValue;
use super::WValue;
use crate::generic::HostImportDescriptor;
use crate::MResult;
use crate::generic::MModuleConfig;
use crate::config::HostAPIVersion;
use crate::config::RawImportCreator;

use marine_wasm_backend_traits::prelude::*;

use marine_it_interfaces::MITInterfaces;
use marine_it_parser::extract_it_from_module;
use marine_utils::SharedString;
use wasmer_it::interpreter::Interpreter;

use futures::future::BoxFuture;
use futures::FutureExt;

use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::borrow::BorrowMut;

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
    pub async fn call_async(
        &mut self,
        store: &mut <WB as WasmBackend>::ContextMut<'_>,
        args: &[IValue],
    ) -> MResult<Vec<IValue>> {
        use wasmer_it::interpreter::stack::Stackable;

        let result = self
            .it_module_func
            .interpreter
            .run(args, Arc::make_mut(&mut self.it_instance), store)
            .await?
            .as_slice()
            .to_owned();
        Ok(result)
    }
}

type ExportFunctions<WB> = HashMap<SharedString, Arc<Callable<WB>>>;

pub(crate) struct MModule<WB: WasmBackend> {
    wasm_instance: Box<<WB as WasmBackend>::Instance>,

    export_funcs: ExportFunctions<WB>,

    // TODO: save refs instead copying of a record types HashMap.
    /// Record types used in exported functions as arguments or return values.
    export_record_types: MRecordTypes,
}

impl<WB: WasmBackend> MModule<WB> {
    pub(crate) async fn new(
        name: &str,
        store: &mut <WB as WasmBackend>::Store,
        wasm_bytes: &[u8],
        config: MModuleConfig<WB>,
        modules: &HashMap<String, MModule<WB>>,
    ) -> MResult<Self> {
        let wasm_module = <WB as WasmBackend>::Module::new(store, wasm_bytes)?;
        crate::misc::check_sdk_version::<WB>(name.to_string(), &wasm_module)?;

        let it = extract_it_from_module::<WB>(&wasm_module)?;
        crate::misc::check_it_version(name, &it.version)?;

        let mit = MITInterfaces::new(it);

        let mut wit_instance = Arc::new_uninit();
        let mut linker = <WB as WasmBackend>::Imports::new(store);

        let MModuleConfig {
            raw_imports,
            host_imports,
            wasi_parameters,
            ..
        } = config;

        Self::add_wit_imports(store, &mut linker, &mit, wit_instance.clone())?;
        Self::add_wasi_imports(store, &mut linker, wasi_parameters)?;
        Self::add_host_imports(store, &mut linker, raw_imports, host_imports, &mit)?;

        let wasm_instance = wasm_module.instantiate(store, &linker).await?;
        let it_instance = unsafe {
            // TODO: check if this MaybeUninit/Arc tricks are still needed
            // get_mut_unchecked here is safe because currently only this modules have reference to
            // it and the environment is single-threaded
            *Arc::get_mut_unchecked(&mut wit_instance) =
                MaybeUninit::new(ITInstance::new(&wasm_instance, store, name, &mit, modules)?);
            std::mem::transmute::<_, Arc<ITInstance<WB>>>(wit_instance)
        };

        let (export_funcs, export_record_types) = Self::instantiate_exports(&it_instance, &mit)?;

        // backend is not expected to call _start or _initialize
        // call _initialize to populate the WASI state of the module
        #[rustfmt::skip]
        if let Ok(initialize_func) = wasm_instance.get_function(store, INITIALIZE_FUNC) {
            initialize_func.call_async(store, &[]).await?;
        }
        // call _start to call module's main function
        #[rustfmt::skip]
        if let Ok(start_func) = wasm_instance.get_function(store, START_FUNC) {
            start_func.call_async(store, &[]).await?;
        }

        Ok(Self {
            wasm_instance: Box::new(wasm_instance),
            export_funcs,
            export_record_types,
        })
    }

    pub(crate) async fn call_async(
        &mut self,
        store: &mut <WB as WasmBackend>::ContextMut<'_>,
        module_name: &str,
        function_name: &str,
        args: &[IValue],
    ) -> MResult<Vec<IValue>> {
        log::debug!(
            "calling {}::{} with args: {:?}",
            module_name,
            function_name,
            args
        );
        let func = self.export_funcs.get_mut(function_name).ok_or_else(|| {
            MError::NoSuchFunction(module_name.to_string(), function_name.to_string())
        })?;
        let res = Arc::make_mut(func).call_async(store, args).await;

        log::debug!(
            "calling {}::{} with result: {:?}",
            module_name,
            function_name,
            res
        );
        res
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
        <WB as WasmBackend>::Wasi::get_wasi_state(self.wasm_instance.borrow_mut())
    }

    /// Returns Wasm linear memory size that this module consumes in bytes.
    pub(crate) fn memory_size(&self, store: &mut <WB as WasmBackend>::ContextMut<'_>) -> usize {
        let memory = self
            .wasm_instance
            .get_nth_memory(store, STANDARD_MEMORY_INDEX)
            .expect("It is expected that the existence of at least one memory is checked in the MModule::new function");

        memory.size(store)
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
        parameters: WasiParameters,
    ) -> MResult<()> {
        <WB as WasmBackend>::Wasi::register_in_linker(
            &mut store.as_context_mut(),
            linker,
            parameters,
        )?;

        Ok(())
    }

    fn add_host_imports(
        store: &mut <WB as WasmBackend>::Store,
        linker: &mut <WB as WasmBackend>::Imports,
        raw_imports: HashMap<HostAPIVersion, HashMap<String, RawImportCreator<WB>>>,
        host_imports: HashMap<HostAPIVersion, HashMap<String, HostImportDescriptor<WB>>>,
        mit: &MITInterfaces<'_>,
    ) -> MResult<()> {
        use crate::host_imports::create_host_import_func;
        for (version, raw_imports) in raw_imports {
            let raw_imports = raw_imports
                .into_iter()
                .map(|(name, creator)| (name, creator(store.as_context_mut())))
                .collect::<Vec<_>>();

            linker.register(store, version.namespace(), raw_imports)?;
        }

        for (version, host_imports) in host_imports {
            let record_types = mit
                .record_types()
                .map(|(id, r)| (id, r.clone()))
                .collect::<HashMap<_, _>>();
            let record_types = Arc::new(record_types);

            let host_imports = host_imports
                .into_iter()
                .map(|(import_name, descriptor)| {
                    let func =
                        create_host_import_func::<WB>(store, descriptor, record_types.clone());
                    (import_name, func)
                })
                .collect::<Vec<_>>();

            linker.register(store, version.namespace(), host_imports)?;
        }

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
        ) -> <WB as WasmBackend>::HostFunction
        where
            F: for<'c> Fn(
                    <WB as WasmBackend>::ImportCallContext<'c>,
                    &'c [WValue],
                ) -> BoxFuture<'c, anyhow::Result<Vec<WValue>>>
                + Sync
                + Send
                + 'static,
            WB: WasmBackend,
            I1: Iterator<Item = &'a IType>,
            I2: Iterator<Item = &'b IType>,
        {
            use super::type_converters::itype_to_wtype;

            let inputs = inputs.map(itype_to_wtype).collect::<Vec<_>>();
            let outputs = outputs.map(itype_to_wtype).collect::<Vec<_>>();
            <WB as WasmBackend>::HostFunction::new_with_caller_async(
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
        ) -> impl for<'c> Fn(
            <WB as WasmBackend>::ImportCallContext<'c>,
            &'c [WValue],
        ) -> BoxFuture<'c, anyhow::Result<Vec<WValue>>>
               + Sync
               + Send
               + 'static {
            let import_namespace = std::sync::Arc::new(import_namespace);
            let import_name = std::sync::Arc::new(import_name);
            let interpreter = std::sync::Arc::new(interpreter);

            move |mut ctx: <WB as WasmBackend>::ImportCallContext<'_>,
                  inputs: &[WValue]|
                  -> BoxFuture<'_, anyhow::Result<Vec<WValue>>> {
                let import_namespace = import_namespace.clone();
                let import_name = import_name.clone();
                let wit_instance = wit_instance.clone();
                let interpreter = interpreter.clone();
                async move {
                    let mut ctx = ctx.as_context_mut();

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
                        interpreter
                            .run(
                                &wit_inputs,
                                Arc::make_mut(&mut wit_instance_callable.assume_init()),
                                &mut ctx.as_context_mut(),
                            )
                            .await
                            .map_err(|e| {
                                log::error!("interpreter got error {e}");
                                anyhow::anyhow!(e)
                            })?
                    };

                    log::trace!(
                        "\nraw import for {}.{} finished",
                        import_namespace,
                        import_name
                    );

                    // TODO: optimize by prevent copying stack values
                    Ok(outputs
                        .as_slice()
                        .iter()
                        .map(ival_to_wval)
                        .collect::<Vec<_>>())
                }
                .boxed()
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
                            adapter_instructions.clone().try_into().map_err(|_| {
                                MError::IncorrectWIT(
                                    "failed to parse instructions for adapter type".to_string(),
                                )
                            })?;

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
            linker.register(store, namespace_name, funcs)?;
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

                let interpreter: ITInterpreter<WB> =
                    adapter_instructions.clone().try_into().map_err(|_| {
                        MError::IncorrectWIT(
                            "failed to parse instructions for adapter type".to_string(),
                        )
                    })?;

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
