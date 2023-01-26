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

use semver::Version;

use std::path::PathBuf;
use std::process::Command;

#[derive(serde::Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
enum DiagnosticMessage {
    BuildScriptExecuted,
    BuildFinished,
    CompilerArtifact {
        filenames: Vec<String>,
        manifest_path: PathBuf,
    },
    RunWithArgs,
}

pub(crate) fn build(trailing_args: Vec<&str>) -> CLIResult<()> {
    let mut cargo = Command::new("cargo");
    cargo.arg("build").arg("--target").arg("wasm32-wasi");
    cargo.arg("--message-format").arg("json-render-diagnostics");
    cargo.args(trailing_args);

    // the piped mode is used here because cargo build prints all necessary for user
    // compilation progress into stderr, while stdout is used for deep compiler-specific messages
    // that DiagnosticMessage represents
    let output = crate::utils::run_command_piped(cargo)
        .map_err(|e| CLIError::WasmCompilationError(e.to_string()))?;
    let mut wasms: Vec<(String, Version)> = Vec::new();
    for line in output.lines() {
        if let Ok(DiagnosticMessage::CompilerArtifact {
            filenames,
            manifest_path,
        }) = serde_json::from_str(line)
        {
            use crate::cargo_manifest::extract_sdk_version;

            let new_wasms = filenames
                .into_iter()
                .filter(|name| name.ends_with(".wasm"))
                .collect::<Vec<_>>();
            if !new_wasms.is_empty() {
                if let Ok(sdk_version) = extract_sdk_version(&manifest_path) {
                    wasms.extend(
                        new_wasms
                            .into_iter()
                            .map(|name| (name, sdk_version.clone())),
                    )
                } else {
                    println!("Failed to extract SDK version from manifest {}", manifest_path.display());
                    println!("Rejected some wasms:");
                    for wasm in &new_wasms {
                        println!("{}", wasm)
                    }

                    println!();
                }
            }
        }
    }

    if wasms.is_empty() {
        println!("Did not find any marine wasm files");
        // it is possible to build a object file without Wasm artifacts
        return Ok(());
    }

    println!("Marine wasm files:");
    for file in &wasms {
        println!("{}", file.0);
    }

    println!();

    for (wasm, sdk_version) in wasms {
        let wasm_path = std::path::PathBuf::from(wasm);
        marine_it_generator::embed_it(&wasm_path)
            .map(|val|{println!("embedded IT section into {}", wasm_path.display()); val})
            .map_err(|err| {println!("failed to embed IT section into {}", wasm_path.display()); err})?;

        marine_module_info_parser::sdk_version::embed_from_path(
            &wasm_path,
            &wasm_path,
            &sdk_version,
        )?;
    }

    Ok(())
}
