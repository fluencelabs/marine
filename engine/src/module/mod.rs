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

mod exports;
mod marine_module;
mod memory;
mod wit_function;
mod wit_instance;
mod type_converters;

pub use wit_instance::RecordTypes;

pub use wasmer_wit::IType;
pub use wasmer_wit::IRecordType;
pub use wasmer_wit::ast::FunctionArg as IFunctionArg;
pub use wasmer_wit::IValue;
pub use marine_module::MFunctionSignature;
pub use wasmer_wit::from_interface_values;
pub use wasmer_wit::to_interface_value;

pub(crate) use marine_module::MModule;
pub(self) use wasmer_core::types::Type as WType;
pub(self) use wasmer_core::types::Value as WValue;

// types that often used together
pub(self) mod wit_prelude {
    pub(super) use super::wit_instance::ITInstance;
    pub(super) use super::exports::WITExport;
    pub(super) use crate::MError;
    pub(super) use super::wit_function::WITFunction;
    pub(super) use super::memory::WITMemoryView;
    pub(super) use super::memory::WITMemory;
}
