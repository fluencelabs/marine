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

use super::custom::ITCustomSection;
use super::errors::ITParserError;
use crate::ParserResult;

use walrus::ModuleConfig;
use wasmer_it::ast::Interfaces;
use wasmer_it::decoders::wat::parse;
use wasmer_it::decoders::wat::Buffer;
use wasmer_it::ToBytes;

use std::path::Path;

/// Embed provided IT to a Wasm file by path.
pub fn embed_text_it<I, O>(in_wasm_path: I, out_wasm_path: O, it: &str) -> ParserResult<()>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let module = ModuleConfig::new()
        .parse_file(in_wasm_path)
        .map_err(ITParserError::CorruptedWasmFile)?;

    let buffer = Buffer::new(it)?;
    let ast = parse(&buffer)?;

    let mut module = embed_it(module, &ast);
    module
        .emit_wasm_file(out_wasm_path)
        .map_err(ITParserError::WasmEmitError)?;

    Ok(())
}

/// Embed provided IT to a Wasm module.
pub fn embed_it(mut wasm_module: walrus::Module, interfaces: &Interfaces<'_>) -> walrus::Module {
    let mut bytes = vec![];
    // TODO: think about possible errors here
    interfaces.to_bytes(&mut bytes).unwrap();

    let custom = ITCustomSection(bytes);
    wasm_module.customs.add(custom);

    wasm_module
}
