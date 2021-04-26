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

use it_lilo::lifter::LiError;
use it_lilo::lowerer::LoError;
use it_lilo::traits::RecordResolvableError;
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

    #[error("{0}")]
    LifterError(#[from] LiError),

    #[error("{0}")]
    LowererError(#[from] LoError),

    #[error("{0}")]
    RecordNotFound(#[from] RecordResolvableError),

    #[error("{0}")]
    InvalidUTF8String(#[from] std::string::FromUtf8Error),
}
