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

use crate::ModuleInfoResult;
use crate::ModuleInfoError;
use marine_rs_sdk_main::VERSION_SECTION_NAME;

use walrus::ModuleConfig;
use walrus::CustomSection;
use walrus::IdsToIndices;

use std::path::Path;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub(super) struct VersionCustomSection(String);

impl CustomSection for VersionCustomSection {
    fn name(&self) -> &str {
        VERSION_SECTION_NAME
    }

    fn data(&self, _ids_to_indices: &IdsToIndices) -> Cow<'_, [u8]> {
        Cow::Borrowed(self.0.as_bytes())
    }
}

/// Embed provided IT to a Wasm module.
pub fn embed_from_module(
    mut wasm_module: walrus::Module,
    version: &semver::Version,
) -> walrus::Module {
    delete_version_sections(&mut wasm_module);

    let custom = VersionCustomSection(version.to_string());
    wasm_module.customs.add(custom);

    wasm_module
}

pub fn embed_from_path<I, O>(
    in_wasm_module_path: I,
    out_wasm_module_path: O,
    version: &semver::Version,
) -> ModuleInfoResult<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let wasm_module = ModuleConfig::new()
        .parse_file(in_wasm_module_path)
        .map_err(ModuleInfoError::CorruptedWasmFile)?;

    let mut wasm_module = embed_from_module(wasm_module, version);
    wasm_module
        .emit_wasm_file(out_wasm_module_path)
        .map_err(ModuleInfoError::WasmEmitError)
}

// TODO: deduplicate it with wit_parser::delete_wit_section_from_file
fn delete_version_sections(wasm_module: &mut walrus::Module) {
    let version_section_ids = wasm_module
        .customs
        .iter()
        .filter_map(|(id, section)| {
            if section.name() == VERSION_SECTION_NAME {
                Some(id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for id in version_section_ids {
        wasm_module.customs.delete(id);
    }
}
