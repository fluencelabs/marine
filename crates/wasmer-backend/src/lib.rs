use marine_wasm_backend_traits::WasmBackend;
use marine_wasm_backend_traits::WasmBackendResult;
use marine_wasm_backend_traits::WasmBackendError;
use marine_wasm_backend_traits::Module;
use marine_wasm_backend_traits::Instance;
use marine_wasm_backend_traits::ImportObject;
use marine_wasm_backend_traits::Export;
use marine_wasm_backend_traits::WasiImplementation;

use std::path::PathBuf;

#[derive(Clone)]
pub struct WasmerBackend {}

impl WasmBackend for WasmerBackend {
    type E = WasmerExport;
    type M = WasmerModule;
    type I = WasmerInstance;
    type IO = WasmerImportObject;
    type Wasi = WasmerWasiImplementation;

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
    ) -> Box<dyn Iterator<Item = (String, wasmer_runtime::Export)> + 'a> {
        let exports = self.instance.exports();
        Box::new(exports)
    }

    fn exports(&self) -> &wasmer_core::instance::Exports {
        &self.instance.exports
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

impl Extend<(String, String, WasmerExport)> for WasmerImportObject {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (String, String, WasmerExport)>,
    {
        self.import_object.extend(
            iter.into_iter()
                .map(|(s1, s2, export)| (s1, s2, export.export)),
        )
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

    fn register<S, N>(
        &mut self,
        name: S,
        namespace: N,
    ) -> Option<Box<dyn wasmer_runtime::LikeNamespace>>
    where
        S: Into<String>,
        N: wasmer_runtime::LikeNamespace + Send + 'static,
    {
        self.import_object.register(name, namespace)
    }

    fn maybe_with_namespace<Func, InnerRet>(&self, namespace: &str, f: Func) -> Option<InnerRet>
    where
        Func: FnOnce(&(dyn wasmer_runtime::LikeNamespace + Send)) -> Option<InnerRet>,
        InnerRet: Sized,
    {
        self.import_object.maybe_with_namespace(namespace, f)
    }
}

pub struct WasmerExport {
    export: wasmer_runtime::Export,
}

impl Export for WasmerExport {}

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

/*
pub struct WasmerExportIter {
    export_iter: Box<dyn Iterator<Item = (String, wasmer_runtime::Export)> + 'a>
}

impl<'a> Iterator for WasmerExportIter<'a> {
    type Item = (String, wasmer_runtime::Export);

    fn next(&mut self) -> Option<Self::Item> {
        self.export_iter.as_mut().next()
    }
}*/
