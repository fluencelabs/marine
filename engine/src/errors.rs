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

use crate::HostImportError;
use marine_it_interfaces::MITInterfacesError;
use marine_it_parser::ITParserError;
use marine_module_info_parser::ModuleInfoError;

use wasmer_runtime::error as wasmer_error;

use thiserror::Error as ThisError;

// TODO: refactor errors

#[derive(Debug, ThisError)]
pub enum MError {
    /// This error type is produced by Wasmer during resolving a Wasm function.
    #[error("WasmerResolveError: {0}")]
    ResolveError(#[from] wasmer_error::ResolveError),

    /// Error related to calling a main Wasm module.
    #[error("WasmerInvokeError: {0}")]
    WasmerInvokeError(String),

    /// Error that raises during compilation Wasm code by Wasmer.
    #[error("WasmerCreationError: {0}")]
    WasmerCreationError(#[from] wasmer_error::CreationError),

    /// Error that raises during creation of some Wasm objects (like table and memory) by Wasmer.
    #[error("WasmerCompileError: {0}")]
    WasmerCompileError(#[from] wasmer_error::CompileError),

    /// Errors arisen during execution of a Wasm module.
    #[error("WasmerCompileError: {0}")]
    WasmerRuntimeError(String),

    /// Errors arisen during linking Wasm modules with already loaded into Marine modules.
    #[error("WasmerLinkError: {0}")]
    WasmerLinkError(#[from] wasmer_error::LinkError),

    /// Errors from the temporary class of amalgamation errors from the Wasmer side.
    #[error("WasmerError: {0}")]
    WasmerError(String),

    /// Errors related to failed resolving of records.
    #[error("{0}")]
    RecordResolveError(String),

    /// Errors arisen during creation of a WASI context.
    #[error("{0}")]
    WASIPrepareError(String),

    /// Error arisen during execution of Wasm modules (especially, interface types).
    #[error("Execution error: {0}")]
    ITInstructionError(#[from] wasmer_it::errors::InstructionError),

    /// Error that raises on the preparation step.
    #[error("PrepareError: {0}, probably module is malformed")]
    PrepareError(#[from] parity_wasm::elements::Error),

    /// Indicates that there is already a module with such name.
    #[error("module with name {0} already loaded in Marine, please specify another name")]
    NonUniqueModuleName(String),

    /// Returns when there is no module with such name.
    #[error("module with name {0} doesn't have function with name {1}")]
    NoSuchFunction(String, String),

    /// Returns when there is no module with such name.
    #[error("module with name {0} doesn't loaded in Marine")]
    NoSuchModule(String),

    /// An error occurred when host functions tries to lift IValues from WValues and lowering back.
    #[error("{0}")]
    HostImportError(#[from] HostImportError),

    /// IT section parse error.
    #[error("{0}")]
    WITParseError(#[from] ITParserError),

    /// Incorrect IT section.
    #[error("{0}")]
    IncorrectWIT(String),

    /// Error is encountered while parsing module version.
    #[error("{0}")]
    ModuleVersionParseError(#[from] ModuleInfoError),

    /// Provided module doesn't contain a sdk version that is necessary.
    #[error("module with name {0} doesn't contain a version of sdk, probably it's compiled with an old one")]
    ModuleWithoutVersion(String),

    /// Module sdk versions are incompatible.
    #[error("module with name {module_name} compiled with {provided} sdk version, but at least {required} required")]
    IncompatibleSDKVersions {
        module_name: String,
        required: semver::Version,
        provided: semver::Version,
    },

    /// Module IT versions are incompatible.
    #[error("module with name {module_name} compiled with {provided} IT version, but at least {required} required")]
    IncompatibleITVersions {
        module_name: String,
        required: semver::Version,
        provided: semver::Version,
    },
}

impl From<MITInterfacesError> for MError {
    fn from(err: MITInterfacesError) -> Self {
        MError::IncorrectWIT(format!("{}", err))
    }
}

impl From<wasmer_error::RuntimeError> for MError {
    fn from(err: wasmer_error::RuntimeError) -> Self {
        Self::WasmerRuntimeError(err.to_string())
    }
}

impl From<wasmer_error::Error> for MError {
    fn from(err: wasmer_error::Error) -> Self {
        Self::WasmerError(err.to_string())
    }
}

impl From<wasmer_error::InvokeError> for MError {
    fn from(err: wasmer_error::InvokeError) -> Self {
        Self::WasmerInvokeError(err.to_string())
    }
}

impl From<()> for MError {
    fn from(_err: ()) -> Self {
        MError::IncorrectWIT("failed to parse instructions for adapter type".to_string())
    }
}
