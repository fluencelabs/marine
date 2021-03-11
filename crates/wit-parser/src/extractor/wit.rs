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

use crate::custom::IT_SECTION_NAME;
use crate::errors::WITParserError;
use crate::Result;

use walrus::{IdsToIndices, ModuleConfig};
use wasmer_wit::ast::Interfaces;
use wasmer_core::Module as WasmerModule;

use std::path::Path;

/// Extracts WIT section of provided Wasm binary and converts it to a string.
pub fn extract_text_wit(wasm_file_path: &Path) -> Result<String> {
    let wit_section_bytes = extract_custom_section(&wasm_file_path)?;
    let wit = extract_wit_from_bytes(&wit_section_bytes)?;
    Ok((&wit).to_string())
}

/// Extracts WIT section of provided Wasm binary and converts it to a FCEWITInterfaces.
pub fn extract_wit(wasmer_module: &WasmerModule) -> Result<Interfaces<'_>> {
    let wit_sections = wasmer_module
        .custom_sections(IT_SECTION_NAME)
        .ok_or(WITParserError::NoITSection)?;

    if wit_sections.len() > 1 {
        return Err(WITParserError::MultipleITSections);
    }

    extract_wit_from_bytes(&wit_sections[0])
}

pub(crate) fn extract_wit_from_bytes(
    wit_section_bytes: &[u8],
) -> Result<Interfaces<'_>> {
    match wasmer_wit::decoders::binary::parse::<()>(wit_section_bytes) {
        Ok((remainder, wit)) if remainder.is_empty() => Ok(wit),
        Ok(_) => Err(WITParserError::ITRemainderNotEmpty),
        Err(_) => Err(WITParserError::CorruptedITSection),
    }
}

pub(crate) fn extract_custom_section(wasm_file_path: &Path) -> Result<Vec<u8>> {
    let module = ModuleConfig::new()
        .parse_file(wasm_file_path)
        .map_err(WITParserError::CorruptedWasmFile)?;

    let sections = module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == IT_SECTION_NAME)
        .collect::<Vec<_>>();

    if sections.is_empty() {
        return Err(WITParserError::NoITSection);
    }
    if sections.len() > 1 {
        return Err(WITParserError::MultipleITSections);
    }

    let default_ids = IdsToIndices::default();
    Ok(sections[0].1.data(&default_ids).into_owned())
}
