extern crate core;

use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use it_memory_traits::MemoryAccessError;
use marine_wasm_backend_traits::*;
use std::collections::HashMap;
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use multimap::MultiMap;
use wasmtime::{AsContextMut, Extern, Func, Linker};
use crate::utils::{sig_to_fn_ty, val_to_wvalue, val_type_to_wtype, wvalue_to_val};

use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;
use marine_wasm_backend_traits::WasmBackendError;
use marine_wasm_backend_traits::ResolveError;


mod utils;

#[derive(Clone, Default)]
pub struct WasmtimeWasmBackend {
    engine: wasmtime::Engine,
}

impl WasmtimeWasmBackend {
    fn custom_sections(bytes: &[u8]) -> Result<MultiMap<String, Vec<u8>>, String> {
        use wasmparser::{Parser, Payload, Result};
        Parser::new(0)
            .parse_all(bytes)
            .filter_map(|payload| {
                let payload = match payload {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e.to_string())),
                };
                match payload {
                    Payload::CustomSection(reader) => {
                        let name = reader.name().to_string();
                        let data = reader.data().to_vec();
                        Some(Ok((name, data)))
                    }
                    _ => None,
                }
            })
            .collect()
    }
}

impl WasmBackend for WasmtimeWasmBackend {
    type Module = WasmtimeModule;
    type Instance = WasmtimeInstance;
    type Store = WasmtimeStore;
    type ContextMut<'c> = WasmtimeStoreContextMut<'c>;
    type Caller<'c> = WasmtimeCaller<'c>;
    //type StoreContextMut = WasmtimeStoreContextMut;
    //type StoreContextMut = WasmtimeStoreContextMut;
    type ImportObject = WasmtimeImportObject;
    type DynamicFunc = WasmtimeDynamicFunc;
    type MemoryExport = WasmtimeMemoryExport;
    type FunctionExport = WasmtimeFunctionExport;
    type Namespace = WasmtimeNamespace;
    type ExportedDynFunc = WasmtimeExportedDynFunc;
    type WITMemory = WasmtimeWITMemory;
    type WITMemoryView = WasmtimeWITMemoryView;
    type Wasi = WasmtimeWasi;

    fn compile(store: &mut WasmtimeStore, wasm: &[u8]) -> WasmBackendResult<Self::Module> {
        let module = wasmtime::Module::new(store.store.engine(), wasm).unwrap(); //todo convert error;
        let custom_sections = WasmtimeWasmBackend::custom_sections(wasm).unwrap(); //todo convert error;
        Ok(WasmtimeModule {
            custom_sections,
            module,
        })
    }
}

#[derive(Default)]
pub struct StoreState {
    wasi: Vec<WasiCtx>, //todo switch to Pool or something
}
// general
pub struct WasmtimeStore {
    store: wasmtime::Store<StoreState>,
}

impl Store<WasmtimeWasmBackend> for WasmtimeStore {
    fn new(backend: &WasmtimeWasmBackend) -> Self {
        Self {
            store: wasmtime::Store::new(&backend.engine, StoreState::default()),
        }
    }
}
/*
impl AsStoreContextMut<WasmtimeWasmBackend> for WasmtimeStore {
    fn store_context_mut<'c, CTX: ExportContext<'c, WasmtimeWasmBackend>>(&mut self) -> ContextMut<'c, WasmtimeWasmBackend, WasmtimeExportContext<'c>> {
        ContextMut::Store(&mut self)
    }
}*/

impl marine_wasm_backend_traits::AsContextMut<WasmtimeWasmBackend> for WasmtimeStore {
    fn as_context_mut(&mut self) -> WasmtimeStoreContextMut<'_> {
        WasmtimeStoreContextMut {
            ctx: self.store.as_context_mut(),
        }
    }
}

pub struct WasmtimeStoreContextMut<'c> {
    ctx: wasmtime::StoreContextMut<'c, StoreState>,
}

impl<'c> ContextMut<WasmtimeWasmBackend> for WasmtimeStoreContextMut<'c> {}
//impl StoreContextMut<WasmtimeWasmBackend> for WasmtimeStoreContextMut {}

impl<'c> marine_wasm_backend_traits::AsContextMut<WasmtimeWasmBackend>
    for WasmtimeStoreContextMut<'c>
{
    fn as_context_mut(&mut self) -> WasmtimeStoreContextMut<'_> {
        WasmtimeStoreContextMut {
            ctx: self.ctx.as_context_mut(),
        }
    }
}
pub struct WasmtimeCaller<'c> {
    caller: wasmtime::Caller<'c, StoreState>,
}

