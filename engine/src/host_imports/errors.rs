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

use crate::IType;
use super::WType;
use super::WValue;

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

    /// An error related to invalid memory access during lifting IValue.
    #[error("Invalid memory access while lifting IValues, offset {0}, size {1}")]
    InvalidMemoryAccess(i32, i32),

    /// An error related to lifting memory from arrays of pointers with odd elements count.
    #[error("Arrays of pointers for value type {0:?} contains non-even count of elements")]
    OddPointersCount(IType),

    /// An error related to not found record in module record types.
    #[error("Record with type id {0} not found")]
    RecordTypeNotFound(u64),

    /// An error encountered while transmiting arrays.
    #[error("array of bytes with len {0} can't be transmuted to {1} type")]
    TransmuteArrayError(usize, &'static str),
}
