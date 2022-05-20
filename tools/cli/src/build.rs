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

use crate::CLIResult;
use crate::errors::CLIError;

use std::process::Command;

const RUST_SDK_VERSION: &str = "0.6.0";

#[derive(serde::Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
enum DiagnosticMessage {
    BuildScriptExecuted,
    BuildFinished,
    CompilerArtifact { filenames: Vec<String> },
    RunWithArgs,
}

pub(crate) fn build(trailing_args: Vec<&str>) -> CLIResult<()> {
    use std::str::FromStr;

    let mut cargo = Command::new("cargo");
    cargo.arg("build").arg("--target").arg("wasm32-wasi");
    cargo.arg("--message-format").arg("json-render-diagnostics");
    cargo.args(trailing_args);

    // the piped mode is used here because cargo build prints all necessary for user
    // compilation progress into stderr, while stdout is used for deep compiler-specific messages
    // that DiagnosticMessage represents
    let output = crate::utils::run_command_piped(cargo)
        .map_err(|e| CLIError::WasmCompilationError(e.to_string()))?;
    let mut wasms: Vec<String> = Vec::new();
    for line in output.lines() {
        if let Ok(DiagnosticMessage::CompilerArtifact { filenames }) = serde_json::from_str(line) {
            wasms.extend(
                filenames
                    .into_iter()
                    .filter(|name| name.ends_with(".wasm"))
                    .collect::<Vec<_>>(),
            )
        }
    }

    if wasms.is_empty() {
        // it is possible to build a object file without Wasm artifacts
        return Ok(());
    }

    let version = semver::Version::from_str(RUST_SDK_VERSION).unwrap();
    for wasm in wasms {
        let wasm_path = std::path::PathBuf::from(wasm);
        marine_it_generator::embed_it(&wasm_path)?;
        marine_module_info_parser::sdk_version::embed_from_path(&wasm_path, &wasm_path, &version)?;
    }

    Ok(())
}
