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
use crate::errors::ITParserError;
use crate::ParserResult;

use walrus::IdsToIndices;
use wasmer_it::ast::Interfaces;
use marine_wasm_backend_traits::WasmBackend;
use marine_wasm_backend_traits::Module as WasmModule;

use std::borrow::Cow;
use std::path::Path;

/// Extracts IT section of provided Wasm binary and converts it to a string.
pub fn extract_text_it<P>(wasm_file_path: P) -> ParserResult<String>
where
    P: AsRef<Path>,
{
    let module = walrus::ModuleConfig::new()
        .parse_file(wasm_file_path)
        .map_err(ITParserError::CorruptedWasmFile)?;

    let raw_custom_section = extract_custom_section(&module)?;
    let wit_section_bytes = raw_custom_section.as_ref();
    let it = extract_it_from_bytes(wit_section_bytes)?;

    Ok((&it).to_string())
}

/// Extracts IT section of provided Wasm binary and converts it to a MITInterfaces.
pub fn extract_it_from_module<WB: WasmBackend>(
    wasm_module: &<WB as WasmBackend>::Module,
) -> ParserResult<Interfaces<'_>> {
    let wit_sections = wasm_module.custom_sections(IT_SECTION_NAME);

    let it_section = match wit_sections.len() {
        0 => Err(ITParserError::NoITSection),
        1 => Ok(&wit_sections[0]),
        _ => Err(ITParserError::MultipleITSections),
    }?;

    extract_it_from_bytes(it_section)
}

pub fn extract_version_from_module(module: &walrus::Module) -> ParserResult<semver::Version> {
    let raw_custom_section = extract_custom_section(module)?;
    let wit_section_bytes = raw_custom_section.as_ref();
    let it = extract_it_from_bytes(wit_section_bytes)?;

    Ok(it.version)
}

pub(crate) fn extract_it_from_bytes(wit_section_bytes: &[u8]) -> ParserResult<Interfaces<'_>> {
    match wasmer_it::decoders::binary::parse::<(&[u8], nom::error::ErrorKind)>(wit_section_bytes) {
        Ok((remainder, it)) if remainder.is_empty() => Ok(it),
        Ok(_) => Err(ITParserError::ITRemainderNotEmpty),
        Err(e) => Err(ITParserError::CorruptedITSection(e.to_owned())),
    }
}

pub(crate) fn extract_custom_section(module: &walrus::Module) -> ParserResult<Cow<'_, [u8]>> {
    let sections = module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == IT_SECTION_NAME)
        .collect::<Vec<_>>();

    if sections.is_empty() {
        return Err(ITParserError::NoITSection);
    }
    if sections.len() > 1 {
        return Err(ITParserError::MultipleITSections);
    }

    let default_ids = IdsToIndices::default();
    Ok(sections[0].1.data(&default_ids))
}
