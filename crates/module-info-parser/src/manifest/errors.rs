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

#[derive(Debug, ThisError, PartialEq)]
pub enum ManifestError {
    /// Manifest of a Wasm file doesn't have enough bytes to read size of a field from its prefix.
    #[error(
        "{0} can't be read: embedded manifest doesn't contain enough bytes to read field size from prefix"
    )]
    NotEnoughBytesForPrefix(&'static str),

    /// Manifest of a Wasm file doesn't have enough bytes to read a field.
    #[error(
        "{0} can't be read: embedded manifest doesn't contain enough bytes to read field of size {1}"
    )]
    NotEnoughBytesForField(&'static str, usize),

    /// Manifest of a Wasm file doesn't have enough bytes to read field.
    #[error("{0} is an invalid Utf8 string: {1}")]
    FieldNotValidUtf8(&'static str, Utf8Error),

    /// Size inside prefix of a field is too big (it exceeds usize or overflows with prefix size).
    #[error("{0} has too big size: {1}")]
    TooBigFieldSize(&'static str, u64),

    /// Version can't be parsed with semver.
    #[error("embedded to the Wasm file version is corrupted: '{0}'")]
    ModuleVersionCorrupted(#[from] SemVerError),

    /// Manifest contains some trailing characters.
    #[error("embedded manifest is corrupted: there are some trailing characters")]
    ManifestRemainderNotEmpty,

    /// Error occurred while parsing embedded build time.
    #[error("build time can't be parsed: {0}")]
    DateTimeError(#[from] chrono::ParseError),
}
