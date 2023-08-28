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
mod wit_function;
mod wit_instance;
mod type_converters;

use marine_wasm_backend_traits::WValue;

pub use wit_instance::MRecordTypes;

pub use wasmer_it::IType;
pub use wasmer_it::IRecordType;
pub use wasmer_it::ast::FunctionArg as IFunctionArg;
pub use wasmer_it::IValue;
pub use wasmer_it::from_interface_values;
pub use wasmer_it::to_interface_value;

use serde::Serialize;
use serde::Deserialize;
use std::sync::Arc;

/// Represent a function type inside Marine module.
#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct MFunctionSignature {
    pub name: Arc<String>,
    pub arguments: Arc<Vec<IFunctionArg>>,
    pub outputs: Arc<Vec<IType>>,
}

pub(crate) use marine_module::MModule;


// types that often used together
pub(crate) mod wit_prelude {
    pub(super) use super::wit_instance::ITInstance;
    pub(super) use super::exports::ITExport;
    pub(super) use crate::MError;
    pub(super) use super::wit_function::WITFunction;
}
