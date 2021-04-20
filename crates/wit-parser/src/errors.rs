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

use wasmer_wit::decoders::wat::Error as WATError;
use thiserror::Error as ThisError;

use std::io::Error as IOError;

#[derive(Debug, ThisError)]
pub enum WITParserError {
    /// WIT section is absent.
    #[error("the module doesn't contain IT section")]
    NoITSection,

    /// Multiple WIT sections.
    #[error("the module contains multiple IT sections that is unsupported")]
    MultipleITSections,

    /// WIT section remainder isn't empty.
    #[error("IT section is corrupted: IT section remainder isn't empty")]
    ITRemainderNotEmpty,

    /// An error occurred while parsing WIT section.
    #[error(
        "IT section is corrupted: {0}.\
    \nProbably the module was compiled with an old of fce cli, please try to update and recompile.\
    \nTo update fce run: cargo install fcli --force"
    )]
    CorruptedITSection(nom::Err<(Vec<u8>, nom::error::ErrorKind)>),

    /// An error related to incorrect data of wit section.
    #[error("{0}")]
    IncorrectITFormat(String),

    /// An error occurred while parsing file in Wat format.
    #[error("provided file with IT definitions is corrupted: {0}")]
    CorruptedITFile(WATError),

    /// An error occurred while parsing Wasm file.
    #[error("provided Wasm file is corrupted: {0}")]
    CorruptedWasmFile(anyhow::Error),

    /// An error occurred while manipulating with converting ast to bytes.
    #[error("Convertation Wast to AST failed with: {0}")]
    AstToBytesError(IOError),

    /// Wasm emitting file error.
    #[error("Emitting resulted Wasm file failed with: {0}")]
    WasmEmitError(anyhow::Error),
}

impl From<WATError> for WITParserError {
    fn from(err: WATError) -> Self {
        WITParserError::CorruptedITFile(err)
    }
}

impl From<IOError> for WITParserError {
    fn from(err: IOError) -> Self {
        WITParserError::AstToBytesError(err)
    }
}
