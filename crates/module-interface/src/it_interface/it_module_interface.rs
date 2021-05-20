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

pub type MRecordTypes = HashMap<u64, Rc<IRecordType>>;

/// Represent a function type inside Marine module.
#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct MFunctionSignature {
    pub name: Rc<String>,
    pub arguments: Rc<Vec<IFunctionArg>>,
    pub outputs: Rc<Vec<IType>>,
}

/// Represent an interface of a Wasm module.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct MModuleInterface {
    pub record_types: MRecordTypes,
    pub function_signatures: Vec<MFunctionSignature>,
}

/// Represent an interface of a Wasm module. This interface is intended for Marine runtime internal
/// usage and includes export and all record types.
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct FullMModuleInterface {
    pub all_record_types: MRecordTypes,
    pub export_record_types: MRecordTypes,
    pub function_signatures: Vec<MFunctionSignature>,
}
