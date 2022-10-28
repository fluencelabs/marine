use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use it_memory_traits::MemoryAccessError;
use marine_wasm_backend_traits::{CallResult, DynamicFunc, Export, ExportContext, ExportedDynFunc, Exports, FuncSig, FunctionExport, ImportObject, Instance, LikeNamespace, Memory, MemoryExport, Module, Namespace, ResolveResult, RuntimeResult, WasiImplementation, WasiState, WasiVersion, WasmBackend, WasmBackendResult, WValue};
use marine_wasm_backend_traits::*;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use multimap::MultiMap;
use wasmtime::AsContextMut;

#[derive(Clone)]
pub struct WasmtimeWasmBackend {

}

impl WasmtimeWasmBackend {
    fn custom_sections(bytes: &[u8]) -> Result<MultiMap<String, Vec<u8>>, String> {
        use wasmparser::{Parser, Payload, Result};
        Parser::new(0)
            .parse_all(bytes)
            .filter_map(|payload| {
                let payload = match payload {
                    Ok(s ) => s,
                    Err(e) => return Some(Err(e.to_string())),
                };
                match payload {
                    Payload::CustomSection (reader ) => {
                        let name = reader.name().to_string();
                        let data = reader.data().to_vec();
                        Some(Ok((name, data)))
                    },
                    _ => None,
                }
            })
            .collect()

    }
}

impl WasmBackend for WasmtimeWasmBackend {
    type Module = WasmtimeModule;
    type Instance = WasmtimeInstance;
    type ImportObject = WasmtimeImportObject;
    type Exports = WasmtimeExports;
    type DynamicFunc = WasmtimeDynamicFunc;
    type MemoryExport = WasmtimeMemoryExport;
    type FunctionExport = WasmtimeFunctionExport;
    type Namespace = WasmtimeNamespace;
    type ExportContext = WasmtimeExportContext;
    type ExportedDynFunc = WasmtimeExportedDynFunc;
    type WITMemory = WasmtimeWITMemory;
    type WITMemoryView = WasmtimeWITMemoryView;
    type Wasi = WasmtimeWasi;

    fn compile(wasm: &[u8]) -> WasmBackendResult<Self::Module> {
        let engine = Rc::new(wasmtime::Engine::default());
        let module = wasmtime::Module::new(&engine, wasm).unwrap();//todo convert error;
        let custom_sections = WasmtimeWasmBackend::custom_sections(wasm).unwrap(); //todo convert error;
        Ok(WasmtimeModule { engine, custom_sections, module })
    }
}

// general
pub struct WasmtimeModule {
    engine: Rc<wasmtime::Engine>,
    custom_sections: MultiMap<String, Vec<u8>>,
    module: wasmtime::Module
}

impl Module<WasmtimeWasmBackend> for WasmtimeModule {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]> {
        self.custom_sections
            .get_vec(key)
            .map(|value| value.as_slice())
    }

    fn instantiate(&self, imports: &<WasmtimeWasmBackend as WasmBackend>::ImportObject) -> WasmBackendResult<<WasmtimeWasmBackend as WasmBackend>::Instance> {
        let store = RefCell::new(wasmtime::Store::new(&self.engine, ()));
        let instance = wasmtime::Instance::new(store.borrow_mut().deref_mut(), &self.module, &imports.imports).unwrap(); // todo handle error
        Ok(WasmtimeInstance {store, instance})
    }
}

pub struct WasmtimeInstance {
    store: RefCell<wasmtime::Store<()>>,
    instance: wasmtime::Instance,
}

impl Instance<WasmtimeWasmBackend> for WasmtimeInstance {
    fn export_iter<'a>(&'a self) -> Box<dyn Iterator<Item=(String, Export<<WasmtimeWasmBackend as WasmBackend>::MemoryExport, <WasmtimeWasmBackend as WasmBackend>::FunctionExport>)> + 'a> {
        /*let iter = self
            .instance
            .exports(self.store.deref().borrow_mut())
            .map(|export| {
                let name = export.name().to_string();
                let export = match export.into_extern() {
                    wasmtime::Extern::Memory(memory) => {
                        Export::Memory(WasmtimeMemoryExport{memory})
                    },
                    wasmtime::Extern::Func(func) => {
                        Export::Function(WasmtimeFunctionExport{func})
                    },
                    _ => Export::Other
                };
                (name, export)
            });

        Box::new(iter)*/
        todo!()
    }

    fn exports(&self) -> &<WasmtimeWasmBackend as WasmBackend>::Exports {
        let exports = self
            .instance
            .exports(self.store.deref().borrow_mut())
            .map(|export| {
                let name = export.name().to_string();
                let export = match export.into_extern() {
                    wasmtime::Extern::Memory(memory) => {
                        Export::Memory(WasmtimeMemoryExport{memory})
                    },
                    wasmtime::Extern::Func(func) => {
                        Export::Function(WasmtimeFunctionExport{func})
                    },
                    _ => Export::Other
                };
                (name, export)
            })
            .collect::<Vec<_>>();
        WasmtimeExports {exports}
    }

    fn memory(&self, memory_index: u32) -> <WasmtimeWasmBackend as WasmBackend>::WITMemory {
        let memory = self
            .instance
            .exports(self.store.deref().borrow_mut().deref_mut())
            .filter_map(wasmtime::Export::into_memory)
            .nth(memory_index as usize)
            .unwrap(); // todo change api to handle error

        WasmtimeWITMemory {memory}
    }

    fn memory_by_name(&self, memory_name: &str) -> Option<<WasmtimeWasmBackend as WasmBackend>::WITMemory> {
        let memory = self
            .instance
            .get_memory(self.store.deref().borrow_mut().deref_mut(), memory_name);

        memory.map(WasmtimeWITMemory::new)
    }
}

