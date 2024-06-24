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
