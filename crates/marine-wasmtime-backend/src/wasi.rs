use crate::{StoreState, WasmtimeContextMut, WasmtimeImports, WasmtimeWasmBackend};

use marine_wasm_backend_traits::*;

use std::path::PathBuf;

pub struct WasmtimeWasi {}

impl WasiImplementation<WasmtimeWasmBackend> for WasmtimeWasi {
    fn register_in_linker(
        store: &mut WasmtimeContextMut<'_>,
        linker: &mut WasmtimeImports,
        version: WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<(Vec<u8>, Vec<u8>)>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<(), String> {
        let id = store.inner.data().wasi.len();
        wasmtime_wasi::add_to_linker(&mut linker.linker, move |s: &mut StoreState| &mut s.wasi[id])
            .unwrap(); // todo handle error
                       // Create a WASI context and put it in a Store; all instances in the storex
                       // share this context. `WasiCtxBuilder` provides a number of ways to
                       // configure what the target program will have access to.
        let args = args
            .into_iter()
            .map(|arg| unsafe { String::from_utf8_unchecked(arg) })
            .collect::<Vec<String>>();
        // todo pass all data to ctx
        let wasi_ctx = wasmtime_wasi::WasiCtxBuilder::new()
            .inherit_stdio()
            .args(&args)
            .unwrap() // todo handle error
            .build();
        let state = store.inner.data_mut();
        state.wasi.push(wasi_ctx); //todo handle duplicate
        Ok(())
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
