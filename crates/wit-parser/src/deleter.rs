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

/// Delete all custom sections with WIT from provided Wasm file.
pub fn delete_wit_section_from_file(in_wasm_path: PathBuf, out_wasm_path: PathBuf) -> Result<(), WITParserError> {
    let module = ModuleConfig::new()
        .parse_file(&in_wasm_path)
        .map_err(WITParserError::CorruptedWasmFile)?;

    let mut module = delete_wit_section(module);

    module
        .emit_wasm_file(&out_wasm_path)
        .map_err(WITParserError::WasmEmitError)?;

    Ok(())
}

/// Delete all custom sections with WIT from provided Wasm module.
pub fn delete_wit_section(mut wasm_module: walrus::Module) -> walrus::Module {
    let wit_section_ids = wasm_module
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
        wasm_module.customs.delete(id);
    }

    wasm_module
}
