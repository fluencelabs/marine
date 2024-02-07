/*
 * Copyright 2024 Fluence Labs Limited
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

use crate::ModuleInfoResult;
use crate::ModuleInfoError;

use walrus::ModuleConfig;
use walrus::Module;

use std::path::Path;

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
