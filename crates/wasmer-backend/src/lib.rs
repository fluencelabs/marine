use std::marker::PhantomData;
//use std::marker::PhantomData;
use marine_wasm_backend_traits::{DynamicFunc, Export, ExportContext, ExportedDynFunc, LikeNamespace, Memory, Namespace, WValue, WasmBackend, CompilationError, InsertFn, WasiVersion, Store};
use marine_wasm_backend_traits::WasmBackendResult;
use marine_wasm_backend_traits::WasmBackendError;
use marine_wasm_backend_traits::Module;
use marine_wasm_backend_traits::Instance;
use marine_wasm_backend_traits::ImportObject;
use marine_wasm_backend_traits::FunctionExport;
use marine_wasm_backend_traits::MemoryExport;
use marine_wasm_backend_traits::WasiImplementation;
use marine_wasm_backend_traits::FuncSig;
use marine_wasm_backend_traits::FuncGetter;
use marine_wasm_backend_traits::WasiState;
use marine_wasm_backend_traits::errors::*;

use std::path::PathBuf;
use std::ptr::NonNull;
use std::slice::Windows;
use std::sync::Arc;
use wasmer_core::backend::SigRegistry;
//use wasmer_core::error::{CallResult, ResolveError, RuntimeError};
use wasmer_core::fault::raw::longjmp;
use wasmer_core::{DynFunc, Func};
use wasmer_core::module::ExportIndex;
use wasmer_core::prelude::Type;
use wasmer_core::prelude::vm::Ctx;
use wasmer_core::typed_func::{ExplicitVmCtx, Host, HostFunction, Wasm, WasmTypeList};
use wasmer_core::types::{LocalOrImport, WasmExternType};
use wasmer_core::types::FuncSig as WasmerFuncSig;
use wasmer_core::typed_func::WasmTypeList as WasmerWasmTypeList;
use wasmer_it::IValue;

mod memory;
mod type_converters;

//use wasmer_it::interpreter::wasm::structures::{SequentialMemoryView, SequentialReader, SequentialWriter};
use crate::memory::WITMemoryView;
use crate::memory::WITMemory;
//use crate::memory_access::{WasmerSequentialReader, WasmerSequentialWriter};
use crate::type_converters::{general_wval_to_wval, wval_to_general_wval};

#[derive(Clone, Default)]
pub struct WasmerBackend /*<'a>*/ {
}

impl WasmBackend for WasmerBackend /*<'b>*/ {
    type Store = WasmerStore;
    type MemoryExport = WasmerMemoryExport;
    type FunctionExport = WasmerFunctionExport;
    type Module = WasmerModule;
    type Instance = WasmerInstance;
    type ImportObject = WasmerImportObject;
    //type SR = WasmerSequentialReader<'b>;
    //type SW = WasmerSequentialWriter<'b>;
    type WITMemory = WITMemory;
    type WITMemoryView = WITMemoryView<'static>;
    type Wasi = WasmerWasiImplementation;
    //type WasiState = WasmerWasiState;
    type DynamicFunc = WasmerDynamicFunc;
    type Namespace = WasmerNamespace;
    type ExportedDynFunc = WasmerExportedDynFunc<'static>;

    fn compile(store: &mut WasmerStore, wasm: &[u8]) -> WasmBackendResult<WasmerModule> {
        wasmer_runtime::compile(wasm)
            .map_err(|e| {
                WasmBackendError::CompilationError(CompilationError::Message(e.to_string()))
            })
            .map(|module| WasmerModule { module })
    }
}

pub struct WasmerStore {

}

impl Store<WasmerBackend> for WasmerStore {
    fn new(_backend: &WasmerBackend) -> Self {
        Self {}
    }
}

pub struct WasmerModule {
    module: wasmer_core::Module,
}

impl Module<WasmerBackend> for WasmerModule {
    fn custom_sections(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.module.custom_sections(name)
    }

