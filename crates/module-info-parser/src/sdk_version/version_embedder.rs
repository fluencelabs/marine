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

use crate::ModuleInfoResult;
use crate::ModuleInfoError;
use fluence_sdk_main::VERSION_SECTION_NAME;

use walrus::ModuleConfig;
use walrus::CustomSection;
use walrus::IdsToIndices;

use std::path::Path;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub(super) struct VersionCustomSection(Vec<u8>);

impl CustomSection for VersionCustomSection {
    fn name(&self) -> &str {
        VERSION_SECTION_NAME
    }

    fn data(&self, _ids_to_indices: &IdsToIndices) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.0)
    }
}

/// Embed provided WIT to a Wasm module.
pub fn embed_by_module(
    mut wasm_module: walrus::Module,
    version: String,
) -> walrus::Module {
    let custom = VersionCustomSection(version.into_bytes());
    wasm_module.customs.add(custom);

    wasm_module
}

pub fn embed_by_path<I, O>(
    in_wasm_module_path: I,
    out_wasm_module_path: O,
    version: String,
) -> ModuleInfoResult<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let wasm_module = ModuleConfig::new()
        .parse_file(in_wasm_module_path)
        .map_err(ModuleInfoError::CorruptedWasmFile)?;

    let mut wasm_module = embed_by_module(wasm_module, version);
    wasm_module
        .emit_wasm_file(out_wasm_module_path)
        .map_err(ModuleInfoError::WasmEmitError)
}
