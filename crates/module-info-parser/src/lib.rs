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

pub mod manifest;
pub mod sdk_version;
pub mod effects;
mod custom_section_extractor;
mod errors;

pub use errors::ModuleInfoError;

pub(crate) use custom_section_extractor::extract_custom_sections_by_name;
pub(crate) use custom_section_extractor::try_as_one_section;

pub(crate) type ModuleInfoResult<T> = std::result::Result<T, ModuleInfoError>;
