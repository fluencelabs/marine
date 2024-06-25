/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::ModuleInfoResult;
use crate::ModuleInfoError;

use walrus::IdsToIndices;
use walrus::Module;

use std::borrow::Cow;

pub(super) fn extract_custom_sections_by_name<'w>(
    wasm_module: &'w Module,
    section_name: &str,
) -> ModuleInfoResult<Vec<Cow<'w, [u8]>>> {
    let default_ids = IdsToIndices::default();

    let sections = wasm_module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == section_name)
        .map(|s| s.1.data(&default_ids))
        .collect::<Vec<_>>();

    Ok(sections)
}

pub(super) fn try_as_one_section<'s, T: Sized>(
    sections: &'s [T],
    section_name: &'static str,
) -> ModuleInfoResult<&'s T> {
    let sections_count = sections.len();

    if sections_count > 1 {
        return Err(ModuleInfoError::MultipleCustomSections(
            section_name,
            sections_count,
        ));
    }

    if sections_count == 0 {
        return Err(ModuleInfoError::NoCustomSection(section_name));
    }

    Ok(&sections[0])
}
