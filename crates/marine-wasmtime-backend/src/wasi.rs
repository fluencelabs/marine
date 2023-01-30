/*
 * Copyright 2023 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{StoreState, WasmtimeContextMut, WasmtimeImports, WasmtimeWasmBackend};

use marine_wasm_backend_traits::*;

use anyhow::anyhow;

use std::path::{Path, PathBuf};

pub struct WasmtimeWasi {}

impl WasiImplementation<WasmtimeWasmBackend> for WasmtimeWasi {
    fn register_in_linker(
        store: &mut WasmtimeContextMut<'_>,
        linker: &mut WasmtimeImports,
        _version: WasiVersion, // wasmtime does not have version in API, looks like it adds everything to the linker
        args: Vec<Vec<u8>>,
        envs: Vec<(Vec<u8>, Vec<u8>)>,
        preopened_files: Vec<PathBuf>,
        mapped_dirs: Vec<(String, PathBuf)>,
    ) -> Result<(), WasiError> {
        let id = store.inner.data().wasi.len();
        wasmtime_wasi::add_to_linker(&mut linker.linker, move |s: &mut StoreState| {
            &mut s.wasi[id]
        })
        .map_err(|e| WasiError::EngineWasiError(anyhow!(e)))?;

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
            .args(&args)
            .map_err(|_| WasiError::TooLargeArgsArray)?
            .envs(&envs)
            .map_err(|_| WasiError::TooLargeEnvsArray)?;

        let wasi_ctx_builder = preopened_files.iter().try_fold(
            wasi_ctx_builder,
            |builder, path| -> Result<_, WasiError> {
                let file = std::fs::File::open(path)?;
                let dir = wasmtime_wasi::Dir::from_std_file(file);
                builder
                    .preopened_dir(dir, path)
                    .map_err(|e| WasiError::EngineWasiError(anyhow!(e)))
            },
        )?;
        let wasi_ctx_builder = mapped_dirs.iter().try_fold(
            wasi_ctx_builder,
            |builder, (guest_name, dir)| -> Result<_, WasiError> {
                let file = std::fs::File::open(dir)?;
                let dir = wasmtime_wasi::Dir::from_std_file(file);
                let path = Path::new(&guest_name);
                builder
                    .preopened_dir(dir, path)
                    .map_err(|e| WasiError::EngineWasiError(anyhow!(e)))
            },
        )?;

        let wasi_ctx = wasi_ctx_builder.build();
        let state = store.inner.data_mut();
        state.wasi.push(wasi_ctx);
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
