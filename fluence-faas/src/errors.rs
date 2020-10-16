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

use serde_json::error::Error as SerdeError;

use std::io::Error as IOError;
use std::error::Error;

#[derive(Debug)]
pub enum FaaSError {
    /// An error related to config parsing.
    ConfigParseError(String),

    /// An error occurred at the instantiation step.
    InstantiationError(String),

    /// Various errors related to file i/o.
    IOError(String),

    /// A function with specified name is missing.
    MissingFunctionError(String),

    /// An argument with specified name is missing.
    MissingArgumentError(String),

    /// Not enough arguments provided for FCE call.
    JsonArgumentsDeserializationError(String),

    /// An error occurred when incorrect json argument is supplied.
    ArgumentDeserializationError(SerdeError),

    /// FCE errors.
    EngineError(FCEError),

    ParseConfigError(toml::de::Error),
}

impl Error for FaaSError {}

impl std::fmt::Display for FaaSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FaaSError::ConfigParseError(err_msg) => write!(f, "{}", err_msg),
            FaaSError::InstantiationError(err_msg) => write!(f, "{}", err_msg),
            FaaSError::MissingFunctionError(func_name) => {
                write!(f, "function with name `{}` is missing", func_name)
            }
            FaaSError::MissingArgumentError(arg_name) => {
                write!(f, "argument with name `{}` is missing", arg_name)
            }
            FaaSError::JsonArgumentsDeserializationError(args) => write!(f, "{}", args),
            FaaSError::ArgumentDeserializationError(err_msg) => write!(f, "{:?}", err_msg),
            FaaSError::IOError(err_msg) => write!(f, "{}", err_msg),
            FaaSError::EngineError(err) => write!(f, "{}", err),
            FaaSError::ParseConfigError(err) => write!(f, "{}", err),
        }
    }
}

impl From<IOError> for FaaSError {
    fn from(err: IOError) -> Self {
        FaaSError::IOError(format!("{}", err))
    }
}

impl From<FCEError> for FaaSError {
    fn from(err: FCEError) -> Self {
        FaaSError::EngineError(err)
    }
}

impl From<toml::de::Error> for FaaSError {
    fn from(err: toml::de::Error) -> Self {
        FaaSError::ConfigParseError(format!("{}", err))
    }
}

impl From<std::convert::Infallible> for FaaSError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
