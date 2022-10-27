use std::path::PathBuf;
use it_memory_traits::MemoryAccessError;
use marine_wasm_backend_traits::{CallResult, DynamicFunc, Export, ExportContext, ExportedDynFunc, Exports, FuncSig, FunctionExport, ImportObject, Instance, LikeNamespace, Memory, MemoryExport, Module, Namespace, ResolveResult, RuntimeResult, WasiImplementation, WasiState, WasiVersion, WasmBackend, WasmBackendResult, WValue};
use marine_wasm_backend_traits::*;
#[derive(Clone)]
pub struct JsWasmBackend {

}

impl WasmBackend for JsWasmBackend {
    type Module = JsModule;
    type Instance = JsInstance;
    type ImportObject = JsImportObject;
    type Exports = JsExports;
    type DynamicFunc = JsDynamicFunc;
    type MemoryExport = JsMemoryExport;
    type FunctionExport = JsFunctionExport;
    type Namespace = JsNamespace;
    type ExportContext = JsExportContext;
    type ExportedDynFunc = JsExportedDynFunc;
    type WITMemory = JsWITMemory;
    type WITMemoryView = JsWITMemoryView;
    type Wasi = JsWasi;

    fn compile(wasm: &[u8]) -> WasmBackendResult<Self::Module> {
        todo!()
    }
}

// general
pub struct JsModule {

}

impl Module<JsWasmBackend> for JsModule {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]> {
        todo!()
    }

    fn instantiate(&self, imports: &<JsWasmBackend as WasmBackend>::ImportObject) -> WasmBackendResult<<JsWasmBackend as WasmBackend>::Instance> {
        todo!()
    }
}


pub struct JsInstance {

}

impl Instance<JsWasmBackend> for JsInstance {
    fn export_iter<'a>(&'a self) -> Box<dyn Iterator<Item=(String, Export<<JsWasmBackend as WasmBackend>::MemoryExport, <JsWasmBackend as WasmBackend>::FunctionExport>)> + 'a> {
        todo!()
    }

    fn exports(&self) -> &<JsWasmBackend as WasmBackend>::Exports {
        todo!()
    }

    fn import_object(&self) -> &<JsWasmBackend as WasmBackend>::ImportObject {
        todo!()
    }

    fn memory(&self, memory_index: u32) -> <JsWasmBackend as WasmBackend>::WITMemory {
        todo!()
    }
}

// imports
#[derive(Clone)]
pub struct JsImportObject {

}

impl Extend<(String, String, Export<JsMemoryExport, JsFunctionExport>)> for JsImportObject {
    fn extend<T: IntoIterator<Item=(String, String, Export<JsMemoryExport, JsFunctionExport>)>>(&mut self, iter: T) {
        todo!()
    }
}

impl ImportObject<JsWasmBackend> for JsImportObject {
    fn new() -> Self {
        todo!()
    }

    fn extend_with_self(&mut self, other: Self) {
        todo!()
    }

    fn register<S>(&mut self, name: S, namespace: <JsWasmBackend as WasmBackend>::Namespace) -> Option<Box<dyn LikeNamespace<JsWasmBackend>>> where S: Into<String> {
        todo!()
    }

    fn get_memory_env(&self) -> Option<Export<<JsWasmBackend as WasmBackend>::MemoryExport, <JsWasmBackend as WasmBackend>::FunctionExport>> {
        todo!()
    }
}

pub struct JsNamespace {

}

macro_rules! impl_insert_fn {
    ($($name:ident: $arg:ty),* => $rets:ty) => {
        impl InsertFn<JsWasmBackend, ($($arg,)*), $rets> for JsNamespace {
            fn insert_fn<F>(&mut self, name: impl Into<String>, func: F)
            where F:'static + Fn(&mut JsExportContext, ($($arg,)*)) -> $rets + std::marker::Send {
                let func = move |$($name: $arg),*| {
                    unsafe {
                        let mut ctx = JsExportContext {};

                        func(&mut ctx, ($($name,)*))
                    }
                };

                todo!()
            }
        }
    };
}

impl_insert_fn!(=> ());
impl_insert_fn!(A: i32 => ());
impl_insert_fn!(A: i32, B: i32 => ());
impl_insert_fn!(A: i32, B: i32, C: i32 => ());
impl_insert_fn!(A: i32, B: i32, C: i32, D: i32 => ());


