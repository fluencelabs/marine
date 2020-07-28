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

use fce_wit_generator::WITGeneratorError;
use fce_wit_parser::WITParserError;

use std::io::Error as StdIOError;
use std::error::Error;

#[derive(Debug)]
pub enum CLIError {
    /// Unknown command was entered by user.
    NoSuchCommand(String),

    /// An error occurred while generating interface types.
    WITGeneratorError(WITGeneratorError),

    /// An error occurred while parsing interface types.
    WITParserError(WITParserError),

    /// An error occurred when no Wasm file was compiled.
    WasmCompilationError(String),

    /// Various errors related to I/O operations.
    IOError(StdIOError),
}

impl Error for CLIError {}

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CLIError::NoSuchCommand(cmd) => write!(f, "{} is unknown command", cmd),
            CLIError::WITGeneratorError(err) => write!(f, "{}", err),
            CLIError::WITParserError(err) => write!(f, "{}", err),
            CLIError::WasmCompilationError(err) => write!(f, "{}", err),
            CLIError::IOError(err) => write!(f, "{:?}", err),
        }
    }
}

impl From<WITGeneratorError> for CLIError {
    fn from(err: WITGeneratorError) -> Self {
        CLIError::WITGeneratorError(err)
    }
}

impl From<WITParserError> for CLIError {
    fn from(err: WITParserError) -> Self {
        CLIError::WITParserError(err)
    }
}

impl From<StdIOError> for CLIError {
    fn from(err: StdIOError) -> Self {
        CLIError::IOError(err)
    }
}
