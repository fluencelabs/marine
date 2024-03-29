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

use wasmtime_wasi::ambient_authority;
use wasmtime_wasi::WasiCtxBuilder;
use anyhow::anyhow;

use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;

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
            mapped_dirs,
        } = parameters;

        let wasi_ctx_builder = WasiCtxBuilder::new();
        // process and add CLI arguments to wasi context
        let wasi_ctx_builder = populate_args(wasi_ctx_builder, args)?;
        // process and add environment variables to wasi context
        let wasi_ctx_builder = populate_envs(wasi_ctx_builder, envs)?;
        // add mapped directories to wasi context, do not create dirs
        let wasi_ctx_builder = populate_mapped_dirs(wasi_ctx_builder, mapped_dirs)?;
        // give access to runner's stdout and stderr, but not stdin
        let mut wasi_ctx_builder = populate_stdio(wasi_ctx_builder);

        let wasi_ctx = wasi_ctx_builder.build();
        add_wasi_to_linker(store, linker, wasi_ctx)
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

fn add_wasi_to_linker(
    store: &mut WasmtimeContextMut<'_>,
    linker: &mut WasmtimeImports,
    wasi_ctx: wasmtime_wasi::WasiCtx,
) -> Result<(), WasiError> {
    // wasmtime-wasi gets its context from ImportCallContext<T>, which can hold any user info
    // the only convenient method is to be provided with a closure that extracts context
    // from used-defined type.
    // So, here each module has its own wasi context which is stored in a vector in store.
    let id = store.inner.data().wasi.len();
    wasmtime_wasi::add_to_linker(&mut linker.linker, move |s: &mut StoreState| {
        &mut s.wasi[id]
    })
    .map_err(|e| WasiError::EngineWasiError(anyhow!(e)))?;

    store.inner.data_mut().wasi.push(wasi_ctx);

    Ok(())
}

fn populate_args(
    mut builder: WasiCtxBuilder,
    args: Vec<String>,
) -> Result<WasiCtxBuilder, WasiError> {
    builder
        .args(&args)
        .map_err(|_| WasiError::TooLargeArgsArray)?;

    Ok(builder)
}

fn populate_mapped_dirs(
    builder: WasiCtxBuilder,
    mapped_dirs: HashMap<String, PathBuf>,
) -> Result<WasiCtxBuilder, WasiError> {
    mapped_dirs.iter().try_fold(
        builder,
        |mut builder, (guest_name, host_path)| -> Result<_, WasiError> {
            let host_dir = wasmtime_wasi::Dir::open_ambient_dir(host_path, ambient_authority())?;
            let guest_path = Path::new(&guest_name);
            builder
                .preopened_dir(host_dir, guest_path)
                .map_err(|e| WasiError::EngineWasiError(anyhow!(e)))?;

            Ok(builder)
        },
    )
}

fn populate_envs(
    mut builder: WasiCtxBuilder,
    envs: HashMap<String, String>,
) -> Result<WasiCtxBuilder, WasiError> {
    let envs = envs.into_iter().collect::<Vec<_>>();

    builder
        .envs(&envs)
        .map_err(|_| WasiError::TooLargeEnvsArray)?;

    Ok(builder)
}

fn populate_stdio(mut builder: WasiCtxBuilder) -> WasiCtxBuilder {
    builder.inherit_stdout().inherit_stderr();

    builder
}
