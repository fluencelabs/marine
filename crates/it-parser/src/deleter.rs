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

use super::errors::ITParserError;
use super::custom::IT_SECTION_NAME;

use walrus::ModuleConfig;

use std::path::PathBuf;

/// Delete all custom sections with IT from provided Wasm file.
pub fn delete_it_section_from_file(
    in_wasm_path: PathBuf,
    out_wasm_path: PathBuf,
) -> Result<(), ITParserError> {
    let module = ModuleConfig::new()
        .parse_file(in_wasm_path)
        .map_err(ITParserError::CorruptedWasmFile)?;

    let mut module = delete_it_section(module);

    module
        .emit_wasm_file(&out_wasm_path)
        .map_err(ITParserError::WasmEmitError)?;

    Ok(())
}

/// Delete all custom sections with IT from provided Wasm module.
pub fn delete_it_section(mut wasm_module: walrus::Module) -> walrus::Module {
    let wit_section_ids = wasm_module
        .customs
        .iter()
        .filter_map(|(id, section)| {
            if section.name() == IT_SECTION_NAME {
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
