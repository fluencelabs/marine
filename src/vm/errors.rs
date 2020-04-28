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

use wasmer_runtime::error::{
    CallError, CompileError, CreationError, Error as WasmerError, ResolveError, RuntimeError,
};

use std::error::Error;

#[derive(Debug)]
pub enum FrankError {
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

    /// Returns where there is no module with such name.
    NoSuchModule,
}

impl Error for FrankError {}

impl std::fmt::Display for FrankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FrankError::IOError(msg) => write!(f, "IOError: {}", msg),
            FrankError::WasmerResolveError(msg) => write!(f, "WasmerResolveError: {}", msg),
            FrankError::WasmerInvokeError(msg) => write!(f, "WasmerInvokeError: {}", msg),
            FrankError::WasmerCompileError(msg) => write!(f, "WasmerCompileError: {}", msg),
            FrankError::WasmerCreationError(msg) => write!(f, "WasmerCreationError: {}", msg),
            FrankError::PrepareError(msg) => {
                write!(f, "Prepare error: {}, probably module is mailformed", msg)
            }
            FrankError::NonUniqueModuleName => write!(f, "Frank already has module with such name"),
            FrankError::NoSuchModule => write!(f, "Frank doesn't have a module with such name"),
        }
    }
}

impl From<CreationError> for FrankError {
    fn from(err: CreationError) -> Self {
        FrankError::WasmerCreationError(format!("{}", err))
    }
}

impl From<CompileError> for FrankError {
    fn from(err: CompileError) -> Self {
        FrankError::WasmerCompileError(format!("{}", err))
    }
}

impl From<parity_wasm::elements::Error> for FrankError {
    fn from(err: parity_wasm::elements::Error) -> Self {
        FrankError::PrepareError(format!("{}", err))
    }
}

impl From<CallError> for FrankError {
    fn from(err: CallError) -> Self {
        match err {
            CallError::Resolve(err) => FrankError::WasmerResolveError(format!("{}", err)),
            CallError::Runtime(err) => FrankError::WasmerInvokeError(format!("{}", err)),
        }
    }
}

impl From<ResolveError> for FrankError {
    fn from(err: ResolveError) -> Self {
        FrankError::WasmerResolveError(format!("{}", err))
    }
}

impl From<RuntimeError> for FrankError {
    fn from(err: RuntimeError) -> Self {
        FrankError::WasmerInvokeError(format!("{}", err))
    }
}

impl From<WasmerError> for FrankError {
    fn from(err: WasmerError) -> Self {
        FrankError::WasmerInvokeError(format!("{}", err))
    }
}

impl From<std::io::Error> for FrankError {
    fn from(err: std::io::Error) -> Self {
        FrankError::IOError(format!("{}", err))
    }
}
