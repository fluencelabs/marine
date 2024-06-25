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
mod errors;
mod service;
mod service_interface;
mod raw_toml_config;
mod app_service_factory;

pub(crate) type Result<T> = std::result::Result<T, AppServiceError>;

pub use errors::AppServiceError;
pub use service_interface::FunctionSignature;
pub use service_interface::RecordType;
pub use service_interface::ServiceInterface;

pub use raw_toml_config::TomlAppServiceConfig;

pub use marine::ConfigContext;
pub use marine::WithContext;
pub use marine::TomlMarineConfig;
pub use marine::TomlMarineModuleConfig;
pub use marine::TomlMarineNamedModuleConfig;
pub use marine::TomlValue;
pub use marine::TomlValueTable;
pub use marine::TomlWASIConfig;

pub use marine::MarineError;
pub use marine::MError;

pub use marine::IValue;
pub use marine::IRecordType;
pub use marine::IFunctionArg;
pub use marine::IType;
pub use marine::HostImportError;
pub use marine::to_interface_value;
pub use marine::from_interface_values;
pub use marine::ModuleMemoryStat;
pub use marine::MemoryStats;
pub use marine::ne_vec;

pub use marine_min_it_version::min_sdk_version;
pub use marine_min_it_version::min_it_version;

pub use marine::CallParameters;
pub use marine::ParticleParameters;
pub use marine::SecurityTetraplet;

pub mod generic {
    pub use crate::service::AppService;
    pub use crate::app_service_factory::AppServiceFactory;
    pub use crate::config::AppServiceConfig;

    pub use marine::generic::MarineConfig;
    pub use marine::generic::MarineModuleConfig;
    pub use marine::generic::ModuleDescriptor;
    pub use marine::generic::HostImportDescriptor;
}

#[cfg(feature = "wasmtime")]
pub mod wasmtime {
    pub type WasmBackend = marine_wasmtime_backend::WasmtimeWasmBackend;

    pub use marine_wasmtime_backend::WasmtimeConfig;

    pub type AppService = crate::service::AppService<WasmBackend>;
    pub type AppServiceFactory = crate::app_service_factory::AppServiceFactory<WasmBackend>;
    pub type AppServiceConfig = crate::config::AppServiceConfig<WasmBackend>;
    pub use crate::app_service_factory::EpochTicker;

    pub use marine::MarineConfig;
    pub use marine::MarineModuleConfig;
    pub use marine::MarineWASIConfig;
    pub use marine::ModuleDescriptor;
    pub use marine::HostImportDescriptor;
}

#[cfg(feature = "wasmtime")]
pub use wasmtime::*;
