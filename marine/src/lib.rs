/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
#![warn(rust_2018_idioms)]
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
mod host_imports;
mod errors;
mod marine;
mod marine_interface;
mod module_loading;

pub(crate) type MarineResult<T> = std::result::Result<T, MarineError>;

pub use marine_interface::MarineInterface;

pub use config::ConfigContext;
pub use config::WithContext;
pub use config::MarineWASIConfig;

pub use config::TomlMarineConfig;
pub use config::TomlMarineModuleConfig;
pub use config::TomlMarineNamedModuleConfig;
pub use config::TomlWASIConfig;
pub use config::TomlValue;
pub use config::TomlValueTable;

pub use errors::MarineError;

// Re-exports from Marine
pub use marine_core::IValue;
pub use marine_core::IRecordType;
pub use marine_core::IFunctionArg;
pub use marine_core::IType;
pub use marine_core::MModuleInterface as MarineModuleInterface;
pub use marine_core::MError;
pub use marine_core::MFunctionSignature as MarineFunctionSignature;
pub use marine_core::MemoryStats;
pub use marine_core::ModuleMemoryStat;
pub use marine_core::MRecordTypes;
pub use marine_core::HostImportError;
pub use marine_core::to_interface_value;
pub use marine_core::from_interface_values;
pub use marine_core::ne_vec;

pub use marine_module_interface::interface::itype_text_view;

pub use marine_rs_sdk::CallParameters;
pub use marine_rs_sdk::ParticleParameters;
pub use marine_rs_sdk::SecurityTetraplet;

pub mod generic {
    pub use crate::marine::Marine;
    pub use crate::config::MarineModuleConfig;
    pub use crate::config::ModuleDescriptor;
    pub use crate::config::MarineConfig;

    pub use marine_core::generic::*;
}

#[cfg(feature = "default")]
pub mod wasmtime {
    pub type WasmBackend = marine_core::wasmtime::WasmBackend;

    pub type Marine = crate::marine::Marine<WasmBackend>;
    pub type MarineModuleConfig = crate::config::MarineModuleConfig<WasmBackend>;
    pub type ModuleDescriptor = crate::config::ModuleDescriptor<WasmBackend>;
    pub type MarineConfig = crate::config::MarineConfig<WasmBackend>;

    pub use marine_core::wasmtime::HostExportedFunc;
    pub use marine_core::wasmtime::HostImportDescriptor;
}

#[cfg(feature = "default")]
pub use wasmtime::*;
