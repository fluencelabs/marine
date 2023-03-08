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

use crate::StoreState;
use crate::WasmtimeContextMut;
use crate::WasmtimeImports;
use crate::WasmtimeWasmBackend;

use marine_wasm_backend_traits::prelude::*;

use anyhow::anyhow;
use wasmtime_wasi::ambient_authority;

use std::path::Path;

pub struct WasmtimeWasi {}

impl WasiImplementation<WasmtimeWasmBackend> for WasmtimeWasi {
    fn register_in_linker(
        store: &mut WasmtimeContextMut<'_>,
        linker: &mut WasmtimeImports,
        parameters: WasiParameters,
    ) -> Result<(), WasiError> {
        let WasiParameters {
            args,
            envs,
            preopened_files,
            mapped_dirs,
        } = parameters;

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
                    // TODO maybe use strings in signature?
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
                let dir = create_or_open_dir(path)?;
                builder
                    .preopened_dir(dir, path)
                    .map_err(|e| WasiError::EngineWasiError(anyhow!(e)))
            },
        )?;
        let wasi_ctx_builder = mapped_dirs.iter().try_fold(
            wasi_ctx_builder,
            |builder, (guest_name, dir)| -> Result<_, WasiError> {
                let dir = create_or_open_dir(dir)?;
                //let dir = wasmtime_wasi::Dir::from_std_file(file);
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
        // TODO give actual state
        Box::new(WasmtimeWasiState {})
    }
}

pub struct WasmtimeWasiState {}

impl WasiState for WasmtimeWasiState {
    fn envs(&self) -> &[Vec<u8>] {
        &[]
    }
}

fn create_or_open_dir(path: impl AsRef<Path>) -> std::io::Result<wasmtime_wasi::Dir> {
    let path = path.as_ref();
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    wasmtime_wasi::Dir::open_ambient_dir(path, ambient_authority())
}
