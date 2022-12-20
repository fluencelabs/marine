
pub mod errors;
pub mod exports;
pub mod imports;
pub mod wasi;
pub mod wtype;

use it_memory_traits::{MemoryView};
use wasmer_it::interpreter::wasm;

pub use errors::*;
pub use exports::*;
pub use imports::*;
pub use wasi::*;
pub use wtype::*;

pub trait WasmBackend: Clone + Default + 'static {
    // general
    type Module: Module<Self>;
    type Instance: Instance<Self>;
    type Store: Store<Self> + AsContextMut<Self>;
    type ContextMut<'c>: ContextMut<Self> + AsContextMut<Self> + wasm::structures::Store;
    type Caller<'c>: AsContextMut<Self>;
    // + AsStoreContextMut<Self>;
   // type StoreContextMut: /*AsStoreContextMut<Self> + */wasmer_it::interpreter::wasm::structures::Store;
    // imports/exports -- subject to improvement
    type ImportObject: ImportObject<Self>; // to be replaced with somethink like Linker or Resolver
    type DynamicFunc: DynamicFunc<'static, Self>;
    type MemoryExport: MemoryExport;
    type FunctionExport: FunctionExport;
    type Namespace: Namespace<Self>;

    //type ExportContext: for<'c> ExportContext<'c, Self>;
    type ExportedDynFunc: ExportedDynFunc<Self>;

    // interface types
    type WITMemory: Memory<Self> + it_memory_traits::Memory<Self::WITMemoryView> + Clone + 'static;
    type WITMemoryView: MemoryView + 'static;
    // wasi
    type Wasi: WasiImplementation<Self>;

    fn compile(store: &mut Self::Store, wasm: &[u8]) -> WasmBackendResult<Self::Module>;
}

pub trait Store<WB: WasmBackend> {
    fn new(backend: &WB) -> Self;
}

pub trait AsContextMut<WB: WasmBackend> {
    fn as_context_mut(&mut self) -> <WB as WasmBackend>::ContextMut<'_>;
}

impl<WB: WasmBackend, T: AsContextMut<WB>> AsContextMut<WB> for &mut T {
    fn as_context_mut(&mut self) -> <WB as WasmBackend>::ContextMut<'_> {
        self.as_context_mut()
    }
}

pub trait Module<WB: WasmBackend> {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]>;
    fn instantiate(
        &self,
        store: &mut <WB as WasmBackend>::Store,
        imports: &<WB as WasmBackend>::ImportObject,
    ) -> WasmBackendResult<<WB as WasmBackend>::Instance>;
}

pub trait Instance<WB: WasmBackend> {
    fn export_iter<'a>(
        &'a self,
        store: &'a mut <WB as WasmBackend>::Store,
    ) -> Box<
        dyn Iterator<
                Item = (
                    String,
                    Export<<WB as WasmBackend>::MemoryExport, <WB as WasmBackend>::FunctionExport>,
                ),
            > + 'a,
    >;

    fn memory(
        &self,
        store: &mut <WB as WasmBackend>::Store,
        memory_index: u32,
    ) -> <WB as WasmBackend>::WITMemory;

    fn memory_by_name(
        &self,
        store: &mut <WB as WasmBackend>::Store,
        memory_name: &str,
    ) -> Option<<WB as WasmBackend>::WITMemory>;

    fn get_func_no_args_no_rets<'a>(
        &'a self,
        store: &mut <WB as WasmBackend>::Store,
        name: &str,
    ) -> ResolveResult<Box<dyn Fn(&mut <WB as WasmBackend>::Store) -> RuntimeResult<()> + 'a>>;

    fn get_dyn_func<'a>(
        &'a self,
        store: &mut <WB as WasmBackend>::Store,
        name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::ExportedDynFunc>;
}

