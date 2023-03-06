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

use crate::function_type_to_func_sig;
use crate::WasmerBackend;
use crate::WasmerContextMut;
use crate::WasmerFunction;
use crate::WasmerMemory;

use marine_wasm_backend_traits::prelude::*;

use anyhow::anyhow;
use wasmer::Extern;

#[derive(Clone)]
pub struct WasmerInstance {
    pub(crate) inner: wasmer::Instance,
}

impl Instance<WasmerBackend> for WasmerInstance {
    fn export_iter<'a>(
        &'a self,
        mut store: WasmerContextMut<'a>,
    ) -> Box<dyn Iterator<Item = (&'a str, Export<WasmerBackend>)> + 'a> {
        let iter =
            self.inner
                .exports
                .iter()
                .map(move |(name, export): (&String, &wasmer::Extern)| {
                    let export = match export {
                        wasmer::Extern::Function(function) => Export::Function(
                            WasmerFunction::from_func(&mut store, function.clone()),
                        ),
                        wasmer::Extern::Memory(memory) => Export::Memory(memory.clone().into()),
                        _ => Export::Other,
                    };

                    (name.as_str(), export)
                });
        Box::new(iter)
    }

    fn get_nth_memory(
        &self,
        _store: &mut impl AsContextMut<WasmerBackend>,
        memory_index: u32,
    ) -> Option<WasmerMemory> {
        self.inner
            .exports
            .iter()
            .filter_map(|(_name, export)| match export {
                Extern::Memory(memory) => Some(memory),
                _ => None,
            }) // TODO is there a way to make it better?
            .nth(memory_index as usize)
            .map(|memory| WasmerMemory {
                inner: memory.clone(),
            })
    }

    fn get_memory(
        &self,
        _store: &mut impl AsContextMut<WasmerBackend>,
        name: &str,
    ) -> ResolveResult<WasmerMemory> {
        self.inner
            .exports
            .get_memory(name)
            .map_err(|e| ResolveError::Other(anyhow!(e))) // TODO make detailed
            .map(|memory| WasmerMemory {
                inner: memory.clone(),
            })
    }

    fn get_function(
        &self,
        store: &mut impl AsContextMut<WasmerBackend>,
        name: &str,
    ) -> ResolveResult<<WasmerBackend as WasmBackend>::Function> {
        let owner_memory = self
            .inner
            .exports
            .iter()
            .filter_map(|(_name, export)| match export {
                Extern::Memory(memory) => Some(memory),
                _ => None,
            })
            .next()
            .map(Clone::clone); // TODO cache memories and export in the instance

        self.inner
            .exports
            .get_function(name)
            .map_err(|e| ResolveError::Other(anyhow!("wasmer cannot find function {}", e))) // TODO make detailed
            .map(|func| {
                let ty = func.ty(&store.as_context());

                WasmerFunction {
                    sig: function_type_to_func_sig(&ty),
                    inner: func.clone(),
                    owner_memory,
                }
            })
    }
}
