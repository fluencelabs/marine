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

use crate::instance::errors::WITFCEError;
use wasmer_interface_types::interpreter::wasm;
use wasmer_interface_types::{types::InterfaceType, values::InterfaceValue};
use wasmer_runtime_core::instance::DynFunc;
use wasmer_runtime_core::types::Value;

pub(crate) struct WITLocalImport {
    inner: DynFunc<'static>,
    inputs: Vec<InterfaceType>,
    outputs: Vec<InterfaceType>,
}

impl WITLocalImport {
    pub fn new(dyn_func: DynFunc<'static>) -> Result<Self, WITFCEError> {
        use super::wtype_to_itype;

        let signature = dyn_func.signature();
        let inputs = signature
            .params()
            .iter()
            .map(wtype_to_itype)
            .collect::<Vec<_>>();
        let outputs = signature
            .returns()
            .iter()
            .map(wtype_to_itype)
            .collect::<Vec<_>>();

        Ok(Self {
            inner: dyn_func,
            inputs,
            outputs,
        })
    }
}

impl std::ops::Deref for WITLocalImport {
    type Target = DynFunc<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl wasm::structures::LocalImport for WITLocalImport {
    fn inputs_cardinality(&self) -> usize {
        self.inputs.len()
    }

    fn outputs_cardinality(&self) -> usize {
        self.outputs.len()
    }

    fn inputs(&self) -> &[InterfaceType] {
        &self.inputs
    }

    fn outputs(&self) -> &[InterfaceType] {
        &self.outputs
    }

    fn call(&self, arguments: &[InterfaceValue]) -> Result<Vec<InterfaceValue>, ()> {
        use super::{ival_to_wval, wval_to_ival};

        self.inner
            .call(&arguments.iter().map(ival_to_wval).collect::<Vec<Value>>())
            .map(|results| results.iter().map(wval_to_ival).collect())
            .map_err(|_| ())
    }
}