    fn instantiate(&self, _store: &mut WasmerStore, imports: &WasmerImportObject) -> WasmBackendResult<WasmerInstance> {
        self.module
            .instantiate(&imports.import_object)
            .map_err(|e| WasmBackendError::InstantiationError(e.to_string()))
            .map(|instance| WasmerInstance {
                instance,
                import_object: imports.clone(),
            })
    }
}

pub struct WasmerInstance {
    pub instance: wasmer_core::Instance,
    pub import_object: WasmerImportObject,
}

impl Instance<WasmerBackend> for WasmerInstance {
    fn export_iter<'a>(
        &'a self,
        _store: &mut WasmerStore,
    ) -> Box<dyn Iterator<Item = (String, Export<WasmerMemoryExport, WasmerFunctionExport>)> + 'a>
    {
        let export_iter = self.instance.exports();
        Box::new(export_iter.map(|(name, export)| (name, export_from_wasmer_export(export))))
    }

    fn memory(&self, _store: &mut WasmerStore, memory_index: u32, ) -> <WasmerBackend as WasmBackend>::WITMemory {
        WITMemory(self.instance.context().memory(memory_index).clone())
    }

    // todo check if right
    fn memory_by_name(&self, _store: &mut WasmerStore, memory_name: &str) -> Option<<WasmerBackend as WasmBackend>::WITMemory> {
        self
            .import_object
            .import_object
            .maybe_with_namespace("env", |env| env.get_export(memory_name))
            .map(|export| match export {
                wasmer_runtime::Export::Memory(memory) => {
                    Some(WITMemory::new( WasmerMemoryExport {memory} ))
                }
                _ => None
            })
            .flatten()
    }

    fn get_func_no_args_no_rets<'a>(
        &'a self,
        _store: &mut WasmerStore,
        name: &str,
    ) -> ResolveResult<Box<dyn Fn(&mut WasmerStore) -> RuntimeResult<()> + 'a>> {
        self.instance
            .exports
            .get::<Func<'a>>(name)
            .map(|func| {
                let func: Box<dyn Fn(&mut WasmerStore) -> RuntimeResult<()> + 'a> =
                    Box::new(move |_store| -> RuntimeResult<()> {
                        func.call()
                            .map_err(|e| RuntimeError::Message(e.to_string()))
                    });
                func
            })
            .map_err(|e| ResolveError::Message(e.to_string()))
    }

    fn get_dyn_func<'a>(
        &'a self,
        _store: &mut WasmerStore,
        name: &str,
    ) -> ResolveResult<<WasmerBackend as WasmBackend>::ExportedDynFunc> {
        self.instance
            .exports
            .get::<DynFunc<'_>>(name)
            .map(|func| unsafe {
                WasmerExportedDynFunc {
                    sig: FuncSigConverter(func.signature()).into(),
                    func: std::mem::transmute::<DynFunc<'_>, DynFunc<'static>>(func),
                }
            })
            .map_err(|e| ResolveError::Message(e.to_string()))
    }
}

#[derive(Clone)]
pub struct WasmerImportObject {
    pub import_object: wasmer_runtime::ImportObject,
}

impl ImportObject<WasmerBackend> for WasmerImportObject {
    fn new(_store: &mut WasmerStore) -> Self {
        WasmerImportObject {
            import_object: wasmer_runtime::ImportObject::new(),
        }
    }

