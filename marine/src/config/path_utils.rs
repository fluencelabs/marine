/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::MarineError;
use crate::MarineResult;

use std::path::Path;
use std::path::PathBuf;

pub fn as_relative_to_base(base: Option<&Path>, path: &Path) -> MarineResult<PathBuf> {
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
