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

use super::wit_prelude::FCEError;
use super::fce_module::FCEModule;
use super::{IType, IValue, WValue};
use crate::vm::module::fce_module::Callable;

use wasmer_wit::interpreter::wasm;
use wasmer_core::instance::DynFunc;

use std::sync::Arc;

#[derive(Clone)]
enum WITFunctionInner {
    Export {
        func: Arc<DynFunc<'static>>,
        inputs: Vec<IType>,
        outputs: Vec<IType>,
    },
    Import {
        // TODO: use dyn Callable here
        callable: Arc<Callable>,
    },
}

/// Represents all import and export functions that could be called from WIT context by call-core.
#[derive(Clone)]
pub(super) struct WITFunction {
    inner: WITFunctionInner,
}

impl WITFunction {
    /// Creates functions from a "usual" (not WIT) module export.
    pub(super) fn from_export(dyn_func: DynFunc<'static>) -> Result<Self, FCEError> {
        use super::type_converters::wtype_to_itype;

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

    /// Creates function from a module import.
    pub(super) fn from_import(
        wit_module: &FCEModule,
        function_name: &str,
    ) -> Result<Self, FCEError> {
        let callable = wit_module.get_callable(function_name)?;

        let inner = WITFunctionInner::Import { callable };

        Ok(Self { inner })
    }
}

impl wasm::structures::LocalImport for WITFunction {
    fn inputs_cardinality(&self) -> usize {
        match &self.inner {
            WITFunctionInner::Export { ref inputs, .. } => inputs.len(),
            WITFunctionInner::Import { ref callable, .. } => callable.wit_module_func.inputs.len(),
        }
    }

    fn outputs_cardinality(&self) -> usize {
        match &self.inner {
            WITFunctionInner::Export { ref outputs, .. } => outputs.len(),
            WITFunctionInner::Import { ref callable, .. } => callable.wit_module_func.outputs.len(),
        }
    }

    fn inputs(&self) -> &[IType] {
        match &self.inner {
            WITFunctionInner::Export { ref inputs, .. } => inputs,
            WITFunctionInner::Import { ref callable, .. } => &callable.wit_module_func.inputs,
        }
    }

    fn outputs(&self) -> &[IType] {
        match &self.inner {
            WITFunctionInner::Export { ref outputs, .. } => outputs,
            WITFunctionInner::Import { ref callable, .. } => &callable.wit_module_func.outputs,
        }
    }

    fn call(&self, arguments: &[IValue]) -> Result<Vec<IValue>, ()> {
        use super::type_converters::{ival_to_wval, wval_to_ival};
        // println!("wit_function called with {:?}", arguments);

        match &self.inner {
            WITFunctionInner::Export { func, .. } => func
                .as_ref()
                .call(&arguments.iter().map(ival_to_wval).collect::<Vec<WValue>>())
                .map(|result| result.iter().map(wval_to_ival).collect())
                .map_err(|_| ()),
            WITFunctionInner::Import { callable } => Arc::make_mut(&mut callable.clone())
                .call(arguments)
                .map_err(|_| ()),
        }
    }
}
