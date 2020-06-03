use crate::custom::WITCustom;

use walrus::ModuleConfig;
use wasmer_wit::{
    decoders::wat::{parse, Buffer},
    encoders::binary::ToBytes,
};

use std::path::PathBuf;

pub struct Config {
    pub in_wasm_path: PathBuf,
    pub wit: String,
    pub out_wasm_path: PathBuf,
}

pub fn embed_wit(options: &Config) -> Result<(), String> {
    let mut module = ModuleConfig::new()
        .parse_file(&options.in_wasm_path)
        .map_err(|e| format!("Failed to parse the Wasm module: {}", e))?;

    let buffer = Buffer::new(&options.wit)
        .map_err(|e| format!("Can't parse provided Wasm module: {}", e))?;
    let ast = parse(&buffer).map_err(|e| format!("Failed to parse the WIT description: {}", e))?;

    let mut bytes = vec![];
    ast.to_bytes(&mut bytes)
        .map_err(|_| "Failed to encode the AST into its binary representation.")?;

    let custom = WITCustom(bytes);
    module.customs.add(custom);
    module
        .emit_wasm_file(&options.out_wasm_path)
        .map_err(|e| format!("Failed to emit Wasm file with bindings: {}", e))?;

    Ok(())
}
