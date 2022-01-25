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

pub use errors::AppServiceError;
pub use service::AppService;
pub use service_interface::FunctionSignature;
pub use service_interface::RecordType;
pub use service_interface::ServiceInterface;

pub use config::AppServiceConfig;
pub use raw_toml_config::TomlAppServiceConfig;

pub use fluence_faas::FaaSConfig;
pub use fluence_faas::FaaSModuleConfig;
pub use fluence_faas::FaaSWASIConfig;
pub use fluence_faas::TomlFaaSConfig;
pub use fluence_faas::TomlFaaSModuleConfig;
pub use fluence_faas::TomlFaaSNamedModuleConfig;
pub use fluence_faas::TomlWASIConfig;
pub use fluence_faas::ModuleDescriptor;

pub use fluence_faas::FaaSError;

pub use fluence_faas::IValue;
pub use fluence_faas::IRecordType;
pub use fluence_faas::IFunctionArg;
pub use fluence_faas::IType;
pub use fluence_faas::HostImportDescriptor;
pub use fluence_faas::HostImportError;
pub use fluence_faas::to_interface_value;
pub use fluence_faas::from_interface_values;
pub use fluence_faas::ModuleMemoryStat;
pub use fluence_faas::MemoryStats;
pub use fluence_faas::ne_vec;

pub use fluence_faas::min_sdk_version;
pub use fluence_faas::min_it_version;

pub use fluence_faas::CallParameters;
pub use fluence_faas::SecurityTetraplet;
