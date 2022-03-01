use std::marker::PhantomData;
use marine_wasm_backend_traits::{
    DynamicFunc, Export, LikeNamespace, Memory, Namespace, Value, WasmBackend,
};
use marine_wasm_backend_traits::WasmBackendResult;
use marine_wasm_backend_traits::WasmBackendError;
use marine_wasm_backend_traits::Module;
use marine_wasm_backend_traits::Instance;
use marine_wasm_backend_traits::ImportObject;
use marine_wasm_backend_traits::FunctionExport;
use marine_wasm_backend_traits::MemoryExport;
use marine_wasm_backend_traits::Exports;
use marine_wasm_backend_traits::WasiImplementation;

use std::path::PathBuf;
use std::slice::Windows;
use std::sync::Arc;
use wasmer_core::fault::raw::longjmp;
use wasmer_core::prelude::vm::Ctx;
use wasmer_core::types::FuncSig;

mod memory_access;
mod memory;

//use wasmer_it::interpreter::wasm::structures::{SequentialMemoryView, SequentialReader, SequentialWriter};
use crate::memory::WITMemoryView;
use crate::memory::WITMemory;
use crate::memory_access::{WasmerSequentialReader, WasmerSequentialWriter};

#[derive(Clone)]
pub struct WasmerBackend /*<'a>*/ {
    //    _data: &'a PhantomData<i32>,
}

impl<'b> WasmBackend for WasmerBackend /*<'b>*/ {
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

    fn context(&self) -> &wasmer_core::vm::Ctx {
        self.instance.context()
    }
    fn context_mut(&mut self) -> &mut wasmer_core::vm::Ctx {
        self.instance.context_mut()
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
}

impl Exports<WasmerBackend> for WasmerInstance {
    fn get<'a, T: wasmer_core::export::Exportable<'a>>(
        &'a self,
        name: &str,
    ) -> wasmer_core::error::ResolveResult<T> {
        self.instance.exports.get(name)
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

    fn view_from_ctx(ctx: &Ctx, memory_index: u32) -> WITMemoryView<'static> {
        let memory = unsafe {
            std::mem::transmute::<&'_ wasmer_runtime::Memory, &'static wasmer_runtime::Memory>(
                ctx.memory(memory_index),
            )
        };

        WITMemoryView(memory.view::<u8>())
    }
}

pub struct WasmerDynamicFunc {
    func: wasmer_core::typed_func::DynamicFunc<'static>,
}

impl<'a> DynamicFunc<'a> for WasmerDynamicFunc {
    fn new<F>(sig: Arc<FuncSig>, func: F) -> Self
    where
        F: Fn(&mut Ctx, &[wasmer_core::prelude::Value]) -> Vec<wasmer_core::prelude::Value>
            + 'static,
    {
        let func = wasmer_core::typed_func::DynamicFunc::new(sig, func);
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
