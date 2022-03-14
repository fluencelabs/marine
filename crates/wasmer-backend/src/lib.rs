use std::marker::PhantomData;
//use std::marker::PhantomData;
use marine_wasm_backend_traits::{DynamicFunc, Export, ExportContext, ExportedDynFunc, LikeNamespace, Memory, Namespace, WValue, WasmBackend};
use marine_wasm_backend_traits::WasmBackendResult;
use marine_wasm_backend_traits::WasmBackendError;
use marine_wasm_backend_traits::Module;
use marine_wasm_backend_traits::Instance;
use marine_wasm_backend_traits::ImportObject;
use marine_wasm_backend_traits::FunctionExport;
use marine_wasm_backend_traits::MemoryExport;
use marine_wasm_backend_traits::Exports;
use marine_wasm_backend_traits::WasiImplementation;
use marine_wasm_backend_traits::FuncSig;
use marine_wasm_backend_traits::Tuple;


use std::path::PathBuf;
use std::ptr::NonNull;
use std::slice::Windows;
use std::sync::Arc;
use wasmer_core::backend::SigRegistry;
use wasmer_core::error::{CallResult, ResolveError, ResolveResult, RuntimeError};
use wasmer_core::fault::raw::longjmp;
use wasmer_core::{DynFunc, Func};
use wasmer_core::module::ExportIndex;
use wasmer_core::prelude::Type;
use wasmer_core::prelude::vm::Ctx;
use wasmer_core::typed_func::Wasm;
use wasmer_core::types::{LocalOrImport, WasmExternType};
use wasmer_core::types::FuncSig as WasmerFuncSig;
use wasmer_core::typed_func::WasmTypeList as WasmerWasmTypeList;
use wasmer_wasi::state::WasiState;
use wasmer_it::IValue;

mod memory_access;
mod memory;
mod type_converters;
mod wasm_type_list;

//use wasmer_it::interpreter::wasm::structures::{SequentialMemoryView, SequentialReader, SequentialWriter};
use crate::memory::WITMemoryView;
use crate::memory::WITMemory;
use crate::memory_access::{WasmerSequentialReader, WasmerSequentialWriter};
use crate::type_converters::{general_wval_to_wval, wval_to_general_wval};



#[derive(Clone)]
pub struct WasmerBackend /*<'a>*/ {
    //    _data: &'a PhantomData<i32>,
}

impl WasmBackend for WasmerBackend /*<'b>*/ {
    type Exports = WasmerInstance;
    type MemoryExport = WasmerMemoryExport;
    type FunctionExport = WasmerFunctionExport;
    type M = WasmerModule;
    type I = WasmerInstance;
    type IO = WasmerImportObject;
    //type SR = WasmerSequentialReader<'b>;
    //type SW = WasmerSequentialWriter<'b>;
    type WITMemory = WITMemory;
    type WITMemoryView = WITMemoryView<'static>;
    type Wasi = WasmerWasiImplementation;
    type DynamicFunc = WasmerDynamicFunc;
    type Namespace = WasmerNamespace;
    type ExportContext = WasmerExportContext<'static>;
    type ExportedDynFunc = WasmerExportedDynFunc<'static>;

    fn compile(wasm: &[u8]) -> WasmBackendResult<WasmerModule> {
        wasmer_runtime::compile(wasm)
            .map_err(|_| WasmBackendError::SomeError)
            .map(|module| WasmerModule { module })
    }
}

pub struct WasmerModule {
    module: wasmer_core::Module,
}

impl Module<WasmerBackend> for WasmerModule {
    fn custom_sections(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.module.custom_sections(name)
    }

    fn instantiate(&self, imports: &WasmerImportObject) -> WasmBackendResult<WasmerInstance> {
        self.module
            .instantiate(&imports.import_object)
            .map_err(|_| WasmBackendError::SomeError)
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
    ) -> Box<dyn Iterator<Item = (String, Export<WasmerMemoryExport, WasmerFunctionExport>)> + 'a>
    {
        let export_iter = self.instance.exports();
        Box::new(export_iter.map(|(name, export)| (name, export_from_wasmer_export(export))))
    }

    fn exports(&self) -> &Self {
        self
    }

    fn import_object(&self) -> &WasmerImportObject {
        &self.import_object
    }

    fn memory(&self, memory_index: u32) -> <WasmerBackend as WasmBackend>::WITMemory {
        WITMemory(self.instance.context().memory(memory_index).clone())
    }
}

#[derive(Clone)]
pub struct WasmerImportObject {
    pub import_object: wasmer_runtime::ImportObject,
}

impl
    Extend<(
        String,
        String,
        Export<WasmerMemoryExport, WasmerFunctionExport>,
    )> for WasmerImportObject
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<
            Item = (
                String,
                String,
                Export<WasmerMemoryExport, WasmerFunctionExport>,
            ),
        >,
    {
        self.import_object
            .extend(iter.into_iter().map(|(s1, s2, export)| match export {
                Export::Memory(memory) => (s1, s2, memory.into()),
                Export::Function(func) => (s1, s2, func.into()),
                _ => unreachable!(),
            }))
    }
}

