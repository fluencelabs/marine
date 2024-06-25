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

use wasmer_it::IType;
use wasmer_it::ast::FunctionArg as IFunctionArg;
use wasmer_it::IRecordType;

use serde::Serialize;
use serde::Deserialize;

use std::collections::HashMap;
use std::sync::Arc;

pub type IRecordTypes = HashMap<u64, Arc<IRecordType>>;

/// Represent a function type inside Marine module.
#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct IFunctionSignature {
    pub name: Arc<String>,
    pub arguments: Arc<Vec<IFunctionArg>>,
    pub outputs: Arc<Vec<IType>>,
    pub adapter_function_type: u32,
}

/// Represent an interface of a Wasm module.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct IModuleInterface {
    pub export_record_types: IRecordTypes,
    pub record_types: IRecordTypes,
    pub function_signatures: Vec<IFunctionSignature>,
}
