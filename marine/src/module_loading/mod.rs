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

use crate::MarineError;
use crate::MarineResult;

use std::collections::HashMap;
use std::path::PathBuf;

/// Loads modules from a directory at a given path. Non-recursive, ignores subdirectories.
pub(crate) fn load_modules_from_fs(
    modules: &HashMap<String, PathBuf>,
) -> MarineResult<HashMap<String, Vec<u8>>> {
    let loaded = modules
        .iter()
        .try_fold(HashMap::new(), |mut hash_map, (import_name, path)| {
            let module_bytes = std::fs::read(path).map_err(|e| {
                MarineError::IOError(format!("failed to load {}: {}", path.display(), e))
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
