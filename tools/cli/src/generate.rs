/*
 * Copyright 2022 Fluence Labs Limited
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

use std::process::Command;

const MARINE_TEMPLATE_URL: &str = "https://github.com/fluencelabs/marine-template";

pub(crate) fn generate(
    project_name: Option<&str>,
    should_be_initialized: bool,
) -> Result<(), anyhow::Error> {
    if !is_cargo_generate_installed()? {
        println!(
            "to use generate `cargo-generate` should be installed:\ncargo install cargo-generate"
        );
        return Ok(());
    }

    let mut cargo = Command::new("cargo");
    cargo.arg("generate").arg("--git").arg(MARINE_TEMPLATE_URL);

    if let Some(project_name) = project_name {
        cargo.arg("--name").arg(project_name);
    }
    if should_be_initialized {
        cargo.arg("--init");
    }

    crate::utils::run_command_inherited(cargo).map(|_| ())
}

fn is_cargo_generate_installed() -> Result<bool, anyhow::Error> {
    let mut cargo = Command::new("cargo");
    cargo.arg("install").arg("--list");

    let output = crate::utils::run_command_piped(cargo)?;
    Ok(output.contains("cargo-generate"))
}
