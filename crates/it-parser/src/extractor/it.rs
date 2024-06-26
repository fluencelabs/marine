/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
