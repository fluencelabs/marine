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

mod modules_load_strategy;

pub(crate) use modules_load_strategy::ModulesLoadStrategy;

use crate::FaaSError;
use crate::Result;

use std::collections::HashMap;
use std::path::Path;

/// Loads modules from a directory at a given path. Non-recursive, ignores subdirectories.
pub(crate) fn load_modules_from_fs(
    modules_dir: &Path,
    modules: ModulesLoadStrategy<'_>,
) -> Result<HashMap<String, Vec<u8>>> {
    use FaaSError::IOError;

    let mut dir_entries =
        std::fs::read_dir(modules_dir).map_err(|e| IOError(format!("{:?}: {}", modules_dir, e)))?;

    let loaded = dir_entries.try_fold(HashMap::new(), |mut hash_map, entry| {
        let entry = entry?;
        let path = entry.path();
        // Skip directories
        if path.is_dir() {
            return Ok(hash_map);
        }

        let file_name = Path::new(
            path.file_name()
                .ok_or_else(|| IOError(format!("No file name in path {:?}", path)))?,
        );

        if modules.should_load(&file_name) {
            let module_bytes = std::fs::read(&path)?;
            let module_name = modules.extract_module_name(&path)?;
            if hash_map.insert(module_name, module_bytes).is_some() {
                return Err(FaaSError::InvalidConfig(String::from(
                    "module {} is duplicated in config",
                )));
            }
        }

        Ok(hash_map)
    })?;

    if modules.required_modules_len() > loaded.len() {
        let loaded = loaded.iter().map(|(n, _)| n);
        let not_found = modules.missing_modules(loaded);
        return Err(FaaSError::InvalidConfig(format!(
            "the following modules were not found: {:?}",
            not_found
        )));
    }

    Ok(loaded)
}
