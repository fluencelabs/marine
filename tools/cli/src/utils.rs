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

use std::fmt::Formatter;
use std::io::ErrorKind;
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
    let process = command
        .stdout(stdout_config)
        .spawn()
        .map_err(|e| process_command_run_error(e, &command))?;

    let output = process.wait_with_output()?;
    if !output.status.success() {
        anyhow::bail!(
            r#"command `{}` exited with {}"#,
            PrintCommand(&command),
            output.status
        )
    }

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    Ok(stdout)
}

struct PrintCommand<'c>(&'c Command);

impl<'c> std::fmt::Display for PrintCommand<'c> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.get_program().to_string_lossy())?;
        for arg in self.0.get_args() {
            write!(f, " {}", arg.to_string_lossy())?;
        }

        Ok(())
    }
}

fn process_command_run_error(e: std::io::Error, command: &Command) -> anyhow::Error {
    if e.kind() == ErrorKind::NotFound {
        anyhow::anyhow!(
            r#"cannot run `{}`: executable not found in $PATH"#,
            command.get_program().to_string_lossy(),
        )
    } else {
        anyhow::anyhow!(
            r#"cannot run `{}`: {}"#,
            command.get_program().to_string_lossy(),
            e
        )
    }
}
