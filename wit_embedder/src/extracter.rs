use std::path::PathBuf;
use walrus::{IdsToIndices, ModuleConfig};

pub fn extract_wit(wasm_file: PathBuf) -> Result<String, String> {
    let module = ModuleConfig::new()
        .parse_file(wasm_file)
        .map_err(|_| "Failed to parse the Wasm module.".to_string())?;

    let sections = module
        .customs
        .iter()
        .filter(|(_, section)| section.name() == "interface-types")
        .collect::<Vec<_>>();

    if sections.is_empty() {
        return Err("Wasm binary doesn't contain interface types section".to_string());
    }
    if sections.len() > 1 {
        return Err("Wasm binary contains more than one interface-types section".to_string());
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
