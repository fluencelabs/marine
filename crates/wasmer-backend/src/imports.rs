/*
 * Copyright 2023 Fluence Labs Limited
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

use crate::WasmerBackend;

use marine_wasm_backend_traits::*;

use wasmer::Extern;

#[derive(Clone)]
pub struct WasmerImports {
    pub(crate) inner: wasmer::Imports,
}

pub struct WasmerNamespace {}

impl Imports<WasmerBackend> for WasmerImports {
    fn new(_store: &mut <WasmerBackend as WasmBackend>::Store) -> Self {
        Self {
            inner: wasmer::Imports::new(),
        }
    }

    fn insert(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        func: <WasmerBackend as WasmBackend>::Function,
    ) -> ImportResult<()> {
        // todo check for existence
        self.inner.define(&module.into(), &name.into(), func.inner);
        Ok(())
    }

    fn register<S, I>(&mut self, name: S, namespace: I) -> ImportResult<()>
    where
        S: Into<String>,
        I: IntoIterator<Item = (String, <WasmerBackend as WasmBackend>::Function)>,
    {
        // todo check for existence
        let namespace = namespace
            .into_iter()
            .map(|(name, func)| (name, Extern::Function(func.inner)));

        self.inner.register_namespace(&name.into(), namespace);

        Ok(())
    }
}
