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

mod block;
mod config;
mod engine;
mod errors;
mod host_imports;
mod misc;

pub use block::IValue;
pub use block::IRecordType;
pub use block::IFunctionArg;
pub use block::IType;
pub use block::RecordTypes;
pub use block::FCEFunctionSignature;
pub use block::from_interface_values;
pub use block::to_interface_value;
pub use config::FCEModuleConfig;
pub use config::HostExportedFunc;
pub use config::HostImportDescriptor;
pub use engine::FCE;
pub use engine::FCEModuleInterface;
pub use errors::FCEError;
pub use host_imports::HostImportError;

pub use wasmer_wit::types::RecordFieldType as IRecordFieldType;
pub mod vec1 {
    pub use wasmer_wit::vec1::Vec1;
}

pub(crate) type Result<T> = std::result::Result<T, FCEError>;
