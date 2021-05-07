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

use std::error::Error;

#[derive(Debug)]
pub enum MITInterfacesError {
    /// WIT doesn't contain such type.
    NoSuchType(u32),

    /// WIT doesn't contain such export.
    NoSuchExport(u32),

    /// WIT doesn't contain such import.
    NoSuchImport(u32),

    /// WIT doesn't contain such import.
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
