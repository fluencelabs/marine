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

use super::custom::WIT_SECTION_NAME;
use super::errors::WITParserError;
use crate::fce_wit_interfaces::FCEWITInterfaces;

use walrus::{IdsToIndices, ModuleConfig};
use wasmer_wit::ast::Interfaces;

use std::path::PathBuf;

pub fn extract_text_wit(wasm_file_path: PathBuf) -> Result<String, WITParserError> {
    extract_wit_with_fn(
        wasm_file_path,
        |wit: Interfaces<'_>| -> Result<String, WITParserError> { Ok((&wit).to_string()) },
    )
}

pub fn extract_fce_wit(wasm_file_path: PathBuf) -> Result<FCEWITInterfaces, WITParserError> {
    extract_wit_with_fn(
        wasm_file_path,
        |wit: Interfaces<'_>| -> Result<FCEWITInterfaces, WITParserError> {
            Ok(FCEWITInterfaces::new(wit))
        },
    )
}

fn extract_wit_with_fn<F, FResultType>(
    wasm_file_path: PathBuf,
    func: F,
) -> Result<FResultType, WITParserError>
where
    F: FnOnce(Interfaces<'_>) -> Result<FResultType, WITParserError>,
{
    let wit_section_bytes = extract_wit_section_bytes(wasm_file_path)?;
    let raw_wit = extract_raw_interfaces(&wit_section_bytes)?;

    func(raw_wit)
}

fn extract_raw_interfaces(wit_section_bytes: &[u8]) -> Result<Interfaces<'_>, WITParserError> {
    let wit = match wasmer_wit::decoders::binary::parse::<()>(&wit_section_bytes) {
        Ok((remainder, wit)) if remainder.is_empty() => wit,
        Ok(_) => {
            return Err(WITParserError::WITRemainderNotEmpty);
        }
        Err(_) => {
            return Err(WITParserError::CorruptedWITSection);
        }
    };

    Ok(wit)
}

fn extract_wit_section_bytes(wasm_file_path: PathBuf) -> Result<Vec<u8>, WITParserError> {
    let module = ModuleConfig::new()
        .parse_file(wasm_file_path)
        .map_err(WITParserError::CorruptedWasmFile)?;

    let sections = module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == WIT_SECTION_NAME)
        .collect::<Vec<_>>();

    if sections.is_empty() {
        return Err(WITParserError::NoWITSection);
    }
    if sections.len() > 1 {
        return Err(WITParserError::MultipleWITSections);
    }

    let default_ids = IdsToIndices::default();
    Ok(sections[0].1.data(&default_ids).into_owned())
}
