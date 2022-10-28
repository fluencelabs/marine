pub mod errors;
pub mod exports;
pub mod imports;
pub mod wasi;
pub mod wtype;

use it_memory_traits::{MemoryView};

pub use errors::*;
pub use exports::*;
pub use imports::*;
pub use wasi::*;
pub use wtype::*;


pub trait WasmBackend: Clone + 'static {
    // general
    type Module: Module<Self>;
    type Instance: Instance<Self>;
    // imports/exports -- subject to improvement
    type ImportObject: ImportObject<Self>;
    type DynamicFunc: DynamicFunc<'static, Self>;
    type MemoryExport: MemoryExport;
    type FunctionExport: FunctionExport;
    type Namespace: Namespace<Self>;
    type ExportContext: for<'c> ExportContext<'c, Self>;
    type ExportedDynFunc: ExportedDynFunc<Self>;

    // interface types
    type WITMemory: Memory<Self> + it_memory_traits::Memory<Self::WITMemoryView> + Clone + 'static;
    type WITMemoryView: MemoryView + 'static;
    // wasi
    type Wasi: WasiImplementation<Self>;

    fn compile(wasm: &[u8]) -> WasmBackendResult<Self::Module>;
}

pub trait Module<WB: WasmBackend> {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]>;
    fn instantiate(
        &self,
        imports: &<WB as WasmBackend>::ImportObject,
    ) -> WasmBackendResult<<WB as WasmBackend>::Instance>;
}

pub trait Instance<WB: WasmBackend> {
    fn export_iter<'a>(
        &'a self,
    ) -> Box<
        dyn Iterator<
                Item = (
                    String,
                    Export<<WB as WasmBackend>::MemoryExport, <WB as WasmBackend>::FunctionExport>,
                ),
            > + 'a,
    >;

    fn memory(&self, memory_index: u32) -> <WB as WasmBackend>::WITMemory;

    fn memory_by_name(&self, memory_name: &str) -> Option<<WB as WasmBackend>::WITMemory>;

    fn get_func_no_args_no_rets<'a>(
        &'a self,
        name: &str,
    ) -> ResolveResult<Box<dyn Fn() -> RuntimeResult<()> + 'a>>;

    fn get_dyn_func<'a>(
        &'a self,
        name: &str,
    ) -> ResolveResult<<WB as WasmBackend>::ExportedDynFunc>;
}
