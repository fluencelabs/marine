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

pub mod manifest;
pub mod sdk_version;
pub mod effects;
mod custom_section_extractor;
mod errors;

pub use errors::ModuleInfoError;

pub(crate) use custom_section_extractor::extract_custom_sections_by_name;
pub(crate) use custom_section_extractor::try_as_one_section;

pub(crate) type ModuleInfoResult<T> = std::result::Result<T, ModuleInfoError>;
