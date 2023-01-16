mod caller;
mod store;
mod utils;
mod module;
mod instance;
mod wasi;
mod function;
mod imports;
mod memory;

use store::*;
use caller::*;
use module::*;
use instance::*;
use wasi::*;
use function::*;
use memory::*;
use imports::*;

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
use wasmtime::{Extern, Func, Linker};
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
    type Module = WasmtimeModule;
    type Instance = WasmtimeInstance;
    type Store = WasmtimeStore;
    type Context<'c> = WasmtimeContext<'c>;
    type ContextMut<'c> = WasmtimeContextMut<'c>;
    type Caller<'c> = WasmtimeCaller<'c>;
    type Imports = WasmtimeImports;
    type Function = WasmtimeFunction;
    type Memory = WasmtimeMemory;
    type MemoryView = WasmtimeMemory;
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

impl MemoryExport for WasmtimeMemory {}

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
                    dyn FnMut(&mut WasmtimeContextMut<'_>, $args) -> Result<$rets, RuntimeError>
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
                            return Ok(rets);
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
