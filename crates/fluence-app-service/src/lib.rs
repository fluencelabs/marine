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

pub(crate) type Result<T> = std::result::Result<T, AppServiceError>;

pub type WasmBackend = marine::DefaultWasmBackend;

pub use errors::AppServiceError;
pub type AppService = service::AppService<WasmBackend>;
pub use service_interface::FunctionSignature;
pub use service_interface::RecordType;
pub use service_interface::ServiceInterface;

pub type AppServiceConfig = config::AppServiceConfig<WasmBackend>;
pub use raw_toml_config::TomlAppServiceConfig;

pub use marine::ConfigContext;
pub use marine::WithContext;
pub type MarineConfig = marine::MarineConfig<WasmBackend>;
pub type MarineModuleConfig = marine::MarineModuleConfig<WasmBackend>;
pub use marine::MarineWASIConfig;
pub use marine::TomlMarineConfig;
pub use marine::TomlMarineModuleConfig;
pub use marine::TomlMarineNamedModuleConfig;
pub use marine::TomlWASIConfig;
pub type ModuleDescriptor = marine::ModuleDescriptor<WasmBackend>;

pub use marine::MarineError;

pub use marine::IValue;
pub use marine::IRecordType;
pub use marine::IFunctionArg;
pub use marine::IType;
pub type HostImportDescriptor = marine::HostImportDescriptor<WasmBackend>;
pub use marine::HostImportError;
pub use marine::to_interface_value;
pub use marine::from_interface_values;
pub use marine::ModuleMemoryStat;
pub use marine::MemoryStats;
pub use marine::ne_vec;

pub use marine_min_it_version::min_sdk_version;
pub use marine_min_it_version::min_it_version;

pub use marine::CallParameters;
pub use marine::SecurityTetraplet;
