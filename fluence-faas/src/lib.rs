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

mod errors;
mod faas;
mod faas_interface;
mod misc;

pub(crate) type Result<T> = std::result::Result<T, FaaSError>;

pub use errors::FaaSError;

pub use fce::IValue;
pub use fce::IRecordType;
pub use fce::IFunctionArg;
pub use fce::IType;
pub use fce::to_interface_value;
pub use fce::from_interface_values;

pub use fluence_sdk_main::CallParameters;

pub use faas::FluenceFaaS;
pub use faas_interface::FaaSInterface;
pub use faas_interface::FaaSFunctionSignature;

pub use misc::RawModulesConfig;
pub use misc::RawModuleConfig;
pub use misc::ModulesConfig;
pub use misc::ModuleConfig;
pub use misc::WASIConfig;
