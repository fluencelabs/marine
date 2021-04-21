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

use super::WType;
use super::WValue;

use it_lilo_utils::error::MemoryAccessError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum HostImportError {
    /// An error occurred when host functions tries to lift IValues from WValues
    /// and the latter has different type.
    #[error(
        "Expected {0} type, but found {1:?} value during interface values lifting from Wasm memory"
    )]
    MismatchWValues(WType, WValue),

    /// An error occurred when a host functions tries to lift IValues from WValues
    /// and the latter is not enough for that.
    #[error("Not enough WValue arguments are provided from the Wasm side")]
    MismatchWValuesCount,

    /// Out-of-bound memory access.
    #[error("{0}")]
    MemoryAccessError(#[from] MemoryAccessError),

    /// An error related to not found record in module record types.
    #[error("Record with type id {0} not found")]
    RecordTypeNotFound(u64),

    /// Errors related to lifting incorrect UTF8 string from a Wasm module.
    #[error("corrupted UTF8 string {0}")]
    CorruptedUTF8String(#[from] std::string::FromUtf8Error),

    /// This error occurred when a record is created from empty values array.
    #[error("Record with name '{0}' can't be empty")]
    EmptyRecord(String),
}
