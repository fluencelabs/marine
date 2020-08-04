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

use fluence_faas::FaaSError;

use std::io::Error as IOError;
use std::error::Error;

#[derive(Debug)]
pub enum ServiceError {
    /// An error related to config parsing.
    InvalidArguments(String),

    /// Various errors related to file i/o.
    IOError(String),

    /// FaaS errors.
    FaaSError(FaaSError),
}

impl Error for ServiceError {}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ServiceError::InvalidArguments(err_msg) => write!(f, "{}", err_msg),
            ServiceError::IOError(err_msg) => write!(f, "{}", err_msg),
            ServiceError::FaaSError(err) => write!(f, "{}", err),
        }
    }
}

impl From<IOError> for ServiceError {
    fn from(err: IOError) -> Self {
        ServiceError::IOError(format!("{}", err))
    }
}

impl From<FaaSError> for ServiceError {
    fn from(err: FaaSError) -> Self {
        ServiceError::FaaSError(err)
    }
}

impl From<toml::de::Error> for ServiceError {
    fn from(err: toml::de::Error) -> Self {
        ServiceError::InvalidArguments(format!("{}", err))
    }
}

impl From<std::convert::Infallible> for ServiceError {
    fn from(inf: std::convert::Infallible) -> Self {
        match inf {}
    }
}
