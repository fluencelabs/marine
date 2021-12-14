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

use marine_module_info_parser::ModuleInfoError;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum PrepareError {
    /// Error that raises on a Wasm module validation.
    #[error("validation error: {0}, probably module is malformed")]
    ParseError(#[from] parity_wasm::elements::Error),

    #[error(transparent)]
    HeapBaseInvalidOrMissing(#[from] HeapBaseError),

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

#[derive(Debug, ThisError)]
pub enum HeapBaseError {
    #[error("a Wasm module doesn't expose __heap_base entry")]
    ExportNotFound,

    #[error("__heap_base is initialized not by i32.const, but by a different set of instructions, that's unsupported")]
    InitializationNotI32Const,

    #[error("__heap_base has not a i32 type")]
    WrongType,
}