impl<'c> Caller<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn memory(&mut self, memory_index: u32) -> <WasmtimeWasmBackend as WasmBackend>::WITMemory {
        let memory = self.caller.get_export("memory").unwrap(); // todo: handle error

        WasmtimeWITMemory {
            memory: memory.into_memory().unwrap(), // todo: handle error
        }
    }
}

impl<'c> marine_wasm_backend_traits::AsContextMut<WasmtimeWasmBackend> for WasmtimeCaller<'c> {
    fn as_context_mut(&mut self) -> <WasmtimeWasmBackend as WasmBackend>::ContextMut<'_> {
        WasmtimeStoreContextMut {
            ctx: wasmtime::AsContextMut::as_context_mut(&mut self.caller),
        }
    }
}

pub struct WasmtimeModule {
    custom_sections: MultiMap<String, Vec<u8>>,
    module: wasmtime::Module,
}

impl Module<WasmtimeWasmBackend> for WasmtimeModule {
    fn custom_sections(&self, key: &str) -> Option<&[Vec<u8>]> {
        self.custom_sections
            .get_vec(key)
            .map(|value| value.as_slice())
    }

    fn instantiate(
        &self,
        store: &mut WasmtimeStore,
        imports: &WasmtimeImportObject,
    ) -> WasmBackendResult<<WasmtimeWasmBackend as WasmBackend>::Instance> {
        let instance = imports
            .linker
            .instantiate(&mut store.store, &self.module)
            .unwrap(); // todo handle error
        Ok(WasmtimeInstance { instance })
    }
}

pub struct WasmtimeInstance {
    instance: wasmtime::Instance,
}

impl Instance<WasmtimeWasmBackend> for WasmtimeInstance {
    fn export_iter<'a>(
        &'a self,
        store: &'a mut WasmtimeStore,
    ) -> Box<dyn Iterator<Item = (String, Export<WasmtimeMemoryExport, WasmtimeFunctionExport>)> + 'a>
    {
        let iter = self.instance.exports(&mut store.store).map(|export| {
            let name = export.name().to_string();
            let export = match export.into_extern() {
                wasmtime::Extern::Memory(memory) => Export::Memory(WasmtimeMemoryExport { memory }),
                wasmtime::Extern::Func(func) => Export::Function(WasmtimeFunctionExport { func }),
                _ => Export::Other,
            };
            (name, export)
        });

        Box::new(iter)
    }

    fn memory(
        &self,
        store: &mut WasmtimeStore,
        memory_index: u32,
    ) -> <WasmtimeWasmBackend as WasmBackend>::WITMemory {
        let memory = self
            .instance
            .exports(&mut store.store)
            .filter_map(wasmtime::Export::into_memory)
            .nth(memory_index as usize)
            .unwrap(); // todo change api to handle error

        WasmtimeWITMemory { memory }
    }

    fn memory_by_name(
        &self,
        store: &mut WasmtimeStore,
        memory_name: &str,
    ) -> Option<<WasmtimeWasmBackend as WasmBackend>::WITMemory> {
        let memory = self.instance.get_memory(&mut store.store, memory_name);

        memory.map(WasmtimeWITMemory::new)
    }

    fn get_func_no_args_no_rets<'a>(
        &'a self,
        store: &mut WasmtimeStore,
        name: &str,
    ) -> ResolveResult<Box<dyn Fn(&mut WasmtimeStore) -> RuntimeResult<()> + Sync + Send + 'a>> {
        let func = match self.instance.get_func(&mut store.store, name) {
            None => return Err(ResolveError::Message(format!("no such function {}", name))),
            Some(func) => func,
        };

        let typed = func.typed::<(), (), _>(&store.store).unwrap(); // todo handle error
        Ok(Box::new(move |store: &mut WasmtimeStore| {
            Ok(typed.call(&mut store.store, ()).unwrap()) //todo handle error
        }))
    }

    fn get_dyn_func<'a>(
        &'a self,
        store: &mut WasmtimeStore,
        name: &str,
    ) -> ResolveResult<<WasmtimeWasmBackend as WasmBackend>::ExportedDynFunc> {
        let func = self.instance.get_func(&mut store.store, name).unwrap(); // todo handle None
        let ty = func.ty(&store.store);
        let params = ty
            .params()
            .map(|ty| {
                val_type_to_wtype(&ty).unwrap() // todo handle error
            })
            .collect::<Vec<_>>();
        let rets = ty
            .results()
            .map(|ty| {
                val_type_to_wtype(&ty).unwrap() // todo handle error
            })
            .collect::<Vec<_>>();

        let sig = FuncSig::new(params, rets);
        Ok(WasmtimeExportedDynFunc {
            func,
            signature: sig,
        })
    }
}

