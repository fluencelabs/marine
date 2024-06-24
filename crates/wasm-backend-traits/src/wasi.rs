/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::WasiError;
use crate::WasmBackend;

use std::path::PathBuf;
use std::collections::HashMap;

/// A type that provides WASI functionality to the given Wasm backend.
pub trait WasiImplementation<WB: WasmBackend> {
    /// Configures WASI state and adds WASI functions to the `imports` object.
    /// # Errors:
    ///     Returns an error if failed to open a open a directory/file.
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
    pub args: Vec<String>,
    pub envs: HashMap<String, String>,
    pub mapped_dirs: HashMap<String, PathBuf>,
}

pub trait WasiState {
    fn envs(&self) -> &[Vec<u8>];
}
