use crate::{WasmerBackend, WasmerCaller};

use marine_wasm_backend_traits::*;

use std::path::PathBuf;
use anyhow::anyhow;

pub struct WasmerWasi {}

impl WasiImplementation<WasmerBackend> for WasmerWasi {
    fn register_in_linker(
        store: &mut <WasmerBackend as WasmBackend>::ContextMut<'_>,
        linker: &mut <WasmerBackend as WasmBackend>::Imports,
        _version: WasiVersion,
        args: Vec<Vec<u8>>,
        envs: Vec<(Vec<u8>, Vec<u8>)>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> WasiResult<()> {
        let state = wasmer_wasi::WasiStateBuilder::default()
            .args(args.iter())
            .envs(envs.into_iter())
            .preopen_dirs(preopened_files.iter())
            .map_err(|e| anyhow!("{}", e))?
            .map_dirs(mapped_dirs.into_iter())
            .map_err(|e| anyhow!("{}", e))?
            .build()
            .map_err(|e| anyhow!("{}", e))?;
        let wasi_env = wasmer_wasi::WasiEnv::new(state);
        let func_env = wasmer::FunctionEnv::new(&mut store.inner, wasi_env);
        let wasi_imports = wasmer_wasi::generate_import_object_from_env(
            &mut store.inner,
            &func_env,
            wasmer_wasi::WasiVersion::Latest,
        ); //todo check if latest is right

        linker.inner.extend(wasi_imports.into_iter());
        Ok(())
    }

    fn get_wasi_state<'s>(
        instance: &'s mut <WasmerBackend as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's> {
        todo!()
    }
}
