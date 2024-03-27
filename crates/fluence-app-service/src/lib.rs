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