    fn register<S>(
        &mut self,
        name: S,
        namespace: WasmerNamespace,
    ) -> Option<Box<dyn LikeNamespace<WasmerBackend>>>
    where
        S: Into<String>,
    {
        self.import_object
            .register(name, namespace.namespace)
            .map(|namespace| {
                let boxed: Box<
                    (dyn marine_wasm_backend_traits::LikeNamespace<WasmerBackend> + 'static),
                > = Box::new(WasmerLikeNamespace { namespace });
                boxed
            })
    }
}

pub struct WasmerFunctionExport {
    func: wasmer_core::export::FuncPointer,
    /// A kind of context.
    ctx: wasmer_core::export::Context,
    /// The signature of the function.
    signature: Arc<wasmer_runtime::types::FuncSig>,
}

impl FunctionExport for WasmerFunctionExport {}

pub struct WasmerMemoryExport {
    memory: wasmer_runtime::Memory,
}

impl MemoryExport for WasmerMemoryExport {}

pub struct WasmerWasiImplementation {}

impl WasiImplementation<WasmerBackend> for WasmerWasiImplementation {
    fn generate_import_object_for_version(
        version: WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<Vec<u8>>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<WasmerImportObject, String> {
        let version = match version {
            WasiVersion::Snapshot0 => wasmer_wasi::WasiVersion::Snapshot0,
            WasiVersion::Snapshot1 => wasmer_wasi::WasiVersion::Snapshot1,
            WasiVersion::Latest => wasmer_wasi::WasiVersion::Latest,
        };

        wasmer_wasi::generate_import_object_for_version(
            version,
            args,
            envs,
            preopened_files,
            mapped_dirs,
        )
        .map(|import_object| WasmerImportObject { import_object })
    }

    fn get_wasi_state<'s>(instance: &'s mut WasmerInstance) -> Box<dyn WasiState + 's> {
        let wasi_state =
            unsafe { wasmer_wasi::state::get_wasi_state(instance.instance.context_mut()) };
        Box::new(WasmerWasiState { wasi_state })
    }
}

pub struct WasmerWasiState<'a> {
    wasi_state: &'a wasmer_wasi::state::WasiState,
}

impl<'a> WasiState for WasmerWasiState<'a> {
    fn envs(&self) -> &[Vec<u8>] {
        &self.wasi_state.envs
    }
}

fn export_from_wasmer_export(
    export: wasmer_core::export::Export,
) -> Export<WasmerMemoryExport, WasmerFunctionExport> {
    match export {
        wasmer_core::export::Export::Function {
            func,
            ctx,
            signature,
        } => Export::Function(WasmerFunctionExport {
            func,
            ctx,
            signature,
        }),
        wasmer_core::export::Export::Memory(memory) => {
            Export::Memory(WasmerMemoryExport { memory })
        }
        wasmer_core::export::Export::Table(_table) => Export::Other,
        wasmer_core::export::Export::Global(_global) => Export::Other,
    }
}

impl Into<wasmer_runtime::Export> for WasmerMemoryExport {
    fn into(self) -> wasmer_core::export::Export {
        wasmer_runtime::Export::Memory(self.memory)
    }
}

impl Into<wasmer_runtime::Export> for WasmerFunctionExport {
    fn into(self) -> wasmer_core::export::Export {
        wasmer_runtime::Export::Function {
            func: self.func,
            ctx: self.ctx,
            signature: self.signature,
        }
    }
}

impl Memory<WasmerBackend> for WITMemory {
    fn new(export: WasmerMemoryExport) -> Self {
        WITMemory(export.memory)
    }
    /*
    fn view_from_ctx(ctx: &Ctx, memory_index: u32) -> WITMemoryView<'static> {
        let memory = unsafe {
            std::mem::transmute::<&'_ wasmer_runtime::Memory, &'static wasmer_runtime::Memory>(
                ctx.memory(memory_index),
            )
        };

        WITMemoryView(memory.view::<u8>())
    }*/
    fn size(&self) -> usize {
        self.0.size().bytes().0
    }
}

pub struct WasmerDynamicFunc {
    func: wasmer_core::typed_func::DynamicFunc<'static>,
}

