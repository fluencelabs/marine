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

use crate::MarineError;
use crate::MarineResult;

use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::private::PathAsDisplay;

/// Loads modules from a directory at a given path. Non-recursive, ignores subdirectories.
pub(crate) fn load_modules_from_fs(
    modules: &HashMap<String, PathBuf>,
) -> MarineResult<HashMap<String, Vec<u8>>> {
    let loaded = modules
        .iter()
        .try_fold(HashMap::new(), |mut hash_map, (import_name, path)| {
            let module_bytes = std::fs::read(&path).map_err(|e| {
                MarineError::IOError(format!("failed to load {}: {}", path.as_display(), e))
            })?;

            if hash_map.insert(import_name.clone(), module_bytes).is_some() {
                return Err(MarineError::InvalidConfig(String::from(
                    "module {} is duplicated in config",
                )));
            }

            Ok(hash_map)
        })?;

    Ok(loaded)
}
