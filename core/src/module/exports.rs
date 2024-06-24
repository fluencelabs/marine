/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use super::IValue;
use super::IType;
use super::IFunctionArg;
use wasmer_it::interpreter::wasm;

use futures::future::BoxFuture;
use futures::FutureExt;

// In current implementation export simply does nothing, because there is no more
// explicit instruction call-export in this version of wasmer-interface-types,
// but explicit Exports is still required by wasmer-interface-types::Interpreter.
#[derive(Clone)]
pub(crate) struct ITExport {
    name: String,
    arguments: Vec<IFunctionArg>,
    outputs: Vec<IType>,
    function: fn(arguments: &[IValue]) -> Result<Vec<IValue>, anyhow::Error>,
}

impl ITExport {
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            arguments: vec![],
            outputs: vec![],
            function: |_| -> _ { Ok(vec![]) },
        }
    }
}

impl wasm::structures::Export for ITExport {
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

    fn call_async<'args>(
        &'args self,
        arguments: &'args [IValue],
    ) -> BoxFuture<'args, Result<Vec<IValue>, anyhow::Error>> {
        async move { (self.function)(arguments) }.boxed()
    }
}
