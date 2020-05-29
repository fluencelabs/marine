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

use wasmer_interface_types::interpreter::wasm;
use wasmer_interface_types::{types::InterfaceType, values::InterfaceValue};

// In current implementation export simply does nothing.
#[derive(Clone)]
pub(crate) struct WITExport {
    pub(crate) inputs: Vec<InterfaceType>,
    pub(crate) outputs: Vec<InterfaceType>,
    pub(crate) function: fn(arguments: &[InterfaceValue]) -> Result<Vec<InterfaceValue>, ()>,
}

impl WITExport {
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self {
            inputs: vec![],
            outputs: vec![],
            function: |_| -> _ { Ok(vec![]) },
        }
    }
}

impl wasm::structures::Export for WITExport {
    fn inputs_cardinality(&self) -> usize {
        self.inputs.len() as usize
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
        (self.function)(arguments)
    }
}
