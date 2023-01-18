use crate::{WasiError, WasmBackend};

use std::path::PathBuf;

pub trait WasiImplementation<WB: WasmBackend> {
    fn register_in_linker(
        store: &mut <WB as WasmBackend>::ContextMut<'_>,
        linker: &mut <WB as WasmBackend>::Imports,
        version: WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<(Vec<u8>, Vec<u8>)>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<(), WasiError>;

    fn get_wasi_state<'s>(
        instance: &'s mut <WB as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's>;
}

pub enum WasiVersion {
    Snapshot0,
    Snapshot1,
    Latest,
}

pub trait WasiState {
    fn envs(&self) -> &[Vec<u8>];
}