// imports
#[derive(Clone)]
pub struct WasmtimeImportObject {
    linker: wasmtime::Linker<StoreState>,
}

impl ImportObject<WasmtimeWasmBackend> for WasmtimeImportObject {
    fn new(store: &mut WasmtimeStore) -> Self {
        Self {
            linker: wasmtime::Linker::new(store.store.engine()),
        }
    }

    fn register<S>(
        &mut self,
        module: S,
        namespace: WasmtimeNamespace,
    ) -> Option<Box<dyn LikeNamespace<WasmtimeWasmBackend>>>
    where
        S: Into<String>,
    {
        let module: String = module.into();
        for (name, func) in namespace.functions {
            func(self, &module, &name).unwrap(); // todo handle error
        }

        None // todo handle collisions
    }
}

pub struct WasmtimeNamespace {
    functions: Vec<(
        String,
        Box<dyn FnOnce(&mut WasmtimeImportObject, &str, &str) -> Result<(), String> + 'static>,
    )>,
}

macro_rules! impl_insert_fn {
    ($($name:ident: $arg:ty),* => $rets:ty) => {
        impl InsertFn<WasmtimeWasmBackend, ($($arg,)*), $rets> for WasmtimeNamespace {
            fn insert_fn<F>(&mut self, name: impl Into<String>, func: F)
            where F:
                Fn(&mut WasmtimeCaller, ($($arg,)*)) -> $rets
                + Sync
                + Send
                + 'static {
                let name: String = name.into();
                println!("calling insert_fn with {}", &name);
                let inserter = move |linker: &mut WasmtimeImportObject, module: &str, name: &str| {
                    let func = move |caller: wasmtime::Caller<'_, StoreState>, $($name: $arg),*| {
                            let mut ctx = WasmtimeCaller {caller};

                            func(&mut ctx, ($($name,)*))
                    };

                    println!("adding function {} {}", module, name);
                    linker.linker.func_wrap(module, name, func).unwrap(); // todo handle error
                    Ok(())
                };

                self.functions.push((name.into(), Box::new(inserter)))
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
        Self {
            functions: Vec::new(),
        }
    }

    fn insert(&mut self, name: impl Into<String>, func: WasmtimeDynamicFunc) {
        let inserter = move |linker: &mut WasmtimeImportObject, module: &str, name: &str| {
            let sig = func.sig.clone();
            let wrapper = move |
                caller: wasmtime::Caller<'_, StoreState>,
                args: &[wasmtime::Val],
                rets: &mut [wasmtime::Val]
            | {
                let mut ctx = WasmtimeCaller {caller};
                let args = args
                    .iter()
                    .map(|val| {
                        val_to_wvalue(val).unwrap() //todo handle error
                    })
                    .collect::<Vec<WValue>>();
                let result = (func.data)(ctx, &args);
                for index in 0..result.len() {
                    rets[index] = wvalue_to_val(&result[index]);
                }

                Ok(())
            };
            let ty = sig_to_fn_ty(&sig);
            linker.linker.func_new(module, name, ty, wrapper).unwrap(); // todo handle error
            Ok(())
        };
        self.functions.push((name.into(), Box::new(inserter)));
    }
}

pub struct WasmtimeDynamicFunc {
    data: Box<
        dyn for<'c> Fn(<WasmtimeWasmBackend as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue> + Sync + Send + 'static,
    >,
    sig: FuncSig,
}

impl<'a> DynamicFunc<'a, WasmtimeWasmBackend> for WasmtimeDynamicFunc {
    fn new<F>(
        _store: &mut <WasmtimeWasmBackend as WasmBackend>::ContextMut<'_>,
        sig: FuncSig,
        func: F,
    ) -> Self
    where
        F: for<'c> Fn(<WasmtimeWasmBackend as WasmBackend>::Caller<'c>, &[WValue]) -> Vec<WValue>
            + Sync
            + Send
            + 'static,
    {
        WasmtimeDynamicFunc {
            data: Box::new(func),
            sig
        }
    }
}

//exports

pub struct WasmtimeExports {
    exports: Vec<(String, Export<WasmtimeMemoryExport, WasmtimeFunctionExport>)>,
}

pub struct WasmtimeMemoryExport {
    memory: wasmtime::Memory,
}

