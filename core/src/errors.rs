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

use crate::HostImportError;
use crate::misc::PrepareError;

use marine_it_interfaces::MITInterfacesError;
use marine_it_parser::ITParserError;
use marine_module_interface::it_interface::ITInterfaceError;
use marine_wasm_backend_traits::errors::*;

use thiserror::Error as ThisError;

// TODO: refactor errors
// TODO: add module name to all errors variants

#[derive(Debug, ThisError)]
pub enum MError {
    /// Errors related to failed resolving of records.
    #[error("{0}")]
    RecordResolveError(String), // TODO: use a proper error type

    /// Errors arisen during creation of a WASI context.
    #[error(transparent)]
    WASIPrepareError(#[from] WasiError),

    /// Errors occurred inside marine-module-interface crate.
    #[error(transparent)]
    ModuleInterfaceError(#[from] ITInterfaceError),

    /// Error arisen during execution of Wasm modules (especially, interface types).
    #[error("Execution error: {0}")]
    ITInstructionError(#[from] wasmer_it::errors::InstructionError),

    /// Error that raises on the preparation step.
    #[error(transparent)]
    PrepareError(#[from] PrepareError),

    /// Indicates that there is already a module with such name.
    #[error("module with name '{0}' already loaded into Marine, please specify another name")]
    NonUniqueModuleName(String),

    /// Returns when there is no module with such name.
    #[error("module with name '{0}' doesn't have function with name {1}")]
    NoSuchFunction(String, String),

    /// Returns when there is no module with such name.
    #[error("module with name '{0}' isn't loaded into Marine")]
    NoSuchModule(String),

    /// An error occurred when host functions tries to lift IValues from WValues and lowering back.
    #[error(transparent)]
    HostImportError(#[from] HostImportError),

    /// IT section parse error.
    #[error(transparent)]
    WITParseError(#[from] ITParserError),

    /// Incorrect IT section.
    #[error("{0}")]
    IncorrectWIT(String), // TODO: use a proper error type

    #[error("Wasm backend error: {0}")]
    WasmBackendError(#[from] WasmBackendError),
}

impl From<MITInterfacesError> for MError {
    fn from(err: MITInterfacesError) -> Self {
        MError::IncorrectWIT(format!("{}", err))
    }
}

impl From<CompilationError> for MError {
    fn from(value: CompilationError) -> Self {
        Into::<WasmBackendError>::into(value).into()
    }
}

impl From<ResolveError> for MError {
    fn from(value: ResolveError) -> Self {
        Into::<WasmBackendError>::into(value).into()
    }
}

impl From<ImportError> for MError {
    fn from(value: ImportError) -> Self {
        Into::<WasmBackendError>::into(value).into()
    }
}

impl From<InstantiationError> for MError {
    fn from(value: InstantiationError) -> Self {
        Into::<WasmBackendError>::into(value).into()
    }
}

impl From<RuntimeError> for MError {
    fn from(value: RuntimeError) -> Self {
        Into::<WasmBackendError>::into(value).into()
    }
}
