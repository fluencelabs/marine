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

pub use config::FCEModuleConfig;
pub use config::HostExportedFunc;
pub use config::HostImportDescriptor;
pub use engine::FCE;
pub use engine::FCEModuleInterface;
pub use errors::FCEError;
pub use host_imports::HostImportError;
pub use module::IValue;
pub use module::IRecordType;
pub use module::IFunctionArg;
pub use module::IType;
pub use module::RecordTypes;
pub use module::FCEFunctionSignature;
pub use module::from_interface_values;
pub use module::to_interface_value;

pub use wasmer_wit::IRecordFieldType;
pub mod ne_vec {
    pub use wasmer_wit::NEVec;
}

pub(crate) type Result<T> = std::result::Result<T, FCEError>;
