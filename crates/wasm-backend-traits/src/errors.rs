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

use thiserror::Error;
use crate::WType;

/*
   General error design goals:
       * expose as much detail as possible
       * make as much domain-specific errors as possible implmentation-independent

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
    CompilationError(#[from] CompilationError),

    #[error(transparent)]
    ImportError(#[from] ImportError),

    #[error(transparent)]
    InstantiationError(#[from] InstantiationError),
}

pub type WasmBackendResult<T> = Result<T, WasmBackendError>;

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

pub type ResolveResult<T> = Result<T, ResolveError>;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Unsupported type encountered: {0}")]
    UnsupportedType(WType),

    #[error(transparent)]
    Trap(anyhow::Error),

    #[error(transparent)]
    UserError(#[from] UserError),

    #[error("A function returned invalid number of results: expected {expected}, got {actual}")]
    IncorrectResultsNumber { expected: usize, actual: usize },

    #[error(transparent)]
    Other(anyhow::Error),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error(transparent)]
    FailedToCompileWasm(anyhow::Error),

    #[error("{0}")]
    FailedToExtractCustomSections(String), // TODO: use a proper error type

    #[error(transparent)]
    Other(anyhow::Error),
}

pub type CompilationResult<T> = Result<T, CompilationError>;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Duplicate import")]
    DuplicateImport(String, String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ImportResult<T> = Result<T, ImportError>;

#[derive(Debug, Error)]
pub enum InstantiationError {
    #[error(transparent)]
    RuntimeError(RuntimeError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type InstantiationResult<T> = Result<T, InstantiationError>;

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

pub type WasiResult<T> = Result<T, WasiError>;
