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

use super::custom::WITCustom;
use super::errors::WITParserError;

use walrus::ModuleConfig;
use wasmer_wit::{
    decoders::wat::{parse, Buffer},
    encoders::binary::ToBytes,
};

use std::path::PathBuf;

pub fn embed_text_wit(
    in_wasm_path: PathBuf,
    out_wasm_path: PathBuf,
    wit: &str,
) -> Result<(), WITParserError> {
    let mut module = ModuleConfig::new()
        .parse_file(&in_wasm_path)
        .map_err(WITParserError::CorruptedWasmFile)?;

    let buffer = Buffer::new(wit)?;
    let ast = parse(&buffer)?;

    let mut bytes = vec![];
    ast.to_bytes(&mut bytes)?;

    let custom = WITCustom(bytes);
    module.customs.add(custom);
    module
        .emit_wasm_file(&out_wasm_path)
        .map_err(WITParserError::WasmEmitError)?;

    Ok(())
}
