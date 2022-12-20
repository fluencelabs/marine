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

use super::marine_module::MModule;
use super::{IType, IFunctionArg, IValue};
use super::marine_module::Callable;
use crate::MResult;

use marine_wasm_backend_traits::{AsContextMut, WasmBackend, WValue};
use marine_wasm_backend_traits::ExportedDynFunc;

use wasmer_it::interpreter::wasm;

// use std::sync::Arc;
use std::rc::Rc;

#[derive(Clone)]
enum WITFunctionInner<WB: WasmBackend> {
    Export {
        func: Rc<<WB as WasmBackend>::ExportedDynFunc>,
    },
    Import {
        // TODO: use dyn Callable here
        callable: Rc<Callable<WB>>,
    },
}

/// Represents all import and export functions that could be called from IT context by call-core.
#[derive(Clone)]
pub(super) struct WITFunction<WB: WasmBackend> {
    name: String,
    arguments: Rc<Vec<IFunctionArg>>,
    outputs: Rc<Vec<IType>>,
    inner: WITFunctionInner<WB>,
}

impl<WB: WasmBackend> WITFunction<WB> {
    /// Creates functions from a "usual" (not IT) module export.
    pub(super) fn from_export(
        store: &mut <WB as WasmBackend>::Store,
        dyn_func: <WB as WasmBackend>::ExportedDynFunc,
        name: String,
    ) -> MResult<Self> {
        use super::type_converters::wtype_to_itype;
        let signature = dyn_func.signature(store.as_context_mut());
        let arguments = signature
            .params()
            .map(|wtype| IFunctionArg {
                // here it's considered as an anonymous arguments
                name: String::new(),
                ty: wtype_to_itype(wtype),
            })
            .collect::<Vec<_>>();
        let outputs = signature.returns().map(wtype_to_itype).collect::<Vec<_>>();

        let inner = WITFunctionInner::Export {
            func: Rc::new(dyn_func),
        };

        let arguments = Rc::new(arguments);
        let outputs = Rc::new(outputs);

        Ok(Self {
            name,
            arguments,
            outputs,
            inner,
        })
    }

    /// Creates function from a module import.
    pub(super) fn from_import(
        wit_module: &MModule<WB>,
        module_name: &str,
        function_name: &str,
        arguments: Rc<Vec<IFunctionArg>>,
        outputs: Rc<Vec<IType>>,
    ) -> MResult<Self> {
        let callable = wit_module.get_callable(module_name, function_name)?;

        let inner = WITFunctionInner::Import { callable };

        let name = function_name.to_string();

        Ok(Self {
            name,
            arguments,
            outputs,
            inner,
        })
    }
}

impl<'c, WB: WasmBackend> wasm::structures::LocalImport<<WB as WasmBackend>::ContextMut<'c>>
    for WITFunction<WB>
{
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn inputs_cardinality(&self) -> usize {
        self.arguments.len()
    }

    fn outputs_cardinality(&self) -> usize {
        self.outputs.len()
    }

    fn arguments(&self) -> &[IFunctionArg] {
        &self.arguments
    }

    fn outputs(&self) -> &[IType] {
        &self.outputs
    }

    fn call(
        &self,
        store: &mut <WB as WasmBackend>::ContextMut<'_>,
        arguments: &[IValue],
    ) -> std::result::Result<Vec<IValue>, ()> {
        use super::type_converters::wval_to_ival;
        use super::type_converters::ival_to_wval;
        match &self.inner {
            WITFunctionInner::Export { func, .. } => func
                .as_ref()
                .call(
                    store.as_context_mut(),
                    arguments
                        .iter()
                        .map(ival_to_wval)
                        .collect::<Vec<WValue>>()
                        .as_slice(),
                )
                .map_err(|_| ())
                .map(|results| results.iter().map(wval_to_ival).collect()),
            WITFunctionInner::Import { callable, .. } => Rc::make_mut(&mut callable.clone())
                .call(store, arguments)
                .map_err(|_| ()),
        }
    }
}
