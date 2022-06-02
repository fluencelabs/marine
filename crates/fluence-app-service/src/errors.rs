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

use marine::MarineError;

use std::io::Error as IOError;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppServiceError {
    /// An error related to config parsing.
    InvalidConfig(String),

    /// Various errors related to file i/o.
    IOError(IOError),

    /// Marine errors.
    MarineError(MarineError),

    /// Directory creation failed
    CreateDir { err: IOError, path: PathBuf },

    /// Errors related to malformed config.
    ConfigParseError(String),
}

impl Error for AppServiceError {}

impl std::fmt::Display for AppServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AppServiceError::InvalidConfig(err_msg) => write!(f, "{}", err_msg),
            AppServiceError::IOError(err) => write!(f, "{}", err),
            AppServiceError::MarineError(err) => write!(f, "{}", err),
            AppServiceError::CreateDir { err, path } => {
                write!(f, "Failed to create dir {:?}: {:?}", path, err)
            }
            AppServiceError::ConfigParseError(err_msg) => write!(f, "{}", err_msg),
        }
    }
}

impl From<MarineError> for AppServiceError {
    fn from(err: MarineError) -> Self {
        AppServiceError::MarineError(err)
    }
}

impl From<IOError> for AppServiceError {
    fn from(err: IOError) -> Self {
        AppServiceError::IOError(err)
    }
}

impl From<toml::de::Error> for AppServiceError {
    fn from(err: toml::de::Error) -> Self {
        AppServiceError::InvalidConfig(format!("{}", err))
    }
}

impl From<std::convert::Infallible> for AppServiceError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