impl MemoryExport for WasmtimeMemoryExport {}

pub struct WasmtimeFunctionExport {
    func: wasmtime::Func,
}

impl FunctionExport for WasmtimeFunctionExport {}
/*
pub struct WasmtimeExportContext<'a> {
    caller: Caller<'a, ()>,
}
*/
macro_rules! impl_func_getter {
    ($args:ty, $rets:ty) => {
        impl<'c> FuncGetter<WasmtimeWasmBackend, $args, $rets> for WasmtimeCaller<'c> {
            unsafe fn get_func(
                &mut self,
                name: &str,
            ) -> Result<
                Box<
                    dyn FnMut(
                            &mut WasmtimeStoreContextMut<'_>,
                            $args,
                        ) -> Result<$rets, RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let export = self.caller.get_export(name).unwrap(); //todo handle error
                match export {
                    Extern::Func(f) => {
                        let f = f.typed(&mut self.caller).unwrap(); //todo handle error
                        let closure = move |store: &mut WasmtimeStoreContextMut<'_>, args| {

                            let rets = f.call(&mut store.ctx, args).unwrap(); //todo handle error
                            return Ok(rets)
                        };


                        Ok(Box::new(closure))
                    }
                    Extern::Memory(m) => {
                        panic!("caller.get_export returned memoryn");
                    }
                    _ => {
                        panic!("caller.get_export returned neither memory nor func")
                    }
                }
            }
        }
    };
}
    /*
impl<'c> FuncGetter<WasmtimeWasmBackend, (), ()> for WasmtimeCaller<'c> {
    unsafe fn get_func<'s>(
        &'s mut self,
        name: &str,
    ) -> Result<
        Box<dyn FnMut(&mut WasmtimeStoreContextMut<'_>, ()) -> Result<(), RuntimeError> + 's>,
        ResolveError,
    > {
        let export = self.caller.get_export(name).unwrap(); //todo handle error
        match export {
            Extern::Func(f) => {
                let f = f.typed(&mut self.caller).unwrap(); //todo handle error
                let closure = move |store, $args| {

                    f.call(store.ctx, $args) //todo handle error
                    return Ok(())
                };


                Ok(Box::new(closure))
            }
            Extern::Memory(m) => {
                panic!("caller.get_export returned memoryn");
            }
            _ => {
                panic!("caller.get_export returned neither memory nor func")
            }
        }
    }
}*/
impl_func_getter!((i32, i32), i32);
impl_func_getter!((i32, i32), ());
impl_func_getter!(i32, i32);
impl_func_getter!(i32, ());
impl_func_getter!((), i32);
impl_func_getter!((), ());

impl<'c> wasmtime::AsContext for WasmtimeStoreContextMut<'c> {
    type Data = StoreState;

    fn as_context(&self) -> wasmtime::StoreContext<'_, Self::Data> {
        self.ctx.as_context()
    }
}
impl<'c> wasmtime::AsContextMut for WasmtimeStoreContextMut<'c> {
    fn as_context_mut(&mut self) -> wasmtime::StoreContextMut<'_, Self::Data> {
        self.ctx.as_context_mut()
    }
}

pub struct WasmtimeExportedDynFunc {
    func: wasmtime::Func,
    signature: FuncSig,
}

impl ExportedDynFunc<WasmtimeWasmBackend> for WasmtimeExportedDynFunc {
    fn signature<'c>(&self, store: WasmtimeStoreContextMut) -> &FuncSig {
        &self.signature
    }

    fn call<'c>(&self, store: WasmtimeStoreContextMut, args: &[WValue]) -> CallResult<Vec<WValue>> {
        let args = args.iter().map(wvalue_to_val).collect::<Vec<_>>();

        let mut rets = Vec::new();
        rets.resize(
            self.signature.returns().collect::<Vec<_>>().len(),
            wasmtime::Val::null(),
        ); // todo make O(1), not O(n)
        self.func.call(store, &args, &mut rets).unwrap(); // todo handle error
        let rets = rets
            .iter()
            .map(val_to_wvalue)
            .collect::<Result<Vec<_>, ()>>()
            .unwrap(); // todo handle error
        Ok(rets)
    }
}

// Interface types
#[derive(Clone)]
pub struct WasmtimeWITMemory {
    memory: wasmtime::Memory,
}

impl WasmtimeWITMemory {
    fn new(memory: wasmtime::Memory) -> Self {
        Self { memory }
    }
}

