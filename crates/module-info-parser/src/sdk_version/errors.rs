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
use std::str::Utf8Error;

#[derive(Debug, ThisError)]
pub enum SDKVersionError {
    /// Version can't be parsed to Utf8 string.
    #[error("embedded to the Wasm file version isn't valid UTF8 string: '{0}'")]
    VersionNotValidUtf8(Utf8Error),

    /// Version can't be parsed with semver.
    #[error("embedded to the Wasm file version is corrupted: '{0}'")]
    VersionCorrupted(#[from] SemVerError),
}
