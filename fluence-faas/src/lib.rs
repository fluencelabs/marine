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
mod logger;
mod faas;
mod faas_interface;
mod misc;
mod raw_toml_config;

pub(crate) type Result<T> = std::result::Result<T, FaaSError>;

pub use faas::FluenceFaaS;
pub use faas_interface::FaaSInterface;
pub use faas_interface::itype_text_view;

pub use config::FaaSConfig;
pub use config::FaaSModuleConfig;
pub use config::FaaSWASIConfig;

pub use raw_toml_config::TomlFaaSConfig;
pub use raw_toml_config::TomlFaaSModuleConfig;
pub use raw_toml_config::TomlFaaSNamedModuleConfig;
pub use raw_toml_config::TomlWASIConfig;
pub use raw_toml_config::from_toml_faas_config;
pub use raw_toml_config::from_toml_module_config;
pub use raw_toml_config::from_toml_named_module_config;
pub use raw_toml_config::from_toml_wasi_config;

pub use errors::FaaSError;

// Re-exports from FCE
pub use fce::IValue;
pub use fce::IRecordType;
pub use fce::IFunctionArg;
pub use fce::IType;
pub use fce::FCEModuleInterface as FaaSModuleInterface;
pub use fce::FCEFunctionSignature as FaaSFunctionSignature;
pub use fce::RecordTypes;
pub use fce::HostExportedFunc;
pub use fce::HostImportDescriptor;
pub use fce::HostImportError;
pub use fce::to_interface_value;
pub use fce::from_interface_values;
pub use fce::vec1;

pub use fluence_sdk_main::CallParameters;

pub use wasmer_core::vm::Ctx;