impl ImportObject<WasmerBackend> for WasmerImportObject {
    fn new() -> Self {
        WasmerImportObject {
            import_object: wasmer_runtime::ImportObject::new(),
        }
    }

    fn extend_with_self(&mut self, other: Self) {
        self.import_object.extend(other.import_object);
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

    fn get_memory_env(&self) -> Option<Export<WasmerMemoryExport, WasmerFunctionExport>> {
        self.import_object
            .maybe_with_namespace("env", |env| env.get_export("memory"))
            .map(|export| match export {
                wasmer_runtime::Export::Memory(memory) => {
                    Export::Memory(WasmerMemoryExport { memory })
                }
                _ => Export::Other,
            })
    }

    /*
    fn maybe_with_namespace<Func, InnerRet>(&self, namespace: &str, f: Func) -> Option<InnerRet>
    where
        Func: FnOnce(&(dyn wasmer_runtime::LikeNamespace + Send)) -> Option<InnerRet>,
        InnerRet: Sized,
    {
        self.import_object.maybe_with_namespace(namespace, f)
    }*/
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
        version: wasmer_wasi::WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<Vec<u8>>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<WasmerImportObject, String> {
        wasmer_wasi::generate_import_object_for_version(
            version,
            args,
            envs,
            preopened_files,
            mapped_dirs,
        )
        .map(|import_object| WasmerImportObject { import_object })
    }

    fn get_wasi_state(instance: &mut <WasmerBackend as WasmBackend>::I) -> &WasiState {
        unsafe { wasmer_wasi::state::get_wasi_state(instance.instance.context_mut()) }
    }
}

impl Exports<WasmerBackend> for WasmerInstance {
    fn get_func_no_args_no_rets<'a>(
        &'a self,
        name: &str,
    ) -> wasmer_core::error::ResolveResult<
        Box<dyn Fn() -> wasmer_core::error::RuntimeResult<()> + 'a>,
    > {
        self.instance.exports.get::<Func<'a>>(name).map(|func| {
            let func: Box<dyn Fn() -> wasmer_core::error::RuntimeResult<()> + 'a> =
                Box::new(move || -> wasmer_core::error::RuntimeResult<()> { func.call() });
            func
        })
    }

    fn get_dyn_func<'a>(
        &'a self,
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
    fn new<F>(sig: FuncSig, func: F) -> Self
    where
        F: Fn(&mut WasmerExportContext<'static>, &[WValue]) -> Vec<WValue> + 'static,
    {
        let func = wasmer_core::typed_func::DynamicFunc::new(
            std::sync::Arc::new(FuncSigConverter(&sig).into()),
            move |ctx: &mut wasmer_core::vm::Ctx, args: &[wasmer_core::prelude::Value]| unsafe {
                let mut ctx = WasmerExportContext {
                    ctx: std::mem::transmute::<
                        &'_ mut wasmer_core::vm::Ctx,
                        &'static mut wasmer_core::vm::Ctx,
                    >(ctx),
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

impl LikeNamespace<WasmerBackend> for WasmerNamespace {}

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

impl<'c> ExportContext<'c, WasmerBackend> for WasmerExportContext<'static> {
    fn memory(&self, memory_index: u32) -> <WasmerBackend as WasmBackend>::WITMemory {
        WITMemory(self.ctx.memory(memory_index).clone())
    }

    unsafe fn get_export_func_by_name<Args, Rets>(
        &'c mut self,
        name: &str,
    ) -> Result<Box<dyn FnMut(Args) -> Result<Rets, RuntimeError> + 'c>, ResolveError>
    where
        Args: Tuple,
        Rets: Tuple,
    {
        let ctx = &mut self.ctx;
        let module_inner = &(*ctx.module);

        let export_index =
            module_inner
                .info
                .exports
                .get(name)
                .ok_or_else(|| ResolveError::ExportNotFound {
                    name: name.to_string(),
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

        let arg_types = <Args::CStruct as wasmer_core::typed_func::WasmTypeList>::types();
        let ret_types = <Rets::CStruct as wasmer_core::typed_func::WasmTypeList>::types();
        if export_func_signature_ref.params() != arg_types//Args::types()
            || export_func_signature_ref.returns() != ret_types//Rets::types()
        {
            return Err(ResolveError::Signature {
                expected: (*export_func_signature).clone(),
                found: /*Helper::<Args>::types()*/ret_types.to_vec(),
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


        let typed_func: Func<'_, Args, Rets, wasmer_core::typed_func::Wasm> =
            Func::from_raw_parts(func_wasm_inner, export_func_ptr, None, (*ctx) as _);

        let result = Box::new(
            move |args: Args| -> Result<Rets, RuntimeError> {
                args.into_c_struct()
                    .call::<Rets::CStruct>(export_func_ptr, func_wasm_inner, *ctx)
                    .map(|rets: Rets| rets.0)
            }
        );

        Ok(result)
    }
}

pub struct WasmerExportedDynFunc<'a> {
    func: DynFunc<'a>,
    sig: FuncSig,
}

impl<'a> ExportedDynFunc<WasmerBackend> for WasmerExportedDynFunc<'a> {
    fn signature(&self) -> &FuncSig {
        &self.sig
    }

    fn call(&self, args: &[WValue]) -> CallResult<Vec<WValue>> {
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
