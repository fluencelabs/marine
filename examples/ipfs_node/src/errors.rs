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

use fce::FCEError;

use std::io::{Error as IOError, IoSliceMut};
use std::error::Error;

#[derive(Debug)]
pub enum NodeError {
    /// Various errors related to file io.
    IOError(IOError),

    /// WIT doesn't contain such type.
    WasmProcessError(FCEError),
}

impl Error for NodeError {}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            NodeError::IOError(err) => write!(f, "{}", err),
            NodeError::WasmProcessError(err) => write!(f, "{}", err),
        }
    }
}

impl From<IOError> for NodeError {
    fn from(err: IOError) -> Self {
        NodeError::IOError(err)
    }
}

impl From<FCEError> for NodeError {
    fn from(err: FCEError) -> Self {
        NodeError::WasmProcessError(err)
    }
}
