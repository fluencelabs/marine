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
use crate::cargo_manifest::ManifestError;

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

            if new_wasms.is_empty() {
                continue;
            }

            use ManifestError::*;
            match extract_sdk_version(&manifest_path) {
                // info
                Ok(sdk_version) => {
                    println!(
                        "Found sdk version {sdk_version} in {}:",
                        manifest_path.display()
                    );
                    println!("Applying to wasm artifacts:");
                    for wasm_path in &new_wasms {
                        println!("{wasm_path}");
                    }

                    wasms.extend(
                        new_wasms
                            .into_iter()
                            .map(|name| (name, sdk_version.clone())),
                    )
                }
                Err(e) => {
                    let manifest_path = manifest_path.display();
                    match e {
                        NoSdkDependencyError => println!("No sdk version found in {manifest_path}"),
                        CannotProcessManifest(message) => println!("WARNING: cannot process {manifest_path}: {message}"),
                        ParseError(e) => println!("WARNING: cannot parse {manifest_path}: {e}"),
                        Io(e) => println!("WARNING: cannot load {manifest_path}: {e}"),
                        NoSdkVersionError => println!("ERROR: found SDK dependency in {manifest_path}, but the version is not specified."),
                        VersionParseError(e) => println!("ERROR: found SDK dependency in {manifest_path}, but encoundeted an error while parsing its version: {e}"),
                        InheritedDependencyUnsupported => println!("ERROR: found SDK dependency in {manifest_path}: unsupported inherited dependency.")
                    }

                    println!("Skipping wasm artifacts:");
                    for wasm_path in &new_wasms {
                        println!("{wasm_path}");
                    }
                }
            }
        }
    }

    if wasms.is_empty() {
        println!("Did not find any marine wasm artifacts.");
        // it is possible to build a object file without Wasm artifacts
        return Ok(());
    }

    for (wasm, sdk_version) in wasms {
        let wasm_path = std::path::PathBuf::from(wasm);
        print!(
            "Embedding IT section and SDK version into {}:",
            wasm_path.display()
        );

        marine_it_generator::embed_it(&wasm_path)?;
        marine_module_info_parser::sdk_version::embed_from_path(
            &wasm_path,
            &wasm_path,
            &sdk_version,
        )?;
        println!(" success");
    }

    Ok(())
}
