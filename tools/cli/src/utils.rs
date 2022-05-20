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
use std::process::Stdio;

/// Run the provided command in the inherited mode (with printing output to stdout).
pub(crate) fn run_command_inherited(command: Command) -> Result<String, anyhow::Error> {
    run_command(command, Stdio::inherit())
}

/// Run the provided command in the piped mode (without printing output to stdout).
pub(crate) fn run_command_piped(command: Command) -> Result<String, anyhow::Error> {
    run_command(command, Stdio::piped())
}

/// Run the provided command and returns its stdout as a string.
fn run_command<T: Into<Stdio>>(
    mut command: Command,
    stdout_config: T,
) -> Result<String, anyhow::Error> {
    let process = command.stdout(stdout_config).spawn()?;

    let output = process.wait_with_output()?;
    if !output.status.success() {
        anyhow::bail!("failed to execute, exited with {}", output.status)
    }

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    Ok(stdout)
}
