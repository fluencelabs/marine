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

use std::error::Error;

#[derive(Debug)]
pub enum MITInterfacesError {
    /// IT doesn't contain such type.
    NoSuchType(u32),

    /// IT doesn't contain such export.
    NoSuchExport(u32),

    /// IT doesn't contain such import.
    NoSuchImport(u32),

    /// IT doesn't contain such import.
    NoSuchAdapter(u32),
}

impl Error for MITInterfacesError {}

impl std::fmt::Display for MITInterfacesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MITInterfacesError::NoSuchType(type_id) => write!(
                f,
                "Loaded module doesn't contain type with idx = {}",
                type_id
            ),
            MITInterfacesError::NoSuchExport(export_type_id) => write!(
                f,
                "Loaded module doesn't contain export with type idx = {}",
                export_type_id
            ),
            MITInterfacesError::NoSuchImport(import_type_id) => write!(
                f,
                "Loaded module doesn't contain import with type idx = {}",
                import_type_id
            ),
            MITInterfacesError::NoSuchAdapter(adapter_type_id) => write!(
                f,
                "Loaded module doesn't contain adapter with type idx = {}",
                adapter_type_id
            ),
        }
    }
}
