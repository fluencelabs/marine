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
use std::io::Error as StdIOError;
use std::error::Error;

#[derive(Debug)]
pub enum WITParserError {
    /// WIT section is absent.
    NoWITSection,

    /// Multiple WIT sections.
    MultipleWITSections,

    /// WIT section remainder isn't empty.
    WITRemainderNotEmpty,

    /// An error occurred while parsing WIT section.
    CorruptedWITSection,

    /// An error occurred while parsing file in Wat format.
    CorruptedWATFile(WATError),

    /// An error occurred while parsing Wasm file
    CorruptedWasmFile(anyhow::Error),

    /// An error occurred while manipulating with converting ast to bytes.
    AstToBytesError(StdIOError),

    // Wasm emittig file error.
    WasmEmitError(anyhow::Error),
}

impl Error for WITParserError {}

impl std::fmt::Display for WITParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WITParserError::NoWITSection => write!(f, "Loaded module doesn't contain WIT section"),
            WITParserError::MultipleWITSections => write!(
                f,
                "Loaded module contains multiple WIT sections that is unsupported now"
            ),
            WITParserError::WITRemainderNotEmpty => write!(
                f,
                "WIT section remainder isn't empty - WIT section possibly corrupted"
            ),
            WITParserError::CorruptedWITSection => write!(f, "WIT section is corrupted"),
            WITParserError::CorruptedWATFile(err) => {
                write!(f, "an error occurred while parsing wat file: {}", err)
            }
            WITParserError::CorruptedWasmFile(err) => {
                write!(f, "Failed to parse the Wasm module: {}", err)
            }
            WITParserError::AstToBytesError(err) => {
                write!(f, "Wasm AST converting to bytes failed with: {}", err)
            }
            WITParserError::WasmEmitError(err) => write!(f, "Failed to emit Wasm file: {}", err),
        }
    }
}

impl From<WATError> for WITParserError {
    fn from(err: WATError) -> Self {
        WITParserError::CorruptedWATFile(err)
    }
}

impl From<StdIOError> for WITParserError {
    fn from(err: StdIOError) -> Self {
        WITParserError::AstToBytesError(err)
    }
}
