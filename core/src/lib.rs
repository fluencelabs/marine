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
#![warn(rust_2018_idioms)]
#![feature(get_mut_unchecked)]
#![feature(new_uninit)]
#![feature(stmt_expr_attributes)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod config;
mod marine_core;
mod errors;
mod host_imports;
mod misc;
mod module;
mod memory_statistic;

pub use crate::marine_core::MModuleInterface;
pub use config::MarineCoreConfig;
pub use config::MarineCoreConfigBuilder;
pub use config::INFINITE_MEMORY_LIMIT;
pub use config::HostAPIVersion;
pub use errors::MError;
pub use host_imports::HostImportError;
pub use module::IValue;
pub use module::IRecordType;
pub use module::IFunctionArg;
pub use module::IType;
pub use module::MRecordTypes;
pub use module::MFunctionSignature;
pub use module::from_interface_values;
pub use module::to_interface_value;
pub use memory_statistic::ModuleMemoryStat;
pub use memory_statistic::MemoryStats;

pub use wasmer_it::IRecordFieldType;
pub mod ne_vec {
    pub use wasmer_it::NEVec;
}

pub(crate) type MResult<T> = std::result::Result<T, MError>;

pub mod generic {
    pub use crate::config::MModuleConfig;
    pub use crate::config::HostExportedFunc;
    pub use crate::config::HostImportDescriptor;
    pub use crate::marine_core::MarineCore;
}

#[cfg(feature = "default")]
pub mod wasmtime {
    pub type WasmBackend = marine_wasmtime_backend::WasmtimeWasmBackend;

    pub type MModuleConfig = crate::config::MModuleConfig<WasmBackend>;
    pub type HostExportedFunc = crate::config::HostExportedFunc<WasmBackend>;
    pub type HostImportDescriptor = crate::config::HostImportDescriptor<WasmBackend>;
    pub type MarineCore = crate::marine_core::MarineCore<WasmBackend>;
}

#[cfg(feature = "default")]
pub use crate::wasmtime::*;