impl<'a> DynamicFunc<'a, WasmerBackend> for WasmerDynamicFunc {
    fn new<F>(_store: &mut WasmerStore, sig: FuncSig, func: F) -> Self
    where
        F: Fn(&mut dyn ExportContext<WasmerBackend>, &[WValue]) -> Vec<WValue> + 'static,
    {
        let func = wasmer_core::typed_func::DynamicFunc::new(
            std::sync::Arc::new(FuncSigConverter(&sig).into()),
            move |ctx: &mut wasmer_core::vm::Ctx, args: &[wasmer_core::prelude::Value]| unsafe {
                let mut ctx = WasmerExportContext {
                    ctx
                    /*ctx: std::mem::transmute::<
                        &'_ mut wasmer_core::vm::Ctx,
                        &'static mut wasmer_core::vm::Ctx,
                    >(ctx),*/
                };

                let args = args.iter().map(wval_to_general_wval).collect::<Vec<_>>();
                func(&mut ctx, &args)
                    .iter()
                    .map(general_wval_to_wval)
                    .collect()
            },
        );

        Self { func }
    }
}

pub struct WasmerNamespace {
    namespace: wasmer_core::import::Namespace,
}

/*
impl WasmerNamespace {
   unsafe fn gen_closure<F, Args: WasmTypeList, Rets: WasmTypeList>(&mut self, func: F)  -> impl Fn(&mut WasmerExportContext, Args) -> Rets
        where F: Fn(&mut <WasmerBackend as WasmBackend>::ExportContext, Args) -> Rets {
        move |ctx: &mut wasmer_core::vm::Ctx, args: Args| -> Rets {
            let mut ctx = WasmerExportContext {
                ctx: std::mem::transmute::<
                    &'_ mut wasmer_core::vm::Ctx,
                    &'static mut wasmer_core::vm::Ctx,
                >(ctx),
            };

            func(&mut ctx, args)
        }
    }

    fn insert_fn_impl<F, Args: WasmTypeList, Rets: WasmTypeList>(&mut self, name: impl Into<String>, func: F) where F: Fn(&mut <WasmerBackend as WasmBackend>::ExportContext, Args) -> Rets {
        let func/*: Func<'_, Args, Rets, Host>*/ = wasmer_runtime::func!(self.gen_closure(func));

        self.namespace.insert(name, func);
    }
}*/

impl LikeNamespace<WasmerBackend> for WasmerNamespace {}

macro_rules! impl_insert_fn {
    ($($name:ident: $arg:ty),* => $rets:ty) => {
        impl InsertFn<WasmerBackend, ($($arg,)*), $rets> for WasmerNamespace {
            fn insert_fn<F>(&mut self, name: impl Into<String>, func: F)
            where F:'static + Fn(&mut dyn ExportContext<WasmerBackend>, ($($arg,)*)) -> $rets + std::marker::Send {
                let func = move |ctx: &mut wasmer_core::vm::Ctx, $($name: $arg),*| {
                    unsafe {
                        let mut ctx = WasmerExportContext {
                            ctx: std::mem::transmute::<
                                &'_ mut wasmer_core::vm::Ctx,
                                &'static mut wasmer_core::vm::Ctx,
                            >(ctx),
                        };

                        func(&mut ctx, ($($name,)*))
                    }
                };

                self.namespace.insert(name, wasmer_runtime::func!(func));
            }
        }
    };
}

impl_insert_fn!(=> ());
impl_insert_fn!(A: i32 => ());
impl_insert_fn!(A: i32, B: i32 => ());
impl_insert_fn!(A: i32, B: i32, C: i32 => ());
impl_insert_fn!(A: i32, B: i32, C: i32, D: i32 => ());

impl Namespace<WasmerBackend> for WasmerNamespace {
    fn new() -> Self {
        Self {
            namespace: wasmer_core::import::Namespace::new(),
        }
    }

    fn insert(&mut self, name: impl Into<String>, func: WasmerDynamicFunc) {
        self.namespace.insert(name, func.func);
    }
}

struct WasmerLikeNamespace {
    namespace: Box<dyn wasmer_core::import::LikeNamespace + 'static>,
}

impl LikeNamespace<WasmerBackend> for WasmerLikeNamespace {}

pub struct WasmerExportContext<'c> {
    ctx: &'c mut wasmer_core::vm::Ctx,
}

