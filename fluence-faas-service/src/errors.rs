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
pub enum AppServiceError {
    /// An error related to config parsing.
    InvalidArguments(String),

    /// Various errors related to file i/o.
    IOError(String),

    /// FaaS errors.
    FaaSError(FaaSError),
}

impl Error for AppServiceError {}

impl std::fmt::Display for AppServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AppServiceError::InvalidArguments(err_msg) => write!(f, "{}", err_msg),
            AppServiceError::IOError(err_msg) => write!(f, "{}", err_msg),
            AppServiceError::FaaSError(err) => write!(f, "{}", err),
        }
    }
}

impl From<IOError> for AppServiceError {
    fn from(err: IOError) -> Self {
        AppServiceError::IOError(format!("{}", err))
    }
}

impl From<FaaSError> for AppServiceError {
    fn from(err: FaaSError) -> Self {
        AppServiceError::FaaSError(err)
    }
}

impl From<toml::de::Error> for AppServiceError {
    fn from(err: toml::de::Error) -> Self {
        AppServiceError::InvalidArguments(format!("{}", err))
    }
}

impl From<std::convert::Infallible> for AppServiceError {
    fn from(inf: std::convert::Infallible) -> Self {
        match inf {}
    }
}
