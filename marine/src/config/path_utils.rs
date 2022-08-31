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

use crate::MarineError;
use crate::MarineResult;

use std::path::Path;
use std::path::PathBuf;

pub fn as_relative_to_base(base: Option<&PathBuf>, path: &Path) -> MarineResult<PathBuf> {
    if path.is_absolute() {
        return Ok(PathBuf::from(path));
    }

    let path = match base {
        None => PathBuf::from(path),
        Some(base) => base.join(path),
    };

    path.canonicalize().map_err(|e| {
        MarineError::IOError(format!(
            "Failed to canonicalize path {}: {}",
            path.display(),
            e
        ))
    })
}
