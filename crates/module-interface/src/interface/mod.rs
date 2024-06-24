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