impl it_memory_traits::Memory<WasmtimeWITMemoryView, DelayedContextLifetime<WasmtimeWasmBackend>> for WasmtimeWITMemory {
    fn view(&self) -> WasmtimeWITMemoryView {
        WasmtimeWITMemoryView {
            view: self.memory.clone()
        }
    }
}

impl Memory<WasmtimeWasmBackend> for WasmtimeWITMemory {
    fn new(export: WasmtimeMemoryExport) -> Self {
        WasmtimeWITMemory {
            memory: export.memory,
        }
    }

    fn size(&self, store: &mut WasmtimeStoreContextMut<'_>) -> usize {
        self.memory.size(store) as usize
    }
}

pub struct WasmtimeWITMemoryView {
    view: wasmtime::Memory
}

impl it_memory_traits::MemoryReadable<DelayedContextLifetime<WasmtimeWasmBackend>> for WasmtimeWITMemoryView {
    fn read_byte(&self, store: &mut WasmtimeStoreContextMut<'_>, offset: u32) -> u8 {
        let mut value = [0u8];
        self.view.read(&mut store.ctx, offset as usize, &mut value).unwrap(); // todo handle error;
        value[0]
    }

    fn read_array<const COUNT: usize>(&self, store: &mut WasmtimeStoreContextMut<'_>, offset: u32) -> [u8; COUNT] {
        let mut value = [0u8; COUNT];
        self.view.read(&mut store.ctx, offset as usize, &mut value).unwrap(); // todo handle error;
        value
    }

    fn read_vec(&self, store: &mut WasmtimeStoreContextMut<'_>, offset: u32, size: u32) -> Vec<u8> {
        let mut value = vec![0u8; size as usize];
        self.view.read(&mut store.ctx, offset as usize, &mut value).unwrap(); // todo handle error;
        value
    }
}

impl it_memory_traits::MemoryWritable<DelayedContextLifetime<WasmtimeWasmBackend>> for WasmtimeWITMemoryView {
    fn write_byte(&self, store: &mut WasmtimeStoreContextMut<'_>, offset: u32, value: u8) {
        let buffer = [value];
        self.view.write(&mut store.ctx, offset as usize, &buffer).unwrap() // todo handle error
    }

    fn write_bytes(&self, store: &mut WasmtimeStoreContextMut<'_>, offset: u32, bytes: &[u8]) {
        self.view.write(&mut store.ctx, offset as usize, bytes).unwrap() // todo handle error
    }
}

impl it_memory_traits::MemoryView<DelayedContextLifetime<WasmtimeWasmBackend>> for WasmtimeWITMemoryView {
    fn check_bounds(&self, store: &mut WasmtimeStoreContextMut<'_>, offset: u32, size: u32) -> Result<(), MemoryAccessError> {
        let memory_size = self.view.size(&mut store.ctx);
        if memory_size <= (offset + size) as u64 {
            Err(MemoryAccessError::OutOfBounds {
                offset,
                size,
                memory_size: memory_size as u32 // todo rewrite api when memory64 arrives
            })
        } else {
            Ok(())
        }
    }
}

// Wasi

pub struct WasmtimeWasi {}

impl WasiImplementation<WasmtimeWasmBackend> for WasmtimeWasi {
    fn generate_import_object_for_version(
        store: &mut WasmtimeStoreContextMut<'_>,
        version: WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<Vec<u8>>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<<WasmtimeWasmBackend as WasmBackend>::ImportObject, String> {
        let id = store.ctx.data().wasi.len();
        let mut linker = Linker::<StoreState>::new(store.ctx.engine());
        wasmtime_wasi::add_to_linker(&mut linker, move |s: &mut StoreState| &mut s.wasi[id])
            .unwrap(); // todo handle error
                       // Create a WASI context and put it in a Store; all instances in the storex
                       // share this context. `WasiCtxBuilder` provides a number of ways to
                       // configure what the target program will have access to.
        let args = args
            .into_iter()
            .map(|arg| unsafe { String::from_utf8_unchecked(arg) })
            .collect::<Vec<String>>();
        // todo pass all data to ctx
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()
            .args(&args)
            .unwrap() // todo handle error
            .build();
        let state = store.ctx.data_mut();
        state.wasi.push(wasi_ctx); //todo handle duplicate
        Ok(WasmtimeImportObject { linker })
    }

    fn get_wasi_state<'s>(
        instance: &'s mut <WasmtimeWasmBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        Box::new(WasmtimeWasiState {})
    }
}

pub struct WasmtimeWasiState {}

impl WasiState for WasmtimeWasiState {
    fn envs(&self) -> &[Vec<u8>] {
        &[]
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
