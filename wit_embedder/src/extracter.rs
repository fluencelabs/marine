use crate::custom::WIT_SECTION_NAME;
use std::path::PathBuf;
use walrus::{IdsToIndices, ModuleConfig};

pub fn extract_wit(wasm_file: PathBuf) -> Result<String, String> {
    let module = ModuleConfig::new()
        .parse_file(wasm_file)
        .map_err(|_| "Failed to parse the Wasm module.".to_string())?;

    let sections = module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == WIT_SECTION_NAME)
        .collect::<Vec<_>>();

    if sections.is_empty() {
        return Err(format!("Wasm binary doesn't contain {} section", WIT_SECTION_NAME));
    }
    if sections.len() > 1 {
        return Err(format!("Wasm binary contains more than one {} section", WIT_SECTION_NAME));
    }

    let default_ids = IdsToIndices::default();
    let wit_section_bytes = sections[0].1.data(&default_ids).into_owned();
    let wit = match wasmer_interface_types::decoders::binary::parse::<()>(&wit_section_bytes) {
        Ok((remainder, wit)) if remainder.is_empty() => wit,
        Ok((remainder, _)) => {
            return Err(format!("remainder isn't empty: {:?}", remainder));
        }
        Err(e) => {
            return Err(format!("An error occurred while parsing: {}", e));
        }
    };

    Ok(format!("{:?}", wit))
}
