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
