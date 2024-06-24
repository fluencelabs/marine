/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
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
    pub use marine_module_interface::it_interface::IRecordTypes;
    pub use marine_module_interface::it_interface::IFunctionSignature;

    pub mod it {
        pub use wasmer_it::IType;
        pub use wasmer_it::ast::FunctionArg as IFunctionArg;
        pub use wasmer_it::IRecordType;
        pub use wasmer_it::IRecordFieldType;
    }
}

pub(crate) type ParserResult<T> = std::result::Result<T, ITParserError>;