// imports
#[derive(Clone)]
pub struct WasmtimeImportObject {
    imports: Vec<wasmtime::Extern>
}

impl ImportObject<WasmtimeWasmBackend> for WasmtimeImportObject {
    fn new() -> Self {
        todo!()
    }

    fn register<S>(&mut self, name: S, namespace: <WasmtimeWasmBackend as WasmBackend>::Namespace) -> Option<Box<dyn LikeNamespace<WasmtimeWasmBackend>>> where S: Into<String> {
        todo!()
    }
}

pub struct WasmtimeNamespace {

}

macro_rules! impl_insert_fn {
    ($($name:ident: $arg:ty),* => $rets:ty) => {
        impl InsertFn<WasmtimeWasmBackend, ($($arg,)*), $rets> for WasmtimeNamespace {
            fn insert_fn<F>(&mut self, name: impl Into<String>, func: F)
            where F:'static + Fn(&mut WasmtimeExportContext, ($($arg,)*)) -> $rets + std::marker::Send {
                let func = move |$($name: $arg),*| {
                    unsafe {
                        let mut ctx = WasmtimeExportContext {};

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


impl LikeNamespace<WasmtimeWasmBackend> for WasmtimeNamespace {}

impl Namespace<WasmtimeWasmBackend> for WasmtimeNamespace {
    fn new() -> Self {
        todo!()
    }

    fn insert(&mut self, name: impl Into<String>, func: <WasmtimeWasmBackend as WasmBackend>::DynamicFunc) {
        todo!()
    }
}

pub struct WasmtimeDynamicFunc {
}

impl<'a> DynamicFunc<'a, WasmtimeWasmBackend> for WasmtimeDynamicFunc {
    fn new<'c, F>(sig: FuncSig, func: F) -> Self where F: Fn(&mut <WasmtimeWasmBackend as WasmBackend>::ExportContext, &[WValue]) -> Vec<WValue> + 'static {
        todo!()
    }
}

//exports

pub struct WasmtimeExports {
    exports: Vec<(String, Export<WasmtimeMemoryExport, WasmtimeFunctionExport>)>
}

impl Exports<WasmtimeWasmBackend> for WasmtimeExports {
    fn get_func_no_args_no_rets<'a>(&'a self, name: &str) -> ResolveResult<Box<dyn Fn() -> RuntimeResult<()> + 'a>> {
        todo!()
    }

    fn get_dyn_func<'a>(&'a self, name: &str) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::ExportedDynFunc> {
        todo!()
    }
}

pub struct WasmtimeMemoryExport {
    memory: wasmtime::Memory
}

impl MemoryExport for WasmtimeMemoryExport {}

pub struct WasmtimeFunctionExport {
    func: wasmtime::Func
}

impl FunctionExport for WasmtimeFunctionExport {}

pub struct WasmtimeExportContext {

}

macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl<'c> FuncGetter<'c, $args, $rets> for WasmtimeExportContext {
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

impl<'a> ExportContext<'a, WasmtimeWasmBackend> for WasmtimeExportContext {
    fn memory(&self, memory_index: u32) -> <WasmtimeWasmBackend as WasmBackend>::WITMemory {
        todo!()
    }
}

pub struct WasmtimeExportedDynFunc {

}

impl ExportedDynFunc<WasmtimeWasmBackend> for WasmtimeExportedDynFunc {
    fn signature(&self) -> &FuncSig {
        todo!()
    }

    fn call(&self, args: &[WValue]) -> CallResult<Vec<WValue>> {
        todo!()
    }
}

// Interface types
#[derive(Clone)]
pub struct WasmtimeWITMemory {
    memory: wasmtime::Memory
}

impl WasmtimeWITMemory {
    fn new(memory: wasmtime::Memory) -> Self {
        Self {memory}
    }
}

impl it_memory_traits::Memory<WasmtimeWITMemoryView> for WasmtimeWITMemory {
    fn view(&self) -> WasmtimeWITMemoryView {
        todo!()
    }
}

impl Memory<WasmtimeWasmBackend> for WasmtimeWITMemory {
    fn new(export: <WasmtimeWasmBackend as WasmBackend>::MemoryExport) -> Self {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }
}

pub struct WasmtimeWITMemoryView {

}

impl it_memory_traits::MemoryReadable for WasmtimeWITMemoryView {
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

impl it_memory_traits::MemoryWritable for WasmtimeWITMemoryView {
    fn write_byte(&self, offset: u32, value: u8) {
        todo!()
    }

    fn write_bytes(&self, offset: u32, bytes: &[u8]) {
        todo!()
    }
}

impl it_memory_traits::MemoryView for WasmtimeWITMemoryView {
    fn check_bounds(&self, offset: u32, size: u32) -> Result<(), MemoryAccessError> {
        todo!()
    }
}

// Wasi

pub struct WasmtimeWasi {}

impl WasiImplementation<WasmtimeWasmBackend> for WasmtimeWasi {
    fn generate_import_object_for_version(version: WasiVersion, args: Vec<Vec<u8>>, envs: Vec<Vec<u8>>, preopened_files: Vec<PathBuf>, mapped_dirs: Vec<(String, PathBuf)>) -> Result<<WasmtimeWasmBackend as WasmBackend>::ImportObject, String> {
        todo!()
    }

    fn get_wasi_state<'s>(instance: &'s mut <WasmtimeWasmBackend as WasmBackend>::Instance) -> Box<dyn WasiState + 's> {
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

