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

use marine_module_info_parser::ModuleInfoError;
use marine_it_generator::ITGeneratorError;
use marine_it_parser::ITParserError;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum CLIError {
    /// Unknown command was entered by user.
    #[error("{0} is an unknown command")]
    NoSuchCommand(String),

    /// A error occurred while embedding rust sdk version.
    #[error("{0}")]
    VersionEmbeddingError(#[from] ModuleInfoError),

    /// An error occurred while generating interface types.
    #[error("{0}")]
    ITGeneratorError(#[from] ITGeneratorError),

    /// An error occurred while parsing interface types.
    #[error("{0}")]
    ITParserError(#[from] ITParserError),

    /// An error occurred when no Wasm file was compiled.
    #[error("{0}")]
    WasmCompilationError(String),

    /// Various errors related to I/O operations.
    #[error("{0:?}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    ManifestParseError(String),

    #[error("{0:?}")]
    VersionParseError(#[from] semver::SemVerError),
}
