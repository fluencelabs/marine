mod caller;
mod store;
mod utils;
mod module;
mod instance;
mod wasi;

use store::*;
use caller::*;
use module::*;
use instance::*;
use wasi::*;

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
    type Store = WasmtimeStore;
    type Context<'c> = WasmtimeContext<'c>;
    type ContextMut<'c> = WasmtimeContextMut<'c>;
    type Module = WasmtimeModule;
    type Instance = WasmtimeInstance;
    type Imports = WasmtimeImportObject;
    type Caller<'c> = WasmtimeCaller<'c>;
    type DynamicFunc = WasmtimeDynamicFunc;
    type MemoryExport = WasmtimeMemoryExport;
    type FunctionExport = WasmtimeFunctionExport;
    type Namespace = WasmtimeNamespace;
    type Function = WasmtimeExportedDynFunc;
    type WITMemory = WasmtimeWITMemory;
    type WITMemoryView = WasmtimeWITMemoryView;
    type Wasi = WasmtimeWasi;

    fn compile(store: &mut WasmtimeStore, wasm: &[u8]) -> WasmBackendResult<Self::Module> {
        let module = wasmtime::Module::new(store.inner.engine(), wasm).unwrap(); //todo convert error;
        let custom_sections = WasmtimeWasmBackend::custom_sections(wasm).unwrap(); //todo convert error;
        Ok(WasmtimeModule {
            custom_sections,
            inner: module,
        })
    }
}


#[derive(Default)]
pub struct StoreState {
    wasi: Vec<WasiCtx>, //todo switch to Pool or something
}

// imports
#[derive(Clone)]
pub struct WasmtimeImportObject {
    linker: wasmtime::Linker<StoreState>,
}

impl Imports<WasmtimeWasmBackend> for WasmtimeImportObject {
    fn new(store: &mut WasmtimeStore) -> Self {
        Self {
            linker: wasmtime::Linker::new(store.inner.engine()),
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
                Fn(&mut WasmtimeCaller<'_>, ($($arg,)*)) -> $rets
                + Sync
                + Send
                + 'static {
                let name: String = name.into();
                println!("calling insert_fn with {}", &name);
                let inserter = move |linker: &mut WasmtimeImportObject, module: &str, name: &str| {
                    let func = move |caller: wasmtime::Caller<'_, StoreState>, $($name: $arg),*| {
                            let mut ctx = WasmtimeCaller {inner: caller};

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
                let mut ctx = WasmtimeCaller {
                    inner: caller
                };
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
                            &mut WasmtimeContextMut<'_>,
                            $args,
                        ) -> Result<$rets, RuntimeError>
                        + Sync
                        + Send
                        + 'static,
                >,
                ResolveError,
            > {
                let export = self.inner.get_export(name).unwrap(); //todo handle error
                match export {
                    Extern::Func(f) => {
                        let f = f.typed(&mut self.inner).unwrap(); //todo handle error
                        let closure = move |store: &mut WasmtimeContextMut<'_>, args| {

                            let rets = f.call(&mut store.inner, args).unwrap(); //todo handle error
                            return Ok(rets)
                        };


                        Ok(Box::new(closure))
                    }
                    Extern::Memory(m) => {
                        panic!("caller.get_export returned memory");
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
        Box<dyn FnMut(&mut WasmtimeContextMut<'_>, ()) -> Result<(), RuntimeError> + 's>,
        ResolveError,
    > {
        let export = self.inner.get_export(name).unwrap(); //todo handle error
        match export {
            Extern::Func(f) => {
                let f = f.typed(&mut self.inner).unwrap(); //todo handle error
                let closure = move |store, $args| {

                    f.call(store.inner, $args) //todo handle error
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


pub struct WasmtimeExportedDynFunc {
    func: wasmtime::Func,
    signature: FuncSig,
}

impl ExportedDynFunc<WasmtimeWasmBackend> for WasmtimeExportedDynFunc {
    fn signature<'c>(&self, store: WasmtimeContextMut) -> &FuncSig {
        &self.signature
    }

    fn call<'c>(&self, store: WasmtimeContextMut, args: &[WValue]) -> CallResult<Vec<WValue>> {
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

    fn size(&self, store: &mut WasmtimeContextMut<'_>) -> usize {
        self.memory.size(store) as usize
    }
}

pub struct WasmtimeWITMemoryView {
    view: wasmtime::Memory
}

impl it_memory_traits::MemoryReadable<DelayedContextLifetime<WasmtimeWasmBackend>> for WasmtimeWITMemoryView {
    fn read_byte(&self, store: &mut WasmtimeContextMut<'_>, offset: u32) -> u8 {
        let mut value = [0u8];
        self.view.read(&mut store.inner, offset as usize, &mut value).unwrap(); // todo handle error;
        value[0]
    }

    fn read_array<const COUNT: usize>(&self, store: &mut WasmtimeContextMut<'_>, offset: u32) -> [u8; COUNT] {
        let mut value = [0u8; COUNT];
        self.view.read(&mut store.inner, offset as usize, &mut value).unwrap(); // todo handle error;
        value
    }

    fn read_vec(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, size: u32) -> Vec<u8> {
        let mut value = vec![0u8; size as usize];
        self.view.read(&mut store.inner, offset as usize, &mut value).unwrap(); // todo handle error;
        value
    }
}

impl it_memory_traits::MemoryWritable<DelayedContextLifetime<WasmtimeWasmBackend>> for WasmtimeWITMemoryView {
    fn write_byte(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, value: u8) {
        let buffer = [value];
        self.view.write(&mut store.inner, offset as usize, &buffer).unwrap() // todo handle error
    }

    fn write_bytes(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, bytes: &[u8]) {
        self.view.write(&mut store.inner, offset as usize, bytes).unwrap() // todo handle error
    }
}

impl it_memory_traits::MemoryView<DelayedContextLifetime<WasmtimeWasmBackend>> for WasmtimeWITMemoryView {
    fn check_bounds(&self, store: &mut WasmtimeContextMut<'_>, offset: u32, size: u32) -> Result<(), MemoryAccessError> {
        let memory_size = self.view.size(&mut store.inner);
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
