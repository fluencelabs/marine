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
