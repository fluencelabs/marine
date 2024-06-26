/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::manifest::ManifestError;
use crate::sdk_version::SDKVersionError;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ModuleInfoError {
    /// Version section is absent.
    #[error("the module doesn't contain section with '{0}', probably it's compiled with an old sdk version")]
    NoCustomSection(&'static str),

    /// Multiple sections with the same name.
    #[error("the module contains {1} sections with name '{0}' - it's corrupted")]
    MultipleCustomSections(&'static str, usize),

    /// Errors related to corrupted version.
    #[error(transparent)]
    VersionError(#[from] SDKVersionError),

    /// Errors related to corrupted manifest.
    #[error(transparent)]
    ManifestError(#[from] ManifestError),

    /// An error occurred while parsing Wasm file.
    #[error("provided Wasm file is corrupted: {0}")]
    CorruptedWasmFile(anyhow::Error),

    /// Wasm emitting file error.
    #[error("emitting resulted Wasm file failed with: {0}")]
    WasmEmitError(anyhow::Error),
}
