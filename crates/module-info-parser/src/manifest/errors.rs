/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use semver::Error as SemVerError;
use thiserror::Error as ThisError;
use derivative::Derivative;

use std::str::Utf8Error;

#[derive(Debug, ThisError, Derivative)]
#[derivative(PartialEq)]
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
    ModuleVersionCorrupted(
        #[derivative(PartialEq(compare_with = "cmp_semver_error"))]
        #[from]
        SemVerError,
    ),

    /// Manifest contains some trailing characters.
    #[error("embedded manifest is corrupted: there are some trailing characters")]
    ManifestRemainderNotEmpty,

    /// Error occurred while parsing embedded build time.
    #[error("build time can't be parsed: {0}")]
    DateTimeParseError(#[from] chrono::ParseError),
}

fn cmp_semver_error(lhs: &SemVerError, rhs: &SemVerError) -> bool {
    // semver::Error does not provide anything better
    lhs.to_string() == rhs.to_string()
}
