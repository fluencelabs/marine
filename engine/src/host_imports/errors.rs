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

use std::error::Error;

#[derive(Debug)]
pub enum HostImportError {
    /// An error occurred when host functions tries to lift IValues from WValues
    /// and the latter has different type.
    MismatchWValues(WType, WValue),

    /// An error occurred when host functions tries to lift IValues from WValues
    /// and the latter is not enough for that.
    MismatchWValuesCount,

    /// An error related to invalid memory access during lifting IValue.
    InvalidMemoryAccess(i32, i32),

    /// An error related to lifting memory from arrays of pointers with odd elements count.
    OddPointersCount(IType),

    /// An error related to not found record in module record types.
    RecordTypeNotFound(u64),
}

impl Error for HostImportError {}

impl std::fmt::Display for HostImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            HostImportError::MismatchWValues(expected_type, found_value) => write!(
                f,
                "Expected {} type, but found {:?} value during interface values lifting from Wasm memory",
                expected_type, found_value
            ),
            HostImportError::MismatchWValuesCount => {
                write!(f, "Not enough WValue arguments are provided from the Wasm side")
            }
            HostImportError::InvalidMemoryAccess(offset, size) => write!(
                f,
                "Invalid memory access while lifting IValues, offset {}, size {}",
                offset, size
            ),
            HostImportError::OddPointersCount(itype) => {
                write!(f, "Arrays of pointers for value type {:?} contains odd count", itype)
            }
            HostImportError::RecordTypeNotFound(record_type_id) => {
                write!(f, "Record with type id {} not found", record_type_id)
            }
        }
    }
}
