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

mod json;
mod errors;
mod faas;
mod faas_interface;

pub(crate) type Result<T> = std::result::Result<T, FaaSError>;

pub use faas::FluenceFaaS;
pub use faas_interface::FaaSInterface;

pub use errors::FaaSError;

// Re-exports from Marine
pub(crate) use crate::IRecordType;
pub(crate) use crate::MModuleInterface as FaaSModuleInterface;

pub use marine_module_interface::interface::itype_text_view;

pub use marine_rs_sdk::CallParameters;
pub use marine_rs_sdk::SecurityTetraplet;
