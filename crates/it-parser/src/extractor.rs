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
