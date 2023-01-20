use crate::{StoreState, WasmtimeContextMut, WasmtimeImports, WasmtimeWasmBackend};

use marine_wasm_backend_traits::*;

use std::path::{Path, PathBuf};

pub struct WasmtimeWasi {}

impl WasiImplementation<WasmtimeWasmBackend> for WasmtimeWasi {
    fn register_in_linker(
        store: &mut WasmtimeContextMut<'_>,
        linker: &mut WasmtimeImports,
        _version: WasiVersion, // todo user version
        args: Vec<Vec<u8>>,
        envs: Vec<(Vec<u8>, Vec<u8>)>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<(), WasiError> {
        let id = store.inner.data().wasi.len();
        wasmtime_wasi::add_to_linker(&mut linker.linker, move |s: &mut StoreState| {
            &mut s.wasi[id]
        })
        .map_err(|e| WasiError::Other(e))?; // todo add detail

        let args = args
            .into_iter()
            .map(|arg| unsafe { String::from_utf8_unchecked(arg) })
            .collect::<Vec<String>>();
        let envs = envs
            .into_iter()
            .map(|(key, value)| {
                unsafe {
                    // todo maybe use strings in signature?
                    (
                        String::from_utf8_unchecked(key),
                        String::from_utf8_unchecked(value),
                    )
                }
            })
            .collect::<Vec<_>>();

        let wasi_ctx_builder = wasmtime_wasi::WasiCtxBuilder::new()
            .inherit_stdio()
            .args(&args).unwrap() // todo handle error
            .envs(&envs).unwrap()// todo handle error
            ;
        let wasi_ctx_builder = preopened_files
            .iter()
            .fold(wasi_ctx_builder, |builder, path| {
                let file = std::fs::File::open(&path).unwrap(); // todo handle error
                let dir = wasmtime_wasi::Dir::from_std_file(file);
                builder.preopened_dir(dir, &path).unwrap() // todo handle errpr
            });
        let wasi_ctx_builder =
            mapped_dirs
                .iter()
                .fold(wasi_ctx_builder, |builder, (guest_name, dir)| {
                    let file = std::fs::File::open(&dir).unwrap(); // todo handle error
                    let dir = wasmtime_wasi::Dir::from_std_file(file);
                    let path = Path::new(&guest_name);
                    builder.preopened_dir(dir, path).unwrap() // todo handle error
                });
        let wasi_ctx = wasi_ctx_builder.build();
        let state = store.inner.data_mut();
        state.wasi.push(wasi_ctx); //todo handle duplicate
        Ok(())
    }

    fn get_wasi_state<'s>(
        _instance: &'s mut <WasmtimeWasmBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        // todo give actual state
        Box::new(WasmtimeWasiState {})
    }
}

pub struct WasmtimeWasiState {}

impl WasiState for WasmtimeWasiState {
    fn envs(&self) -> &[Vec<u8>] {
        &[]
    }
}
