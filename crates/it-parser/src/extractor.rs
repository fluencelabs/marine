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

mod it;

pub use it::*;

use crate::interface::ModuleInterface;
use crate::it_interface::IModuleInterface;
use crate::ParserResult;
use crate::ITParserError;

use marine_module_interface::it_interface;
use marine_module_interface::interface;
use marine_it_interfaces::MITInterfaces;
use std::path::Path;

pub fn module_interface<P>(module_path: P) -> ParserResult<ModuleInterface>
where
    P: AsRef<Path>,
{
    create_mit_with(module_path, |it| interface::get_interface(&it))
}

pub fn module_it_interface<P>(module_path: P) -> ParserResult<IModuleInterface>
where
    P: AsRef<Path>,
{
    create_mit_with(module_path, |it| it_interface::get_interface(&it))
}

fn create_mit_with<P, T, E>(
    module_path: P,
    transformer: impl FnOnce(MITInterfaces<'_>) -> std::result::Result<T, E>,
) -> ParserResult<T>
where
    P: AsRef<Path>,
    ITParserError: From<E>,
{
    let module = walrus::ModuleConfig::new()
        .parse_file(module_path)
        .map_err(ITParserError::CorruptedWasmFile)?;
    let raw_custom_section = extract_custom_section(&module)?;
    let custom_section_bytes = raw_custom_section.as_ref();
    let it = extract_it_from_bytes(custom_section_bytes)?;

    let mit = MITInterfaces::new(it);

    transformer(mit).map_err(Into::into)
}
