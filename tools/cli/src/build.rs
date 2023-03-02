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
use crate::cargo_manifest::ManifestError;

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
    env_logger::init();

    let mut cargo = Command::new("cargo");
    cargo.arg("build").arg("--target").arg("wasm32-wasi");
    cargo.arg("--message-format").arg("json-render-diagnostics");
    cargo.args(trailing_args.iter());

    // the piped mode is used here because cargo build prints all necessary for user
    // compilation progress into stderr, while stdout is used for deep compiler-specific messages
    // that DiagnosticMessage represents
    let output = crate::utils::run_command_piped(cargo)
        .map_err(|e| CLIError::WasmCompilationError(e.to_string()))?;

    let metadata = get_cargo_metadata(&trailing_args)?;
    let lockfile = load_lockfile(metadata.workspace_root)?;

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
                match extract_sdk_version(&manifest_path, &lockfile) {
                    Ok(sdk_version) => wasms.extend(
                        new_wasms
                            .into_iter()
                            .map(|name| (name, sdk_version.clone())),
                    ),
                    Err(ManifestError::NonMarineWasm) => {
                        log::info!("Skipping {} as non-marine wasm", manifest_path.display())
                    }
                    Err(e) => log::error!("marine-rs-sdk version extraction failed: {}", e),
                }
            }
        }
    }

    if wasms.is_empty() {
        // it is possible to build a object file without Wasm artifacts
        return Ok(());
    }

    for (wasm, sdk_version) in wasms {
        let wasm_path = std::path::PathBuf::from(wasm);
        marine_it_generator::embed_it(&wasm_path)?;
        marine_module_info_parser::sdk_version::embed_from_path(
            &wasm_path,
            &wasm_path,
            &sdk_version,
        )?;
    }

    Ok(())
}

fn load_lockfile(workspace_root: impl Into<PathBuf>) -> CLIResult<cargo_lock::Lockfile> {
    let lockfile_path = workspace_root.into().join("Cargo.lock");

    cargo_lock::Lockfile::load(&lockfile_path)
        .map_err(|e| CLIError::LockfileError(lockfile_path, e))
}

fn get_cargo_metadata(args: &[&str]) -> CLIResult<cargo_metadata::Metadata> {
    let mut args = args
        .iter()
        .skip_while(|arg| !arg.starts_with("--manifest-path"));

    let mut cmd = cargo_metadata::MetadataCommand::new();
    match args.next() {
        Some(p) if *p == "--manifest-path" => {
            if let Some(path) = args.next() {
                cmd.manifest_path(&path);
            }
        }
        Some(p) => {
            cmd.manifest_path(p.trim_start_matches("--manifest-path="));
        }
        None => {}
    };

    cmd.exec().map_err(Into::into)
}
