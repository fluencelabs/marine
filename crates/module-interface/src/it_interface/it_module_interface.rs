/*
 * Copyright 2021 Fluence Labs Limited
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

use wasmer_it::IType;
use wasmer_it::ast::FunctionArg as IFunctionArg;
use wasmer_it::IRecordType;

use serde::Serialize;
use serde::Deserialize;

use std::collections::HashMap;
use std::rc::Rc;

pub type IRecordTypes = HashMap<u64, Rc<IRecordType>>;

/// Represent a function type inside Marine module.
#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct IFunctionSignature {
    pub name: Rc<String>,
    pub arguments: Rc<Vec<IFunctionArg>>,
    pub outputs: Rc<Vec<IType>>,
    pub adapter_function_type: u32,
}

/// Represent an interface of a Wasm module.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct IModuleInterface {
    pub export_record_types: IRecordTypes,
    pub record_types: IRecordTypes,
    pub function_signatures: Vec<IFunctionSignature>,
}
