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

#[derive(serde::Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
enum DiagnosticMessage {
    BuildScriptExecuted,
    BuildFinished,
    CompilerArtifact { filenames: Vec<String> },
    RunWithArgs,
}

pub(crate) fn build(trailing_args: Vec<&str>) -> CLIResult<()> {
    use std::io::Read;

    let mut cargo = Command::new("cargo");
    cargo.arg("build").arg("--target").arg("wasm32-wasi");
    cargo.arg("--message-format").arg("json-render-diagnostics");
    cargo.args(trailing_args);

    let mut process = cargo.stdout(std::process::Stdio::piped()).spawn()?;

    let mut output = String::new();
    process
        .stdout
        .take()
        .ok_or_else(|| CLIError::WasmCompilationError("Compilation failed: no output".to_string()))?
        .read_to_string(&mut output)?;

    let status = process.wait()?;
    if !status.success() {
        return Err(CLIError::WasmCompilationError(format!(
            "Compilation failed with status {}",
            status
        )));
    }

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

    for wasm in wasms {
        let wasm_path = std::path::PathBuf::from(wasm);
        fce_wit_generator::embed_wit(wasm_path)?;
    }

    Ok(())
}
