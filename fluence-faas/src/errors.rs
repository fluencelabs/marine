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

use marine::MError;

use thiserror::Error;
use std::io::Error as IOError;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum FaaSError {
    /// Errors that happened due to invalid config content
    #[error("InvalidConfig: {0}")]
    InvalidConfig(String),

    /// An error occurred at the instantiation step.
    #[error(
        "module with name {module_import_name} is specified in config (dir: {modules_dir:?}), \
         but not found in provided modules: {provided_modules:?}"
    )]
    InstantiationError {
        module_import_name: String,
        modules_dir: Option<PathBuf>,
        provided_modules: Vec<String>,
    },

    /// Various errors related to file i/o.
    #[error("IOError: {0}")]
    IOError(String),

    /// A function with specified name is missing.
    #[error("function with name `{0}` is missing")]
    MissingFunctionError(String),

    /// An argument with specified name is missing.
    #[error(r#"argument with name "{0}" is missing"#)]
    MissingArgumentError(String),

    /// Returns when there is no module with such name.
    #[error(r#"module with name "{0}" is missing"#)]
    NoSuchModule(String),

    /// Provided arguments aren't compatible with a called function signature.
    #[error("arguments from json deserialization error: {0}")]
    JsonArgumentsDeserializationError(String),

    /// Returned outputs aren't compatible with a called function signature.
    #[error("output to json serialization error: {0}")]
    JsonOutputSerializationError(String),

    /// Errors related to invalid config.
    #[error("parsing config error: {0}")]
    ParseConfigError(#[from] toml::de::Error),

    /// Marine errors.
    #[error("engine error: {0}")]
    EngineError(#[from] MError),
}

impl From<IOError> for FaaSError {
    fn from(err: IOError) -> Self {
        FaaSError::IOError(format!("{}", err))
    }
}

impl From<std::convert::Infallible> for FaaSError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
