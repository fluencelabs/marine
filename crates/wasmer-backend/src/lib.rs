use marine_wasm_backend_traits::*;


mod module;
mod store;
mod instance;
mod function;
mod imports;
mod utils;
mod memory;

use module::*;
use store::*;
use instance::*;
use function::*;
use imports::*;
use memory::*;
pub(crate) use utils::*;

#[derive(Clone, Default)]
pub struct WasmerBackend {
}

impl WasmBackend for WasmerBackend {
    type Store = WasmerStore;
    type Module = WasmerModule;
    type Context<'c> = WasmerContext<'c>;
    type ContextMut<'c> = WasmerContextMut<'c>;
    type Instance = WasmerInstance;
    type Caller<'c> = ();
    type Imports = ();
    type DynamicFunc = ();
    type Namespace = ();
    type Function = ();
    type Memory = ();
    type MemoryView = ();
    type Wasi = ();

    fn compile(store: &mut Self::Store, wasm: &[u8]) -> WasmBackendResult<Self::Module> {
        wasmer::Module::new(store.inner.engine(), wasm)
            .map_err(|e| {
               WasmBackendError::CompilationError(CompilationError::Message(format!("Wasmer module compilation failed: {}", e)))
            })
            .map(|module| {
                WasmerModule {
                    inner: module
                }
            })
    }
}
