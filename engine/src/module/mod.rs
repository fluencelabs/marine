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
mod memory;
mod wit_function;
mod wit_instance;
mod type_converters;
mod fce_module;

pub use wasmer_wit::types::InterfaceType as IType;
pub use wasmer_wit::types::RecordType as IRecordType;
pub use wasmer_wit::ast::FunctionArg as IFunctionArg;
pub use wasmer_wit::values::InterfaceValue as IValue;
pub use fce_module::FCEFunctionSignature;
pub use wasmer_wit::values::from_interface_values;
pub use wasmer_wit::values::to_interface_value;

pub(crate) use fce_module::FCEModule;
pub(self) use wasmer_core::types::Type as WType;
pub(self) use wasmer_core::types::Value as WValue;

// types that often used together
pub(self) mod wit_prelude {
    pub(super) use super::wit_instance::WITInstance;
    pub(super) use super::exports::WITExport;
    pub(super) use crate::FCEError;
    pub(super) use super::wit_function::WITFunction;
    pub(super) use super::memory::WITMemoryView;
    pub(super) use super::memory::WITMemory;
}