impl<'c> WasmerExportContext<'c> {
    unsafe fn get_func_impl<'s, Args, Rets>(
        &'s mut self,
        name: &str,
    ) -> Result<Box<dyn FnMut(Args) -> Result<Rets, RuntimeError> + 's>, ResolveError>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let ctx = &mut self.ctx;
        let module_inner = &(*ctx.module);

        let export_index = module_inner.info.exports.get(name).ok_or_else(|| {
            ResolveError::Message(
                wasmer_core::error::ResolveError::ExportNotFound {
                    name: name.to_string(),
                }
                .to_string(),
            )
        })?;

        let export_func_index = match export_index {
            ExportIndex::Func(func_index) => func_index,
            _ => {
                return Err(ResolveError::Message(
                    wasmer_core::error::ResolveError::ExportWrongType {
                        name: name.to_string(),
                    }
                    .to_string(),
                ))
            }
        };

        let export_func_signature_idx = *module_inner
            .info
            .func_assoc
            .get(*export_func_index)
            .expect("broken invariant, incorrect func index");

        let export_func_signature = &module_inner.info.signatures[export_func_signature_idx];
        let export_func_signature_ref = SigRegistry.lookup_signature_ref(export_func_signature);

        let arg_types = Args::types();
        let ret_types = Rets::types();
        if export_func_signature_ref.params() != arg_types
            || export_func_signature_ref.returns() != ret_types
        {
            return Err(ResolveError::Message(
                wasmer_core::error::ResolveError::Signature {
                    expected: (*export_func_signature).clone(),
                    found: ret_types.to_vec(),
                }
                .to_string(),
            ));
        }

        let func_wasm_inner = module_inner
            .runnable_module
            .get_trampoline(&module_inner.info, export_func_signature_idx)
            .unwrap();

        let export_func_ptr = match export_func_index.local_or_import(&module_inner.info) {
            LocalOrImport::Local(local_func_index) => module_inner
                .runnable_module
                .get_func(&module_inner.info, local_func_index)
                .unwrap(),
            _ => {
                return Err(ResolveError::Message(
                    wasmer_core::error::ResolveError::ExportNotFound {
                        name: name.to_string(),
                    }
                    .to_string(),
                ))
            }
        };

        let typed_func: Func<'_, Args, Rets, wasmer_core::typed_func::Wasm> =
            Func::from_raw_parts(func_wasm_inner, export_func_ptr, None, (*ctx) as _);

        let result = Box::new(move |args: Args| -> Result<Rets, RuntimeError> {
            unsafe {
                args.call::<Rets>(export_func_ptr, func_wasm_inner, *ctx)
                    .map_err(|e| RuntimeError::Message(e.to_string()))
            }
        });

        Ok(result)
    }
}

macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl<'r> FuncGetter<$args, $rets> for WasmerExportContext<'r> {
            unsafe fn get_func<'c>(
                &'c mut self,
                name: &str,
            ) -> Result<Box<dyn FnMut($args) -> Result<$rets, RuntimeError> + 'c>, ResolveError>
            {
                self.get_func_impl(name)
            }
        }
    };
}

impl_func_getter!((i32, i32), i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!(i32, i32);
impl_func_getter!(i32, ());
impl_func_getter!((), i32);
impl_func_getter!((), ());

impl<'c, 'r> ExportContext<'c, WasmerBackend> for WasmerExportContext<'r> {
    fn memory(&mut self, memory_index: u32) -> <WasmerBackend as WasmBackend>::WITMemory {
        WITMemory(self.ctx.memory(memory_index).clone())
    }
}

pub struct WasmerExportedDynFunc<'a> {
    func: DynFunc<'a>,
    sig: FuncSig,
}

impl<'a> ExportedDynFunc<WasmerBackend> for WasmerExportedDynFunc<'a> {
    fn signature(&self, _store: &WasmerStore) -> &FuncSig {
        &self.sig
    }

