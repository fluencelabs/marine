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
mod engine;
mod errors;
mod host_imports;
mod misc;
mod module;
mod heap_statistic;

pub use config::MModuleConfig;
pub use config::HostExportedFunc;
pub use config::HostImportDescriptor;
pub use engine::Marine;
pub use engine::MModuleInterface;
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
pub use heap_statistic::ModuleMemoryStat;
pub use heap_statistic::MemoryStat;

pub use wasmer_it::IRecordFieldType;
pub mod ne_vec {
    pub use wasmer_it::NEVec;
}

pub(crate) type MResult<T> = std::result::Result<T, MError>;

use once_cell::sync::Lazy;

use std::str::FromStr;
static MINIMAL_SUPPORTED_SDK_VERSION: Lazy<semver::Version> = Lazy::new(|| {
    semver::Version::from_str("0.6.0").expect("invalid minimal sdk version specified")
});
static MINIMAL_SUPPORTED_IT_VERSION: Lazy<semver::Version> = Lazy::new(|| {
    semver::Version::from_str("0.20.0").expect("invalid minimal sdk version specified")
});

// These locals intended for check that set versions are correct at the start of an application.
thread_local!(static MINIMAL_SUPPORTED_SDK_VERSION_CHECK: &'static semver::Version = Lazy::force(&MINIMAL_SUPPORTED_SDK_VERSION));
thread_local!(static MINIMAL_SUPPORTED_IT_VERSION_CHECK: &'static semver::Version = Lazy::force(&MINIMAL_SUPPORTED_IT_VERSION));

/// Return minimal support version of interface types.
pub fn min_it_version() -> &'static semver::Version {
    Lazy::force(&MINIMAL_SUPPORTED_IT_VERSION)
}

/// Return minimal support version of SDK.
pub fn min_sdk_version() -> &'static semver::Version {
    Lazy::force(&MINIMAL_SUPPORTED_SDK_VERSION)
}
