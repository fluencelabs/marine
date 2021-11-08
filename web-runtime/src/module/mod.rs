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
pub mod type_converters;

pub use wit_instance::MRecordTypes;
pub use wasmer_it::IType;
pub use wasmer_it::IRecordType;
pub use wasmer_it::ast::FunctionArg as IFunctionArg;
pub use wasmer_it::IValue;
pub use wasmer_it::from_interface_values;
pub use wasmer_it::to_interface_value;

use serde::Serialize;
use serde::Deserialize;
use std::rc::Rc;

/// Represent a function type inside Marine module.
#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct MFunctionSignature {
    pub name: Rc<String>,
    pub arguments: Rc<Vec<IFunctionArg>>,
    pub outputs: Rc<Vec<IType>>,
}

#[allow(unused)]
pub(crate) use marine_module::MModule;
//pub(self) use wasmer_core::types::Type as WType;
pub(self) use crate::marine_js::WType;
//pub(self) use wasmer_core::types::Value as WValue;
pub(self) use crate::marine_js::WValue;

// types that often used together
pub(self) mod wit_prelude {
    pub(super) use super::wit_instance::ITInstance;
    pub(super) use super::exports::ITExport;
    pub(super) use crate::MError;
    pub(super) use super::wit_function::WITFunction;
    pub(super) use super::memory::WITMemoryView;
    pub(super) use super::memory::WITMemory;
}
