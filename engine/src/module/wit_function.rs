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

use super::fce_module::FCEModule;
use super::{IType, IFunctionArg, IValue, WValue};
use super::fce_module::Callable;
use crate::Result;

use wasmer_wit::interpreter::wasm;
use wasmer_core::instance::DynFunc;

use std::sync::Arc;

#[derive(Clone)]
enum WITFunctionInner {
    Export {
        func: Arc<DynFunc<'static>>,
        arguments: Vec<IFunctionArg>,
        outputs: Vec<IType>,
    },
    Import {
        // TODO: use dyn Callable here
        callable: Arc<Callable>,
        arguments: Vec<IFunctionArg>,
        outputs: Vec<IType>,
    },
}

/// Represents all import and export functions that could be called from WIT context by call-core.
#[derive(Clone)]
pub(super) struct WITFunction {
    inner: WITFunctionInner,
}

impl WITFunction {
    /// Creates functions from a "usual" (not WIT) module export.
    pub(super) fn from_export(dyn_func: DynFunc<'static>) -> Result<Self> {
        use super::type_converters::wtype_to_itype;

        let signature = dyn_func.signature();
        let arguments = signature
            .params()
            .iter()
            .map(|wtype| IFunctionArg {
                // here it's considered as an anonymous arguments
                name: String::new(),
                ty: wtype_to_itype(wtype),
            })
            .collect::<Vec<_>>();
        let outputs = signature
            .returns()
            .iter()
            .map(wtype_to_itype)
            .collect::<Vec<_>>();

        let inner = WITFunctionInner::Export {
            func: Arc::new(dyn_func),
            arguments,
            outputs,
        };

        Ok(Self { inner })
    }

    /// Creates function from a module import.
    pub(super) fn from_import(
        wit_module: &FCEModule,
        function_name: &str,
        arguments: Vec<IFunctionArg>,
        outputs: Vec<IType>,
    ) -> Result<Self> {
        let callable = wit_module.get_callable(function_name)?;

        let inner = WITFunctionInner::Import {
            callable,
            arguments,
            outputs,
        };

        Ok(Self { inner })
    }
}

impl wasm::structures::LocalImport for WITFunction {
    fn inputs_cardinality(&self) -> usize {
        match &self.inner {
            WITFunctionInner::Export { arguments, .. } => arguments.len(),
            WITFunctionInner::Import { arguments, .. } => arguments.len(),
        }
    }

    fn outputs_cardinality(&self) -> usize {
        match &self.inner {
            WITFunctionInner::Export { outputs, .. } => outputs.len(),
            WITFunctionInner::Import { outputs, .. } => outputs.len(),
        }
    }

    fn arguments(&self) -> &[IFunctionArg] {
        match &self.inner {
            WITFunctionInner::Export { arguments, .. } => arguments,
            WITFunctionInner::Import { arguments, .. } => arguments,
        }
    }

    fn outputs(&self) -> &[IType] {
        match &self.inner {
            WITFunctionInner::Export { outputs, .. } => outputs,
            WITFunctionInner::Import { outputs, .. } => outputs,
        }
    }

    fn call(&self, arguments: &[IValue]) -> std::result::Result<Vec<IValue>, ()> {
        use super::type_converters::{ival_to_wval, wval_to_ival};

        match &self.inner {
            WITFunctionInner::Export { func, .. } => func
                .as_ref()
                .call(&arguments.iter().map(ival_to_wval).collect::<Vec<WValue>>())
                .map(|result| result.iter().map(wval_to_ival).collect())
                .map_err(|_| ()),
            WITFunctionInner::Import { callable, .. } => Arc::make_mut(&mut callable.clone())
                .call(arguments)
                .map_err(|_| ()),
        }
    }
}
