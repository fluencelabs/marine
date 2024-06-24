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
use super::SDKVersionError;
use crate::extract_custom_sections_by_name;
use crate::try_as_one_section;

use marine_wasm_backend_traits::WasmBackend;
use marine_wasm_backend_traits::Module as WasmModule;

use marine_rs_sdk_main::VERSION_SECTION_NAME;
use walrus::ModuleConfig;
use walrus::Module;

use std::borrow::Cow;
use std::str::FromStr;
use std::path::Path;

pub fn extract_from_path<P>(wasm_module_path: P) -> ModuleInfoResult<semver::Version>
where
    P: AsRef<Path>,
{
    let module = ModuleConfig::new()
        .parse_file(wasm_module_path)
        .map_err(ModuleInfoError::CorruptedWasmFile)?;

    extract_from_module(&module)
}

pub fn extract_from_module(wasm_module: &Module) -> ModuleInfoResult<semver::Version> {
    let sections = extract_custom_sections_by_name(wasm_module, VERSION_SECTION_NAME)?;
    let section = try_as_one_section(&sections, VERSION_SECTION_NAME)?;

    let version = match section {
        Cow::Borrowed(bytes) => as_semver(bytes),
        Cow::Owned(vec) => as_semver(vec),
    }?;

    Ok(version)
}

pub fn extract_from_compiled_module<WB: WasmBackend>(
    wasm_module: &<WB as WasmBackend>::Module,
) -> ModuleInfoResult<semver::Version> {
    let sections = wasm_module.custom_sections(VERSION_SECTION_NAME);
    let section = try_as_one_section(sections, VERSION_SECTION_NAME)?;
    let version = as_semver(section)?;

    Ok(version)
}

fn as_semver(version_as_bytes: &[u8]) -> Result<semver::Version, super::SDKVersionError> {
    match std::str::from_utf8(version_as_bytes) {
        Ok(str) => Ok(semver::Version::from_str(str)?),
        Err(e) => Err(SDKVersionError::VersionNotValidUtf8(e)),
    }
}
