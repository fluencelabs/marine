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
mod interface_transformer;
mod itype_to_text;
mod module_interface;
mod records_transformer;

pub use errors::InterfaceError;
pub use interface_transformer::it_to_module_interface;
pub use itype_to_text::*;
pub use module_interface::*;

pub type InterfaceResult<T> = std::result::Result<T, InterfaceError>;

use marine_it_interfaces::MITInterfaces;

/// Returns interface of a Marine module.
pub fn get_interface(mit: &MITInterfaces<'_>) -> InterfaceResult<ModuleInterface> {
    let it_interface = crate::it_interface::get_interface(mit)?;
    let interface = it_to_module_interface(it_interface)?;

    Ok(interface)
}
