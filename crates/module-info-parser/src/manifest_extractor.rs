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
use crate::ManifestParserError;
use crate::extract_custom_sections_by_name;
use crate::ModuleManifest;

use fluence_sdk_main::MANIFEST_SECTION_NAME;
use walrus::ModuleConfig;
use walrus::Module;

use std::borrow::Cow;
use std::path::Path;
use std::convert::TryInto;

pub fn extract_manifest_by_path(wasm_module_path: &Path) -> Result<ModuleManifest> {
    let module = ModuleConfig::new()
        .parse_file(wasm_module_path)
        .map_err(ManifestParserError::CorruptedWasmFile)?;

    extract_version_by_module(&module)
}

pub fn extract_version_by_module(wasm_module: &Module) -> Result<ModuleManifest> {
    let sections = extract_custom_sections_by_name(&wasm_module, MANIFEST_SECTION_NAME)?;
    let section = as_one_section(sections)?;

    match section {
        Cow::Borrowed(bytes) => bytes.try_into(),
        Cow::Owned(vec) => vec.as_slice().try_into(),
    }
}

fn as_one_section(mut sections: Vec<Cow<'_, [u8]>>) -> Result<Cow<'_, [u8]>> {
    let sections_count = sections.len();

    if sections_count > 1 {
        return Err(ManifestParserError::MultipleVersionSections(sections_count));
    }

    if sections_count == 0 {
        return Err(ManifestParserError::NoVersionSection);
    }

    Ok(sections.remove(0))
}
