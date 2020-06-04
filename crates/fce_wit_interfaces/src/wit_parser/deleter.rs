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

use super::errors::WITParserError;
use super::custom::WIT_SECTION_NAME;

use walrus::ModuleConfig;

use std::path::PathBuf;

pub fn delete_wit_section(
    in_wasm_path: PathBuf,
    out_wasm_path: PathBuf,
) -> Result<(), WITParserError> {
    let mut module = ModuleConfig::new()
        .parse_file(&in_wasm_path)
        .map_err(WITParserError::CorruptedWasmFile)?;

    let wit_section_ids = module
        .customs
        .iter()
        .filter_map(|(id, section)| {
            if section.name() == WIT_SECTION_NAME {
                Some(id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for id in wit_section_ids {
        module.customs.delete(id);
    }

    module
        .emit_wasm_file(&out_wasm_path)
        .map_err(WITParserError::WasmEmitError)?;

    Ok(())
}
