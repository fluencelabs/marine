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
use crate::instance::wit_module::WITModule;
use std::sync::Arc;
use wasmer_interface_types::interpreter::wasm;
use wasmer_interface_types::{types::InterfaceType, values::InterfaceValue};
use wasmer_runtime_core::instance::DynFunc;
use wasmer_runtime_core::types::Value;

#[derive(Clone)]
enum WITFunctionInner {
    Export {
        func: Arc<DynFunc<'static>>,
        inputs: Vec<InterfaceType>,
        outputs: Vec<InterfaceType>,
    },
    Import {
        wit_module: Arc<WITModule>,
        func_name: String,
        inputs: Vec<InterfaceType>,
        outputs: Vec<InterfaceType>,
    },
}

#[derive(Clone)]
pub(crate) struct WITFunction {
    inner: WITFunctionInner,
}

impl WITFunction {
    pub fn from_export(dyn_func: DynFunc<'static>) -> Result<Self, WITFCEError> {
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

        let inner = WITFunctionInner::Export {
            func: Arc::new(dyn_func),
            inputs,
            outputs,
        };

        Ok(Self { inner })
    }

    pub fn from_import(wit_module: Arc<WITModule>, func_name: String) -> Result<Self, WITFCEError> {
        let func_type = wit_module.as_ref().get_func_signature(&func_name)?;
        let inputs = func_type.0.clone();
        let outputs = func_type.1.clone();

        let inner = WITFunctionInner::Import {
            wit_module,
            func_name,
            inputs,
            outputs,
        };

        Ok(Self { inner })
    }
}

/*
impl std::ops::Deref for WITFuncs {
    type Target = DynFunc<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
 */

impl wasm::structures::LocalImport for WITFunction {
    fn inputs_cardinality(&self) -> usize {
        match &self.inner {
            WITFunctionInner::Export { ref inputs, .. } => inputs.len(),
            WITFunctionInner::Import { ref inputs, .. } => inputs.len(),
        }
    }

    fn outputs_cardinality(&self) -> usize {
        match &self.inner {
            WITFunctionInner::Export { ref outputs, .. } => outputs.len(),
            WITFunctionInner::Import { ref outputs, .. } => outputs.len(),
        }
    }

    fn inputs(&self) -> &[InterfaceType] {
        match &self.inner {
            WITFunctionInner::Export { ref inputs, .. } => inputs,
            WITFunctionInner::Import { ref inputs, .. } => inputs,
        }
    }

    fn outputs(&self) -> &[InterfaceType] {
        match &self.inner {
            WITFunctionInner::Export { ref outputs, .. } => outputs,
            WITFunctionInner::Import { ref outputs, .. } => outputs,
        }
    }

    fn call(&self, arguments: &[InterfaceValue]) -> Result<Vec<InterfaceValue>, ()> {
        use super::{ival_to_wval, wval_to_ival};

        match &self.inner {
            WITFunctionInner::Export { func, .. } => func
                .as_ref()
                .call(&arguments.iter().map(ival_to_wval).collect::<Vec<Value>>())
                .map(|results| results.iter().map(wval_to_ival).collect())
                .map_err(|_| ()),
            WITFunctionInner::Import {
                wit_module,
                func_name,
                ..
            } => {
                let mut wit_module_caller = wit_module.clone();
                unsafe {
                    // get_mut_unchecked here is safe because it is single-threaded environment
                    // without cyclic reference between modules
                    Arc::get_mut_unchecked(&mut wit_module_caller)
                        .call(func_name, arguments)
                        .map_err(|_| ())
                }
            }
        }
    }
}
