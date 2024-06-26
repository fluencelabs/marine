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

use super::ModuleManifest;
use crate::ModuleInfoResult;
use crate::ModuleInfoError;
use crate::extract_custom_sections_by_name;
use crate::try_as_one_section;

use marine_wasm_backend_traits::Module as ModuleTrait;
use marine_wasm_backend_traits::WasmBackend;

use marine_rs_sdk_main::MANIFEST_SECTION_NAME;
use walrus::ModuleConfig;
use walrus::Module;

use std::borrow::Cow;
use std::path::Path;
use std::convert::TryInto;

pub fn extract_from_path<P>(wasm_module_path: P) -> ModuleInfoResult<ModuleManifest>
where
    P: AsRef<Path>,
{
    let module = ModuleConfig::new()
        .parse_file(wasm_module_path)
        .map_err(ModuleInfoError::CorruptedWasmFile)?;

    extract_from_module(&module)
}

pub fn extract_from_bytes(wasm_module_bytes: &[u8]) -> ModuleInfoResult<ModuleManifest> {
    let module = ModuleConfig::new()
        .parse(wasm_module_bytes)
        .map_err(ModuleInfoError::CorruptedWasmFile)?;

    extract_from_module(&module)
}

pub fn extract_from_module(wasm_module: &Module) -> ModuleInfoResult<ModuleManifest> {
    let sections = extract_custom_sections_by_name(wasm_module, MANIFEST_SECTION_NAME)?;
    let section = try_as_one_section(&sections, MANIFEST_SECTION_NAME)?;

    let manifest = match section {
        Cow::Borrowed(bytes) => (*bytes).try_into(),
        Cow::Owned(vec) => vec.as_slice().try_into(),
    }?;

    Ok(manifest)
}

pub fn extract_from_compiled_module<WB: WasmBackend>(
    module: &<WB as WasmBackend>::Module,
) -> ModuleInfoResult<ModuleManifest> {
    let sections = module.custom_sections(MANIFEST_SECTION_NAME);
    let section = try_as_one_section(sections, MANIFEST_SECTION_NAME)?;
    let manifest = section.as_slice().try_into()?;

    Ok(manifest)
}
