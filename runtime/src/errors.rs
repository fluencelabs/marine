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
use marine_wasm_backend_traits::WasmBackendError;
use wasmer_runtime::error as wasmer_error;

use thiserror::Error as ThisError;

// TODO: refactor errors
// TODO: add module name to all errors variants

#[derive(Debug, ThisError)]
pub enum MError {
    /// This error type is produced by Wasmer during resolving a Wasm function.
    #[error("Wasmer resolve error: {0}")]
    ResolveError(#[from] wasmer_error::ResolveError),

    /// Error related to calling a main Wasm module.
    #[error("Wasmer invoke error: {0}")]
    WasmerInvokeError(String),

    /// Error that raises during compilation Wasm code by Wasmer.
    #[error("Wasmer creation error: {0}")]
    WasmerCreationError(#[from] wasmer_error::CreationError),

    /// Error that raises during creation of some Wasm objects (like table and memory) by Wasmer.
    #[error("Wasmer compile error: {0}")]
    WasmerCompileError(#[from] wasmer_error::CompileError),

    /// Errors arisen during execution of a Wasm module.
    #[error("Wasmer runtime error: {0}")]
    WasmerRuntimeError(String),

    /// Errors arisen during linking Wasm modules with already loaded into Marine modules.
    #[error("Wasmer link error: {0}")]
    WasmerLinkError(#[from] wasmer_error::LinkError),

    /// Errors from the temporary class of amalgamation errors from the Wasmer side.
    #[error("Wasmer error: {0}")]
    WasmerError(String),

    /// Errors related to failed resolving of records.
    #[error("{0}")]
    RecordResolveError(String),

    /// Errors arisen during creation of a WASI context.
    #[error("{0}")]
    WASIPrepareError(String),

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
    IncorrectWIT(String),

    #[error("WASM BACKEND ERROR: {0}")]
    WasmBackendError(WasmBackendError),
}

impl From<MITInterfacesError> for MError {
    fn from(err: MITInterfacesError) -> Self {
        MError::IncorrectWIT(format!("{}", err))
    }
}

impl From<wasmer_error::RuntimeError> for MError {
    fn from(err: wasmer_error::RuntimeError) -> Self {
        Self::WasmerRuntimeError(err.to_string())
    }
}

impl From<wasmer_error::Error> for MError {
    fn from(err: wasmer_error::Error) -> Self {
        Self::WasmerError(err.to_string())
    }
}

impl From<wasmer_error::InvokeError> for MError {
    fn from(err: wasmer_error::InvokeError) -> Self {
        Self::WasmerInvokeError(err.to_string())
    }
}

impl From<()> for MError {
    fn from(_err: ()) -> Self {
        MError::IncorrectWIT("failed to parse instructions for adapter type".to_string())
    }
}

impl From<WasmBackendError> for MError {
    fn from(err: WasmBackendError) -> Self {
        MError::WasmBackendError(err)
    }
}
