use marine_wasm_backend_traits::WasmBackend;
use marine_wasm_backend_traits::WasmBackendResult;
use marine_wasm_backend_traits::WasmBackendError;
use marine_wasm_backend_traits::Module;
use marine_wasm_backend_traits::Instance;

#[derive(Clone)]
pub struct WasmerBackend {}

impl WasmBackend for WasmerBackend {
    type M = WasmerModule;

    fn compile(wasm: &[u8]) -> WasmBackendResult<WasmerModule> {
        wasmer_runtime::compile(wasm).map_err(|_| {
            WasmBackendError::SomeError
        }).map(|module| {
            WasmerModule { module }
        })
    }
}

pub struct WasmerModule {
    module: wasmer_core::Module,
}

impl Module for WasmerModule {
    type I = WasmerInstance;

    fn custom_sections(&self, name: &str) -> Option<&[Vec<u8>]> {
        self.module.custom_sections(name)
    }

    fn instantiate(&self, imports: &wasmer_runtime::ImportObject) -> WasmBackendResult<Self::I> {
        self.module
            .instantiate(&imports)
            .map_err(|_| {
                WasmBackendError::SomeError
            })
            .map(|instance| {WasmerInstance{instance}})
    }
}

pub struct WasmerInstance {
    instance: wasmer_core::Instance,
}

impl Instance for WasmerInstance {
    fn export_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (String, wasmer_runtime::Export)> + 'a> {
        let exports = self.instance.exports();
        Box::new(exports)
    }

    fn exports(&self) -> &wasmer_core::instance::Exports {
        &self.instance.exports
    }

    fn import_object(&self) -> &wasmer_runtime::ImportObject {
        &self.instance.import_object
    }

    fn context(&self) -> &wasmer_core::vm::Ctx { self.instance.context() }
    fn context_mut(&mut self) -> &mut wasmer_core::vm::Ctx { self.instance.context_mut()}
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