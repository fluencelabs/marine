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

mod default_export_api_config;
mod errors;
mod instructions_generator;
mod interface_generator;

pub use interface_generator::embed_wit;
pub use errors::WITGeneratorError;

pub(crate) type Result<T> = std::result::Result<T, WITGeneratorError>;

pub(crate) use wasmer_wit::ast::Interfaces;
pub(crate) use wasmer_wit::ast::Adapter as AstAdapter;
pub(crate) use wasmer_wit::ast::Type as AstType;
pub(crate) use wasmer_wit::ast::Export as AstExport;
pub(crate) use wasmer_wit::ast::Import as AstImport;
pub(crate) use wasmer_wit::ast::Implementation as AstImplementation;
pub(crate) use wasmer_wit::types::InterfaceType as IType;
pub(crate) use wasmer_wit::types::RecordType as IRecordType;
pub(crate) use wasmer_wit::types::FunctionArg as IFunctionArg;
pub(crate) use wasmer_wit::types::FunctionType as IFunctionType;
