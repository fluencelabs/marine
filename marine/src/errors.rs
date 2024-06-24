/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use marine_core::MError;
use marine_wasm_backend_traits::MemoryAllocationStats;
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

    /// Marine errors.
    #[error("engine error: {0}")]
    EngineError(#[from] MError),

    /// When marine returned an error and there was a rejected allocation,
    /// the most probable cause is OOM. Otherwise this error is the same as EngineError.
    /// This error is on marine-runtime level,
    /// because otherwise it is impossible to check allocation stats after a failed instantiation.
    #[error("Engine error when OOM suspected ({0} failed allocations), original error: {original_error}", .allocation_stats.allocation_rejects)]
    HighProbabilityOOM {
        original_error: MError,
        allocation_stats: MemoryAllocationStats,
    },
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
