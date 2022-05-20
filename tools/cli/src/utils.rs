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

use std::io::Read;
use std::process::Command;
use std::process::Stdio;

/// Run the provided command and returns its stdout.
pub(crate) fn run_command(mut command: Command) -> Result<String, anyhow::Error> {
    let mut process = command.stdin(Stdio::inherit()).spawn()?;

    let status = process.wait()?;
    if !status.success() {
        anyhow::bail!("failed to execute, exited with {}", status,)
    }

    let mut output = String::new();
    process
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("failed to read the stdout"))?
        .read_to_string(&mut output)?;
    Ok(output)
}
