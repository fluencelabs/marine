/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use marine_wasm_backend_traits::WasmBackendError;
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

    // Wasm backend errors
    WasmBackendError(WasmBackendError),

    /// Directory creation failed
    CreateDir {
        err: IOError,
        path: PathBuf,
    },

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
            AppServiceError::WasmBackendError(err) => {
                write!(f, "{}", err)
            }
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
