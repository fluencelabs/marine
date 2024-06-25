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

use crate::cargo_manifest::ManifestError;

use marine_module_info_parser::ModuleInfoError;
use marine_it_generator::ITGeneratorError;
use marine_it_parser::ITParserError;

use thiserror::Error as ThisError;

use std::path::PathBuf;

#[derive(Debug, ThisError)]
pub enum CLIError {
    /// Unknown command was entered by user.
    #[error("{0} is an unknown command")]
    NoSuchCommand(String),

    /// A error occurred while embedding rust sdk version.
    #[error(transparent)]
    VersionEmbeddingError(#[from] ModuleInfoError),

    /// An error occurred while generating interface types.
    #[error(transparent)]
    ITGeneratorError(#[from] ITGeneratorError),

    /// An error occurred while parsing interface types.
    #[error(transparent)]
    ITParserError(#[from] ITParserError),

    /// An error occurred when no Wasm file was compiled.
    #[error("{0}")]
    WasmCompilationError(String),

    /// Various errors related to I/O operations.
    #[error("{0:?}")]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ManifestError(#[from] ManifestError),

    #[error("Error loading lockfile at {0}: {1}")]
    LockfileError(PathBuf, cargo_lock::Error),

    #[error(transparent)]
    MetadataError(#[from] cargo_metadata::Error),
}