    fn call(&self, _store: &mut WasmerStore, args: &[WValue]) -> CallResult<Vec<WValue>> {
        use crate::type_converters::general_wval_to_wval;
        use crate::type_converters::wval_to_general_wval;
        self.func
            .call(
                &args
                    .iter()
                    .map(general_wval_to_wval)
                    .collect::<Vec<wasmer_runtime::Value>>(),
            )
            .map(|rets| rets.iter().map(wval_to_general_wval).collect())
            .map_err(|e| CallError::Message(e.to_string()))
    }
}

struct FuncSigConverter<'a, T>(&'a T);

impl<'a> From<FuncSigConverter<'a, FuncSig>> for WasmerFuncSig {
    fn from(sig: FuncSigConverter<'a, FuncSig>) -> Self {
        let params = sig
            .0
            .params()
            .map(type_converters::general_wtype_to_wtype)
            .collect::<Vec<_>>();
        let returns = sig
            .0
            .returns()
            .map(type_converters::general_wtype_to_wtype)
            .collect::<Vec<_>>();
        Self::new(params, returns)
    }
}

impl<'a> From<FuncSigConverter<'a, WasmerFuncSig>> for FuncSig {
    fn from(sig: FuncSigConverter<'a, WasmerFuncSig>) -> Self {
        let params = sig
            .0
            .params()
            .iter()
            .map(type_converters::wtype_to_general_wtype)
            .collect::<Vec<_>>();
        let returns = sig
            .0
            .returns()
            .iter()
            .map(type_converters::wtype_to_general_wtype)
            .collect::<Vec<_>>();
        Self::new(params, returns)
    }
}

macro_rules! gen_get_func_impl {
    ($args:ty, $rets:ty) => {
        unsafe fn get_export_func_by_name<'c>(
            &'c mut self,
            name: &str,
        ) -> Result<
            Box<dyn FnMut($args) -> Result<$rets, wasmer_runtime::error::RuntimeError> + 'c>,
            wasmer_runtime::error::ResolveError,
        > {
            let ctx = &mut self.ctx;
            let module_inner = &(*ctx.module);

            let export_index = module_inner.info.exports.get(name).ok_or_else(|| {
                ResolveError::ExportNotFound {
                    name: name.to_string(),
                }
            })?;

            let export_func_index = match export_index {
                ExportIndex::Func(func_index) => func_index,
                _ => {
                    return Err(ResolveError::ExportWrongType {
                        name: name.to_string(),
                    })
                }
            };

            let export_func_signature_idx = *module_inner
                .info
                .func_assoc
                .get(*export_func_index)
                .expect("broken invariant, incorrect func index");

            let export_func_signature = &module_inner.info.signatures[export_func_signature_idx];
            let export_func_signature_ref = SigRegistry.lookup_signature_ref(export_func_signature);

            if export_func_signature_ref.params() != $args::types()
                || export_func_signature_ref.returns() != $rets::types()
            {
                return Err(ResolveError::Signature {
                    expected: (*export_func_signature).clone(),
                    found: $args::types().to_vec(),
                });
            }

            let func_wasm_inner = module_inner
                .runnable_module
                .get_trampoline(&module_inner.info, export_func_signature_idx)
                .unwrap();

            let export_func_ptr = match export_func_index.local_or_import(&module_inner.info) {
                LocalOrImport::Local(local_func_index) => module_inner
                    .runnable_module
                    .get_func(&module_inner.info, local_func_index)
                    .unwrap(),
                _ => {
                    return Err(ResolveError::ExportNotFound {
                        name: name.to_string(),
                    })
                }
            };

            /*
            let typed_func: Func<'_, Args, Rets, wasmer_core::typed_func::Wasm> =
                Func::from_raw_parts(func_wasm_inner, export_func_ptr, None, (*ctx) as _);
            */
            let result = Box::new(move |args: $args| -> Result<$rets, RuntimeError> {
                $args
                    .into_c_struct()
                    .call::<$rets::CStruct>(export_func_ptr, func_wasm_inner, *ctx)
            });

            Ok(result)
        }
    };
}
