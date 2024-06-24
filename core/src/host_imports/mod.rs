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
mod lifting;
mod lowering;
mod imports;
mod utils;

use marine_wasm_backend_traits::TypedFunc;

pub use errors::HostImportError;
pub(crate) use imports::create_host_import_func;

use marine_wasm_backend_traits::WValue;
use marine_wasm_backend_traits::WType;

type HostImportResult<T> = std::result::Result<T, HostImportError>;
type AllocateFunc<WB> = TypedFunc<WB, (i32, i32), i32>;

const ALLOCATE_FUNC_NAME: &str = "allocate";
const SET_PTR_FUNC_NAME: &str = "set_result_ptr";
const SET_SIZE_FUNC_NAME: &str = "set_result_size";
