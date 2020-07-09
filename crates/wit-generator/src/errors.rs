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

use std::error::Error;
use serde_json::Error as SerdeDeserializationError;

#[derive(Debug)]
pub enum WITGeneratorError {
    /// An error related to serde deserialization.
    DeserializationError(SerdeDeserializationError),

    /// Various errors occurred during the parsing/emitting a Wasm file.
    IOError(String),
}

impl Error for WITGeneratorError {}

impl std::fmt::Display for WITGeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WITGeneratorError::DeserializationError(err) => write!(
                f,
                "Embedded by rust-sdk metadata could't be parsed by serde: {:?}",
                err
            ),
            WITGeneratorError::IOError(err) => write!(f, "I/O error occurred: {:?}", err),
        }
    }
}

impl From<SerdeDeserializationError> for WITGeneratorError {
    fn from(err: SerdeDeserializationError) -> Self {
        WITGeneratorError::DeserializationError(err)
    }
}
