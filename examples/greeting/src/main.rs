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

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

use std::fs;
use std::env;
use std::path::PathBuf;

module_manifest!();

pub fn main() {}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

#[marine]
pub fn ls_current() -> String {
    match traverse_current() {
        Ok(_) => "Success".to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

#[marine]
pub fn ls(dir: String) -> String {
    match traverse_any(dir.into()) {
        Ok(_) => "Success".to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

fn traverse_current() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    traverse_any(current_dir)
}

fn traverse_any(path: PathBuf) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        println!(
            "{}, is_readonly: {}",
            path.display(),
            metadata.permissions().readonly()
        )
    }

    Ok(())
}
