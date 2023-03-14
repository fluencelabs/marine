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

use marine_module_interface::interface::InterfaceError;
use marine_module_interface::it_interface::ITInterfaceError;

use wasmer_it::decoders::wat::Error as WATError;
use thiserror::Error as ThisError;

use std::io::Error as IOError;

#[derive(Debug, ThisError)]
pub enum ITParserError {
    /// IT section is absent.
    #[error("the module doesn't contain IT section")]
    NoITSection,

    /// Multiple IT sections.
    #[error("the module contains multiple IT sections that is unsupported")]
    MultipleITSections,

    /// IT section remainder isn't empty.
    #[error("IT section is corrupted: IT section remainder isn't empty")]
    ITRemainderNotEmpty,

    /// An error occurred while parsing IT section.
    #[error(
        "IT section is corrupted: {0}.\
    \nProbably the module was compiled with an old version of marine cli, please try to update and recompile.\
    \nTo update marine run: cargo install marine --force"
    )]
    CorruptedITSection(nom::Err<(Vec<u8>, nom::error::ErrorKind)>),

    /// An error related to incorrect data in IT section.
    #[error("0")]
    IncorrectITFormat(String), // TODO: use a proper error type

    /// An error occurred while processing module interface.
    #[error(transparent)]
    ModuleInterfaceError(#[from] InterfaceError),

    /// An error occurred while processing module IT interface.
    #[error(transparent)]
    ModuleITInterfaceError(#[from] ITInterfaceError),

    /// An error occurred while parsing file in Wat format.
    #[error("provided file with IT definitions is corrupted: {0}")]
    CorruptedITFile(#[from] WATError),

    /// An error occurred while parsing Wasm file.
    #[error("provided Wasm file is corrupted: {0}")]
    CorruptedWasmFile(anyhow::Error),

    /// An error occurred while manipulating with converting ast to bytes.
    #[error("Convertation Wast to AST failed with: {0}")]
    AstToBytesError(#[from] IOError),

    /// Wasm emitting file error.
    #[error("Emitting resulted Wasm file failed with: {0}")]
    WasmEmitError(anyhow::Error),
}
