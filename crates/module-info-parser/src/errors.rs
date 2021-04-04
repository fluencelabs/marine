/*
 * Copyright 2020 Fluence Labs Limited
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
    #[error("{0}")]
    VersionError(#[from] SDKVersionError),

    /// Errors related to corrupted manifest.
    #[error("{0}")]
    ManifestError(#[from] ManifestError),

    /// An error occurred while parsing Wasm file.
    #[error("provided Wasm file is corrupted: {0}")]
    CorruptedWasmFile(anyhow::Error),

    /// Wasm emitting file error.
    #[error("emitting resulted Wasm file failed with: {0}")]
    WasmEmitError(anyhow::Error),
}
