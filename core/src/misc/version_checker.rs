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

use super::PrepareResult;
use super::PrepareError;

use marine_module_info_parser::sdk_version;
use marine_min_it_version::min_sdk_version;
use marine_min_it_version::min_it_version;
use marine_wasm_backend_traits::WasmBackend;

pub(crate) fn check_sdk_version<WB: WasmBackend>(
    name: String,
    wasmer_module: &<WB as WasmBackend>::Module,
) -> PrepareResult<()> {
    let module_version = sdk_version::extract_from_compiled_module::<WB>(wasmer_module)?;

    let required_version = min_sdk_version();
    if module_version < *required_version {
        return Err(PrepareError::IncompatibleSDKVersions {
            module_name: name,
            required: required_version.clone(),
            provided: module_version,
        });
    }

    Ok(())
}

pub(crate) fn check_it_version(
    name: impl Into<String>,
    it_version: &semver::Version,
) -> PrepareResult<()> {
    let required_version = min_it_version();
    if it_version < required_version {
        return Err(PrepareError::IncompatibleITVersions {
            module_name: name.into(),
            required: required_version.clone(),
            provided: it_version.clone(),
        });
    }

    Ok(())
}
