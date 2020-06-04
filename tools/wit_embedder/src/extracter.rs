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

use crate::custom::WIT_SECTION_NAME;

use walrus::{IdsToIndices, ModuleConfig};

pub fn extract_wit(wasm_file: std::path::PathBuf) -> Result<String, String> {
    let module = ModuleConfig::new()
        .parse_file(wasm_file)
        .map_err(|_| "Failed to parse the Wasm module.".to_string())?;

    let sections = module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == WIT_SECTION_NAME)
        .collect::<Vec<_>>();

    if sections.is_empty() {
        return Err(format!(
            "Wasm binary doesn't contain {} section",
            WIT_SECTION_NAME
        ));
    }
    if sections.len() > 1 {
        return Err(format!(
            "Wasm binary contains more than one {} section",
            WIT_SECTION_NAME
        ));
    }

    let default_ids = IdsToIndices::default();
    let wit_section_bytes = sections[0].1.data(&default_ids).into_owned();
    let wit = match wasmer_wit::decoders::binary::parse::<()>(&wit_section_bytes) {
        Ok((remainder, wit)) if remainder.is_empty() => wit,
        Ok((remainder, _)) => {
            return Err(format!("remainder isn't empty: {:?}", remainder));
        }
        Err(e) => {
            return Err(format!("An error occurred while parsing: {}", e));
        }
    };

    Ok((&wit).to_string())
}
