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

use wasmer_wit::errors::InstructionError;
use wasmer_runtime::error::{
    CallError, CompileError, CreationError, Error as WasmerError, ResolveError, RuntimeError,
};

use std::error::Error;

#[derive(Debug)]
#[allow(unused)]
pub enum WITFCEError {
    /// Errors for I/O errors raising while opening a file.
    IOError(String),

    /// This error type is produced by Wasmer during resolving a Wasm function.
    WasmerResolveError(String),

    /// Error related to calling a main Wasm module.
    WasmerInvokeError(String),

    /// Error that raises during compilation Wasm code by Wasmer.
    WasmerCreationError(String),

    /// Error that raises during creation of some Wasm objects (like table and memory) by Wasmer.
    WasmerCompileError(String),

    /// Error that raises on the preparation step.
    PrepareError(String),

    /// Indicates that there is already a module with such name.
    NonUniqueModuleName,

    /// Returns when there is no module with such name.
    NoSuchFunction(String),

    /// Returns when there is no module with such name.
    NoSuchModule,

    /// WIT section is absent.
    NoWITSection,

    /// Multiple WIT sections.
    MultipleWITSections,

    /// WIT section remainder isn't empty.
    WITRemainderNotEmpty,

    /// An error occurred while parsing WIT section.
    WITParseError,

    /// Indicates that modules currently in use and couldn't be deleted.
    ModuleInUse,
}

impl Error for WITFCEError {}

impl std::fmt::Display for WITFCEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WITFCEError::IOError(msg) => write!(f, "IOError: {}", msg),
            WITFCEError::WasmerResolveError(msg) => write!(f, "WasmerResolveError: {}", msg),
            WITFCEError::WasmerInvokeError(msg) => write!(f, "WasmerInvokeError: {}", msg),
            WITFCEError::WasmerCompileError(msg) => write!(f, "WasmerCompileError: {}", msg),
            WITFCEError::WasmerCreationError(msg) => write!(f, "WasmerCreationError: {}", msg),
            WITFCEError::PrepareError(msg) => {
                write!(f, "Prepare error: {}, probably module is mailformed", msg)
            }
            WITFCEError::NonUniqueModuleName => {
                write!(f, "FCE already has module with such a name")
            }
            WITFCEError::NoSuchFunction(msg) => {
                write!(f, "FCE doesn't have a function with such a name: {}", msg)
            }
            WITFCEError::NoSuchModule => write!(f, "FCE doesn't have a module with such a name"),
            WITFCEError::ModuleInUse => {
                write!(f, "Module is used by other modules and couldn't be deleted")
            }
            _ => unimplemented!(),
        }
    }
}

impl From<CreationError> for WITFCEError {
    fn from(err: CreationError) -> Self {
        WITFCEError::WasmerCreationError(format!("{}", err))
    }
}

impl From<CompileError> for WITFCEError {
    fn from(err: CompileError) -> Self {
        WITFCEError::WasmerCompileError(format!("{}", err))
    }
}

impl From<CallError> for WITFCEError {
    fn from(err: CallError) -> Self {
        match err {
            CallError::Resolve(err) => WITFCEError::WasmerResolveError(format!("{}", err)),
            CallError::Runtime(err) => WITFCEError::WasmerInvokeError(format!("{}", err)),
        }
    }
}

impl From<ResolveError> for WITFCEError {
    fn from(err: ResolveError) -> Self {
        WITFCEError::WasmerResolveError(format!("{}", err))
    }
}

impl From<RuntimeError> for WITFCEError {
    fn from(err: RuntimeError) -> Self {
        WITFCEError::WasmerInvokeError(format!("{}", err))
    }
}

impl From<WasmerError> for WITFCEError {
    fn from(err: WasmerError) -> Self {
        WITFCEError::WasmerInvokeError(format!("{}", err))
    }
}

impl From<std::io::Error> for WITFCEError {
    fn from(err: std::io::Error) -> Self {
        WITFCEError::IOError(format!("{}", err))
    }
}

impl From<InstructionError> for WITFCEError {
    fn from(err: InstructionError) -> Self {
        WITFCEError::WasmerInvokeError(format!("{}", err))
    }
}
