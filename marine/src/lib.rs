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
mod marine;
mod marine_interface;
mod module_loading;

pub(crate) type MarineResult<T> = std::result::Result<T, MarineError>;

pub use crate::marine::Marine;
pub use marine_interface::MarineInterface;

pub use config::ConfigContext;
pub use config::WithContext;
pub use config::MarineConfig;
pub use config::MarineModuleConfig;
pub use config::MarineWASIConfig;
pub use config::ModuleDescriptor;

pub use config::TomlMarineConfig;
pub use config::TomlMarineModuleConfig;
pub use config::TomlMarineNamedModuleConfig;
pub use config::TomlWASIConfig;

pub use errors::MarineError;

// Re-exports from Marine
pub use marine_core::IValue;
pub use marine_core::IRecordType;
pub use marine_core::IFunctionArg;
pub use marine_core::IType;
pub use marine_core::MModuleInterface as MarineModuleInterface;
pub use marine_core::MFunctionSignature as MarineFunctionSignature;
pub use marine_core::MemoryStats;
pub use marine_core::ModuleMemoryStat;
pub use marine_core::MRecordTypes;
pub use marine_core::HostExportedFunc;
pub use marine_core::HostImportDescriptor;
pub use marine_core::HostImportError;
pub use marine_core::to_interface_value;
pub use marine_core::from_interface_values;
pub use marine_core::ne_vec;

pub use marine_module_interface::interface::itype_text_view;

pub use marine_rs_sdk::CallParameters;
pub use marine_rs_sdk::SecurityTetraplet;
