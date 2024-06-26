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

impl From<ModuleCreationError> for MError {
    fn from(value: ModuleCreationError) -> Self {
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
