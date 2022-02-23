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
mod host_imports;
mod errors;
mod faas;
mod faas_interface;
mod module_loading;

pub(crate) type FaaSResult<T> = std::result::Result<T, FaaSError>;

pub use faas::FluenceFaaS;
pub use faas_interface::FaaSInterface;

pub use config::FaaSConfig;
pub use config::FaaSModuleConfig;
pub use config::FaaSWASIConfig;
pub use config::ModuleDescriptor;

pub use config::TomlFaaSConfig;
pub use config::TomlFaaSModuleConfig;
pub use config::TomlFaaSNamedModuleConfig;
pub use config::TomlWASIConfig;

pub use errors::FaaSError;

// Re-exports from Marine
pub use marine::IValue;
pub use marine::IRecordType;
pub use marine::IFunctionArg;
pub use marine::IType;
pub use marine::MModuleInterface as FaaSModuleInterface;
pub use marine::MFunctionSignature as FaaSFunctionSignature;
pub use marine::MemoryStats;
pub use marine::ModuleMemoryStat;
pub use marine::MRecordTypes;
pub use marine::HostExportedFunc;
pub use marine::HostImportDescriptor;
pub use marine::HostImportError;
pub use marine::to_interface_value;
pub use marine::from_interface_values;
pub use marine::ne_vec;

pub use marine_module_interface::interface::itype_text_view;

pub use marine_rs_sdk::CallParameters;
pub use marine_rs_sdk::SecurityTetraplet;

pub use wasmer_core::vm::Ctx;
