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

use semver::SemVerError;
use thiserror::Error as ThisError;
use std::str::Utf8Error;
use std::io::Error as IOError;

#[derive(Debug, ThisError)]
pub enum ManifestParserError {
    /// Version section is absent.
    #[error("the module doesn't contain section with '{0}', probably it's compiled with older sdk version")]
    NoCustomSection(&'static str),

    /// Multiple sections with the same name.
    #[error("the module contains {1} sections with name '{0}' - it's corrupted")]
    MultipleCustomSections(&'static str, usize),

    /// Version can't be parsed to Utf8 string.
    #[error("embedded to the Wasm file version isn't valid UTF8 string: '{0}'")]
    VersionNotValidUtf8(Utf8Error),

    /// Version can't be parsed with semver.
    #[error("embedded to the Wasm file version is corrupted: '{0}'")]
    VersionCorrupted(SemVerError),

    /// Manifest of a Wasm file doesn't have enough bytes to read field.
    #[error(
        "{0} can't be read: embedded manifest doesn't contain enough bytes to read field prefix"
    )]
    ManifestCorrupted(&'static str),

    /// An error occurred while parsing Wasm file.
    #[error("provided Wasm file is corrupted: {0}")]
    CorruptedWasmFile(anyhow::Error),

    /// An error occurred while manipulating with converting ast to bytes.
    #[error("Convertation Wast to AST failed with: {0}")]
    AstToBytesError(IOError),
}

impl From<SemVerError> for ManifestParserError {
    fn from(err: SemVerError) -> Self {
        Self::VersionCorrupted(err)
    }
}

impl From<IOError> for ManifestParserError {
    fn from(err: IOError) -> Self {
        Self::AstToBytesError(err)
    }
}
