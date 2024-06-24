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
