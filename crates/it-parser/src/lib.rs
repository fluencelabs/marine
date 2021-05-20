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

mod custom;
mod deleter;
mod embedder;
mod errors;
mod extractor;

pub use errors::ITParserError;

pub use deleter::delete_it_section;
pub use deleter::delete_it_section_from_file;

pub use embedder::embed_it;
pub use embedder::embed_text_it;

pub use extractor::extract_it_from_module;
pub use extractor::extract_version_from_module;
pub use extractor::extract_text_it;
pub use extractor::module_interface;
pub use extractor::module_it_interface;

pub mod interface {
    pub use marine_module_interface::interface::ModuleInterface;
    pub use marine_module_interface::interface::RecordType;
    pub use marine_module_interface::interface::RecordField;
    pub use marine_module_interface::interface::FunctionSignature;
}

pub mod it_interface {
    pub use marine_module_interface::it_interface::IModuleInterface;
    pub use marine_module_interface::it_interface::MRecordTypes;
    pub use marine_module_interface::it_interface::MFunctionSignature;

    pub mod it {
        pub use wasmer_it::IType;
        pub use wasmer_it::ast::FunctionArg as IFunctionArg;
        pub use wasmer_it::IRecordType;
        pub use wasmer_it::IRecordFieldType;
    }
}

pub(crate) type ParserResult<T> = std::result::Result<T, ITParserError>;
