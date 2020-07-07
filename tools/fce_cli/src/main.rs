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
#![warn(rust_2018_idioms)]
#![deny(
    // dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod args;

use args::*;

use clap::{App, AppSettings};
// use std::path::PathBuf;
use std::process::Command;

#[derive(serde::Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
enum DiagnosticMessage {
    BuildScriptExecuted,
    BuildFinished,
    CompilerArtifact {
        executable: String,
        /*
               filenames: Vec<String>,
               profile: Profile,
               fresh: bool,
        */
    },
    RunWithArgs {
        //        args: Vec<String>,
    },
}

pub fn main() -> Result<(), exitfailure::ExitFailure> {
    let app = App::new("CLI tool for embedding WIT to provided Wasm file")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(build())
        .subcommand(validate());

    match app.get_matches().subcommand() {
        ("build", Some(arg)) => {
            use std::io::Read;

            let wasm_path = arg.value_of(IN_WASM_PATH).unwrap();
            // wit_embedder::delete_wit_section(wasm_path.clone(), wasm_path.clone());

            let mut cargo = Command::new("cargo");
            cargo.arg("build").arg("--target").arg("wasm32-wasi");
            cargo.arg("--message-format").arg("json-render-diagnostics");
            cargo.arg("--manifest-path").arg(wasm_path);

            let mut process = cargo.stdout(std::process::Stdio::piped()).spawn().unwrap();

            let mut output = String::new();

            process
                .stdout
                .take()
                .unwrap()
                .read_to_string(&mut output)
                .unwrap();
            let _status = process.wait().unwrap();

            let mut wasms = Vec::new();
            for line in output.lines() {
                match serde_json::from_str(line) {
                    Ok(DiagnosticMessage::CompilerArtifact { executable }) => {
                        wasms.push(executable)
                    }
                    _ => {}
                }
            }

            let wasm_path = std::path::PathBuf::from(wasms.first().unwrap());
            wit::embed_wit(wasm_path);

            Ok(())
        }
        ("validate", Some(arg)) => {
            let _wasm_path = arg.value_of(IN_WASM_PATH).unwrap();

            Ok(())
        }
        c => Err(failure::err_msg(format!("Unexpected command: {}", c.0)).into()),
    }
}
