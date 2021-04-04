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

#[derive(Debug, ThisError)]
pub enum SDKVersionError {
    /// Version can't be parsed to Utf8 string.
    #[error("embedded to the Wasm file version isn't valid UTF8 string: '{0}'")]
    VersionNotValidUtf8(Utf8Error),

    /// Version can't be parsed with semver.
    #[error("embedded to the Wasm file version is corrupted: '{0}'")]
    VersionCorrupted(#[from] SemVerError),
}
