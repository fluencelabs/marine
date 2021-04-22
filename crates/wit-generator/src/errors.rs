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

use thiserror::Error as ThisError;
use serde_json::Error as SerdeDeserializationError;

#[derive(Debug, ThisError)]
pub enum WITGeneratorError {
    /// An error related to serde deserialization.
    #[error("Embedded by rust-sdk metadata couldn't be parsed by serde: {0:?}")]
    DeserializationError(#[from] SerdeDeserializationError),

    /// Various errors related to records
    #[error("{0}")]
    CorruptedRecord(String),

    /// Various errors occurred during the parsing/emitting a Wasm file.
    #[error("I/O error occurred: {0}")]
    IOError(String),
}
