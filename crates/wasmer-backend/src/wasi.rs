use crate::{WasmerBackend, WasmerCaller};

use marine_wasm_backend_traits::*;

use std::path::PathBuf;

pub struct WasmerWasi {}

impl WasiImplementation<WasmerBackend> for WasmerWasi {
    fn register_in_linker(
        store: &mut <WasmerBackend as WasmBackend>::ContextMut<'_>,
        linker: &mut <WasmerBackend as WasmBackend>::Imports,
        version: WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<Vec<u8>>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<<WasmerBackend as WasmBackend>::Imports, String> {
        todo!()
    }

    fn get_wasi_state<'s>(
        instance: &'s mut <WasmerBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        todo!()
    }
}
