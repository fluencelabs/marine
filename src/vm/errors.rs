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
pub enum FCEError {
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

    /// Indicates that modules currently in use and couldn't be deleted.
    ModuleInUse,
}

impl Error for FCEError {}

impl std::fmt::Display for FCEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FCEError::IOError(msg) => write!(f, "IOError: {}", msg),
            FCEError::WasmerResolveError(msg) => write!(f, "WasmerResolveError: {}", msg),
            FCEError::WasmerInvokeError(msg) => write!(f, "WasmerInvokeError: {}", msg),
            FCEError::WasmerCompileError(msg) => write!(f, "WasmerCompileError: {}", msg),
            FCEError::WasmerCreationError(msg) => write!(f, "WasmerCreationError: {}", msg),
            FCEError::PrepareError(msg) => {
                write!(f, "Prepare error: {}, probably module is mailformed", msg)
            }
            FCEError::NonUniqueModuleName => write!(f, "FCE already has module with such name"),
            FCEError::NoSuchModule => write!(f, "FCE doesn't have a module with such name"),
            FCEError::ModuleInUse => {
                write!(f, "Module is used by other modules and couldn't be deleted")
            }
        }
    }
}

impl From<CreationError> for FCEError {
    fn from(err: CreationError) -> Self {
        FCEError::WasmerCreationError(format!("{}", err))
    }
}

impl From<CompileError> for FCEError {
    fn from(err: CompileError) -> Self {
        FCEError::WasmerCompileError(format!("{}", err))
    }
}

impl From<parity_wasm::elements::Error> for FCEError {
    fn from(err: parity_wasm::elements::Error) -> Self {
        FCEError::PrepareError(format!("{}", err))
    }
}

impl From<CallError> for FCEError {
    fn from(err: CallError) -> Self {
        match err {
            CallError::Resolve(err) => FCEError::WasmerResolveError(format!("{}", err)),
            CallError::Runtime(err) => FCEError::WasmerInvokeError(format!("{}", err)),
        }
    }
}

impl From<ResolveError> for FCEError {
    fn from(err: ResolveError) -> Self {
        FCEError::WasmerResolveError(format!("{}", err))
    }
}

impl From<RuntimeError> for FCEError {
    fn from(err: RuntimeError) -> Self {
        FCEError::WasmerInvokeError(format!("{}", err))
    }
}

impl From<WasmerError> for FCEError {
    fn from(err: WasmerError) -> Self {
        FCEError::WasmerInvokeError(format!("{}", err))
    }
}

impl From<std::io::Error> for FCEError {
    fn from(err: std::io::Error) -> Self {
        FCEError::IOError(format!("{}", err))
    }
}
