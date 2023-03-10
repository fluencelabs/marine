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

use marine_wasm_backend_traits::RuntimeResult;
use marine_wasm_backend_traits::WasmBackend;

pub use errors::HostImportError;
pub(crate) use imports::create_host_import_func;

pub(self) use marine_wasm_backend_traits::WValue;
pub(self) use marine_wasm_backend_traits::WType;

pub(self) type HostImportResult<T> = std::result::Result<T, HostImportError>;
pub(self) type WasmModuleFunc<WB, Args, Rets> = Box<
    dyn FnMut(&mut <WB as WasmBackend>::ContextMut<'_>, Args) -> RuntimeResult<Rets> + Sync + Send,
>;
pub(self) type AllocateFunc<WB> = WasmModuleFunc<WB, (i32, i32), i32>;

pub(self) const ALLOCATE_FUNC_NAME: &str = "allocate";
pub(self) const SET_PTR_FUNC_NAME: &str = "set_result_ptr";
pub(self) const SET_SIZE_FUNC_NAME: &str = "set_result_size";
