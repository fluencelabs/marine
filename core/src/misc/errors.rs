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

use marine_module_info_parser::ModuleInfoError;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum PrepareError {
    #[error("overflow was happened while summation globals size '{globals_pages_count}' and heap size '{max_heap_pages_count}'")]
    MemSizesOverflow {
        globals_pages_count: u32,
        max_heap_pages_count: u32,
    },

    /// Error is encountered while parsing module version.
    #[error(transparent)]
    ModuleVersionParseError(#[from] ModuleInfoError),

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
}
