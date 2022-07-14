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

use marine_core::MError;
use it_json_serde::ITJsonSeDeError;

use thiserror::Error;

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum MarineError {
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
    #[error(r#"arguments from json deserialization error in module "{module_name}", function "{function_name}": {error}"#)]
    JsonArgumentsDeserializationError {
        module_name: String,
        function_name: String,
        error: ITJsonSeDeError,
    },

    /// Returned outputs aren't compatible with a called function signature.
    #[error(r#"output to json serialization error in module "{module_name}", function "{function_name}": {error}"#)]
    JsonOutputSerializationError {
        module_name: String,
        function_name: String,
        error: ITJsonSeDeError,
    },

    /// Errors related to invalid config.
    #[error("parsing config error: {0}")]
    ParseConfigError(#[from] toml::de::Error),

    /// Errors related to invalid config.
    #[error(
        r#""modules_dir" field is not defined, but it is required to load module "{module_name}""#
    )]
    ModulesDirIsRequiredButNotSpecified { module_name: String },

    /// Errors related to invalid config.
    #[error(
        "max_heap_size = '{max_heap_size_wanted}' can't be bigger than {max_heap_size_allowed}'"
    )]
    MaxHeapSizeOverflow {
        max_heap_size_wanted: u64,
        max_heap_size_allowed: u64,
    },

    /// Marine errors.
    #[error("engine error: {0}")]
    EngineError(#[from] MError),
}

impl From<std::convert::Infallible> for MarineError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

#[macro_export]
macro_rules! json_to_marine_err {
    ($json_expr:expr, $module_name:expr, $function_name:expr) => {
        $json_expr.map_err(|e| match e {
            it_json_serde::ITJsonSeDeError::Se(_) => MarineError::JsonOutputSerializationError {
                module_name: $module_name,
                function_name: $function_name,
                error: e,
            },
            it_json_serde::ITJsonSeDeError::De(_) => {
                MarineError::JsonArgumentsDeserializationError {
                    module_name: $module_name,
                    function_name: $function_name,
                    error: e,
                }
            }
        })
    };
}
