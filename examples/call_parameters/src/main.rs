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

use marine_rs_sdk::CallParameters;
#[cfg(target_arch = "wasm32")]
use marine_rs_sdk::marine;
#[cfg(target_arch = "wasm32")]
use marine_rs_sdk::module_manifest;

#[cfg(target_arch = "wasm32")]
module_manifest!();

pub fn main() {}

#[marine]
#[cfg(target_arch = "wasm32")]
pub fn call_parameters() -> CallParameters {
    marine_rs_sdk::get_call_parameters()
}
