use std::cell::RefCell;
use std::collections::HashMap;
use anyhow::anyhow;
use multimap::MultiMap;
use marine_wasm_backend_traits::*;

mod module;
mod store;
mod instance;
mod function;
mod imports;
mod utils;
mod memory;
mod caller;
mod wasi;

use module::*;
use store::*;
use instance::*;
use function::*;
use imports::*;
use memory::*;
use caller::*;
use wasi::*;

pub(crate) use utils::*;

#[derive(Clone, Default)]
pub struct WasmerBackend {}

impl WasmBackend for WasmerBackend {
    type Module = WasmerModule;
    type Instance = WasmerInstance;
    type Store = WasmerStore;
    type Context<'c> = WasmerContext<'c>;
    type ContextMut<'c> = WasmerContextMut<'c>;
    type Caller<'c> = WasmerCaller<'c>;
    type Imports = WasmerImports;
    type Function = WasmerFunction;
    type Memory = WasmerMemory;
    type MemoryView = WasmerMemory;
    type Wasi = WasmerWasi;

    fn compile(store: &mut Self::Store, wasm: &[u8]) -> CompilationResult<Self::Module> {
        wasmer::Module::new(store.inner.engine(), wasm)
            .map_err(|e| {
                CompilationError::Other(anyhow!("Wasmer module compilation failed: {}", e))
                // todo make detailed
            })
            .and_then(|module| {
                let custom_sections = Self::custom_sections(wasm).map_err(|e| {
                    CompilationError::Other(anyhow!("{}", e)) // todo make detailed
                })?;
                Ok(WasmerModule {
                    inner: module,
                    custom_sections,
                })
            })
    }
}

impl WasmerBackend {
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
