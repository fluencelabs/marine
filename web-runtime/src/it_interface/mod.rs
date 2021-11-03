/*
 * Copyright 2021 Fluence Labs Limited
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