impl LikeNamespace<JsWasmBackend> for JsNamespace {}

impl Namespace<JsWasmBackend> for JsNamespace {
    fn new() -> Self {
        todo!()
    }

    fn insert(&mut self, name: impl Into<String>, func: <JsWasmBackend as WasmBackend>::DynamicFunc) {
        todo!()
    }
}

pub struct JsDynamicFunc {
}

impl<'a> DynamicFunc<'a, JsWasmBackend> for JsDynamicFunc {
    fn new<'c, F>(sig: FuncSig, func: F) -> Self where F: Fn(&mut <JsWasmBackend as WasmBackend>::ExportContext, &[WValue]) -> Vec<WValue> + 'static {
        todo!()
    }
}

//exports

pub struct JsExports {

}

impl Exports<JsWasmBackend> for JsExports {
    fn get_func_no_args_no_rets<'a>(&'a self, name: &str) -> ResolveResult<Box<dyn Fn() -> RuntimeResult<()> + 'a>> {
        todo!()
    }

    fn get_dyn_func<'a>(&'a self, name: &str) -> ResolveResult<<JsWasmBackend as WasmBackend>::ExportedDynFunc> {
        todo!()
    }
}

pub struct JsMemoryExport {

}

impl MemoryExport for JsMemoryExport {

}

pub struct JsFunctionExport {

}

impl FunctionExport for JsFunctionExport {

}

pub struct JsExportContext {

}

macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl<'c> FuncGetter<'c, $args, $rets> for JsExportContext {
            unsafe fn get_func(
                &'c mut self,
                name: &str,
            ) -> Result<Box<dyn FnMut($args) -> Result<$rets, RuntimeError> + 'c>, ResolveError>
            {
                todo!()
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

impl<'a> ExportContext<'a, JsWasmBackend> for JsExportContext {
    fn memory(&self, memory_index: u32) -> <JsWasmBackend as WasmBackend>::WITMemory {
        todo!()
    }
}

pub struct JsExportedDynFunc {

}

impl ExportedDynFunc<JsWasmBackend> for JsExportedDynFunc {
    fn signature(&self) -> &FuncSig {
        todo!()
    }

    fn call(&self, args: &[WValue]) -> CallResult<Vec<WValue>> {
        todo!()
    }
}

// Interface types
#[derive(Clone)]
pub struct JsWITMemory {

}

impl it_memory_traits::Memory<JsWITMemoryView> for JsWITMemory {
    fn view(&self) -> JsWITMemoryView {
        todo!()
    }
}

impl Memory<JsWasmBackend> for JsWITMemory {
    fn new(export: <JsWasmBackend as WasmBackend>::MemoryExport) -> Self {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }
}

pub struct JsWITMemoryView {

}

impl it_memory_traits::MemoryReadable for JsWITMemoryView {
    fn read_byte(&self, offset: u32) -> u8 {
        todo!()
    }

    fn read_array<const COUNT: usize>(&self, offset: u32) -> [u8; COUNT] {
        todo!()
    }

    fn read_vec(&self, offset: u32, size: u32) -> Vec<u8> {
        todo!()
    }
}

impl it_memory_traits::MemoryWritable for JsWITMemoryView {
    fn write_byte(&self, offset: u32, value: u8) {
        todo!()
    }

    fn write_bytes(&self, offset: u32, bytes: &[u8]) {
        todo!()
    }
}

impl it_memory_traits::MemoryView for JsWITMemoryView {
    fn check_bounds(&self, offset: u32, size: u32) -> Result<(), MemoryAccessError> {
        todo!()
    }
}

// Wasi

pub struct JsWasi {}

impl WasiImplementation<JsWasmBackend> for JsWasi {
    fn generate_import_object_for_version(version: WasiVersion, args: Vec<Vec<u8>>, envs: Vec<Vec<u8>>, preopened_files: Vec<PathBuf>, mapped_dirs: Vec<(String, PathBuf)>) -> Result<<JsWasmBackend as WasmBackend>::ImportObject, String> {
        todo!()
    }

    fn get_wasi_state<'s>(instance: &'s mut <JsWasmBackend as WasmBackend>::Instance) -> Box<dyn WasiState + 's> {
        todo!()
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

