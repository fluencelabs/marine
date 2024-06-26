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

mod errors;
mod export_it_functions;
mod export_it_records;
mod it_module_interface;

pub use errors::*;
pub use export_it_functions::*;
pub use export_it_records::*;
pub use it_module_interface::*;

pub type RIResult<T> = std::result::Result<T, ITInterfaceError>;

use marine_it_interfaces::MITInterfaces;

/// Returns Marine module interface that includes both export and all record types.
pub fn get_interface(mit: &MITInterfaces<'_>) -> RIResult<IModuleInterface> {
    let function_signatures = get_export_funcs(mit)?;
    let FullRecordTypes {
        record_types,
        export_record_types,
    } = get_record_types(mit, function_signatures.iter())?;

    let mm_interface = IModuleInterface {
        export_record_types,
        record_types,
        function_signatures,
    };

    Ok(mm_interface)
}
