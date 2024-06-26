/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
