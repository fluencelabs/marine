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

use crate::Result;
use crate::ModuleInfoError;
use crate::extract_custom_sections_by_name;
use crate::try_as_one_section;

use fluence_sdk_main::VERSION_SECTION_NAME;
use walrus::ModuleConfig;
use walrus::Module;

use std::borrow::Cow;
use std::str::FromStr;
use std::path::Path;

pub fn extract_sdk_version_by_path(wasm_module_path: &Path) -> Result<Option<semver::Version>> {
    let module = ModuleConfig::new()
        .parse_file(wasm_module_path)
        .map_err(ModuleInfoError::CorruptedWasmFile)?;

    extract_sdk_version_by_module(&module)
}

pub fn extract_sdk_version_by_module(wasm_module: &Module) -> Result<Option<semver::Version>> {
    let sections = extract_custom_sections_by_name(&wasm_module, VERSION_SECTION_NAME)?;

    if sections.is_empty() {
        return Ok(None);
    }
    let section = try_as_one_section(sections, VERSION_SECTION_NAME)?;

    let version = match section {
        Cow::Borrowed(bytes) => as_semver(bytes),
        Cow::Owned(vec) => as_semver(&vec),
    };

    version.map(Some)
}

fn as_semver(version_as_bytes: &[u8]) -> Result<semver::Version> {
    match std::str::from_utf8(version_as_bytes) {
        Ok(str) => Ok(semver::Version::from_str(str)?),
        Err(e) => Err(ModuleInfoError::VersionNotValidUtf8(e)),
    }
}
