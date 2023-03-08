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

use crate::WasiError;
use crate::WasmBackend;

use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::HashSet;

/// A type that provides WASI functionality to the given Wasm backend.
pub trait WasiImplementation<WB: WasmBackend> {
    /// Configures WASI state and adds WASI functions to the `imports` object.
    /// # Errors:
    ///     Returns an error if failed to open a preopen directory/file.
    fn register_in_linker(
        store: &mut <WB as WasmBackend>::ContextMut<'_>,
        linker: &mut <WB as WasmBackend>::Imports,
        config: WasiParameters,
    ) -> Result<(), WasiError>;

    /// Optional API for getting current WASI state.
    /// Returns None if not supported by current backend.
    fn get_wasi_state<'s>(
        instance: &'s mut <WB as WasmBackend>::Instance,
    ) -> Box<dyn WasiState + 's>;
}

#[derive(Default)]
pub struct WasiParameters {
    pub args: Vec<Vec<u8>>,
    pub envs: HashMap<Vec<u8>, Vec<u8>>,
    pub preopened_files: HashSet<PathBuf>,
    pub mapped_dirs: HashMap<String, PathBuf>,
}

pub trait WasiState {
    fn envs(&self) -> &[Vec<u8>];
}
