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
