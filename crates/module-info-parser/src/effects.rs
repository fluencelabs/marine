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

use crate::ModuleInfoResult;
use crate::ModuleInfoError;

use walrus::ModuleConfig;
use walrus::Module;

use std::path::Path;

// TODO: create a common place for these consts to use in both marine and marine-rs-sdk to use in both marine and marine-rs-sdk
const HOST_IMPORT_NAMESPACE_V0: &str = "host";
const HOST_IMPORT_NAMESPACE_PREFIX: &str = "__marine_host_api_v";
const LOGGER_IMPORT_NAME: &str = "log_utf8_string";
const CALL_PARAMETERS_IMPORT_NAME: &str = "get_call_parameters";

#[derive(Debug)]
pub enum WasmEffect {
    Logger,
    MountedBinary(String),
}

pub fn extract_from_path<P>(wasm_module_path: P) -> ModuleInfoResult<Vec<WasmEffect>>
where
    P: AsRef<Path>,
{
    let module = ModuleConfig::new()
        .parse_file(wasm_module_path)
        .map_err(ModuleInfoError::CorruptedWasmFile)?;

    extract_from_module(&module)
}

pub fn extract_from_bytes(wasm_module_bytes: &[u8]) -> ModuleInfoResult<Vec<WasmEffect>> {
    let module = ModuleConfig::new()
        .parse(wasm_module_bytes)
        .map_err(ModuleInfoError::CorruptedWasmFile)?;

    extract_from_module(&module)
}

pub fn extract_from_module(wasm_module: &Module) -> ModuleInfoResult<Vec<WasmEffect>> {
    let effects = wasm_module
        .imports
        .iter()
        .filter_map(|import| inspect_import(&import.module, &import.name))
        .collect();

    Ok(effects)
}

fn inspect_import(module: &str, name: &str) -> Option<WasmEffect> {
    if !is_host_import(module) {
        return None;
    }

    match name {
        LOGGER_IMPORT_NAME => Some(WasmEffect::Logger),
        CALL_PARAMETERS_IMPORT_NAME => None,
        name => Some(WasmEffect::MountedBinary(name.to_string())),
    }
}

fn is_host_import(namespace: &str) -> bool {
    namespace == HOST_IMPORT_NAMESPACE_V0 || namespace.starts_with(HOST_IMPORT_NAMESPACE_PREFIX)
}
