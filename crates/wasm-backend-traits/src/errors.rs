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

use crate::WType;

use thiserror::Error;

pub type WasmBackendResult<T> = Result<T, WasmBackendError>;
pub type ResolveResult<T> = Result<T, ResolveError>;
pub type RuntimeResult<T> = Result<T, RuntimeError>;
pub type ModuleCreationResult<T> = Result<T, ModuleCreationError>;
pub type InstantiationResult<T> = Result<T, InstantiationError>;
pub type WasiResult<T> = Result<T, WasiError>;

/*
   General error design goals:
       * expose as much detail as possible
       * make as much domain-specific errors as possible implementation-independent

   So, Error enums should follow this principle:
       * errors fully expressible without implementation info should have implementation-independent view
       * errors not fully expressible without implementation info should have some common view and a way to get implmententation-specific details
       * "Other" type for all errors not suited for listed options
*/

#[derive(Debug, Error)]
pub enum WasmBackendError {
    #[error(transparent)]
    ResolveError(#[from] ResolveError),

    #[error(transparent)]
    RuntimeError(#[from] RuntimeError),

    #[error(transparent)]
    ModuleCreationError(#[from] ModuleCreationError),

    #[error(transparent)]
    ImportError(#[from] ImportError),

    #[error(transparent)]
    InstantiationError(#[from] InstantiationError),

    #[error(transparent)]
    InitializationError(anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("export not found: {0}")]
    ExportNotFound(String),

    #[error("export type mismatch: expected {expected}, found {actual}")]
    ExportTypeMismatch {
        expected: &'static str,
        actual: &'static str,
    },

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Unsupported type encountered: {0}")]
    UnsupportedType(WType),

    #[error("Trap occurred: {0}")]
    Trap(anyhow::Error),

    #[error(transparent)]
    UserError(#[from] UserError),

    #[error("A function returned invalid number of results: expected {expected}, got {actual}")]
    IncorrectResultsNumber { expected: usize, actual: usize },

    #[error("Unrecognized error: {0}")]
    Other(anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ModuleCreationError {
    #[error(transparent)]
    FailedToCompileWasm(anyhow::Error),

    #[error("{0}")]
    FailedToExtractCustomSections(String), // TODO: use a proper error type

    #[error(transparent)]
    Other(anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Duplicate import")]
    DuplicateImport(String, String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum InstantiationError {
    #[error(transparent)]
    RuntimeError(RuntimeError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum WasiError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    EngineWasiError(#[from] anyhow::Error),

    #[error("Cumulative size of args array exceeds 2^32")]
    TooLargeArgsArray,

    #[error("Cumulative size of envs array exceeds 2^32")]
    TooLargeEnvsArray,
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Recoverable(anyhow::Error),

    #[error(transparent)]
    Unrecoverable(anyhow::Error),
}
