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

use marine_it_interfaces::MITInterfacesError;
use marine_module_interface::it_interface::ITInterfaceError;

use thiserror::Error as ThisError;

// TODO: refactor errors

#[derive(Debug, ThisError)]
pub enum MError {
    /// Errors related to failed resolving of records.
    #[error("{0}")]
    RecordResolveError(String),

    /// Errors occurred inside marine-module-interface crate.
    #[error(transparent)]
    ModuleInterfaceError(#[from] ITInterfaceError),

    /// Error arisen during execution of Wasm modules (especially, interface types).
    #[error("Execution error: {0}")]
    ITInstructionError(#[from] wasmer_it::errors::InstructionError),

    /// Indicates that there is already a module with such name.
    #[error("module with name '{0}' already loaded into Marine, please specify another name")]
    NonUniqueModuleName(String),

    /// Returns when there is no module with such name.
    #[error("module with name '{0}' doesn't have function with name {1}")]
    NoSuchFunction(String, String),

    /// Returns when there is no module with such name.
    #[error("module with name '{0}' isn't loaded into Marine")]
    NoSuchModule(String),

    /// Incorrect IT section.
    #[error("{0}")]
    IncorrectWIT(String),

    /// Provided module doesn't contain a sdk version that is necessary.
    #[error("module with name '{0}' doesn't contain a version of sdk, probably it's compiled with an old one")]
    ModuleWithoutVersion(String),

    /// Module sdk versions are incompatible.
    #[error("module with name '{module_name}' compiled with {provided} sdk version, but at least {required} required")]
    IncompatibleSDKVersions {
        module_name: String,
        required: semver::Version,
        provided: semver::Version,
    },

    /// Module IT versions are incompatible.
    #[error("module with name '{module_name}' compiled with {provided} IT version, but at least {required} required")]
    IncompatibleITVersions {
        module_name: String,
        required: semver::Version,
        provided: semver::Version,
    },

    #[error("some error expressed as string: {0}")]
    StringError(String),
}

impl From<MITInterfacesError> for MError {
    fn from(err: MITInterfacesError) -> Self {
        MError::IncorrectWIT(format!("{}", err))
    }
}

impl From<String> for MError {
    fn from(err: String) -> Self {
        MError::StringError(format!("{}", err))
    }
}

impl From<()> for MError {
    fn from(_err: ()) -> Self {
        MError::IncorrectWIT("failed to parse instructions for adapter type".to_string())
    }
}
