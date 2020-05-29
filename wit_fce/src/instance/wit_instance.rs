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
use crate::instance::exports::WITExport;
use crate::instance::locals_imports::WITLocalImport;
use crate::instance::memory::{WITMemory, WITMemoryView};

use wasmer_interface_types::interpreter::wasm;
use wasmer_runtime_core::Instance as WasmerInstance;

use std::collections::HashMap;
use wasmer_interface_types::ast::Interfaces;
use wasmer_interface_types::ast::Type;
use wasmer_interface_types::interpreter::wasm::structures::{LocalImportIndex, TypedIndex};

pub struct WITInstance {
    // represent all import and export functions
    funcs: HashMap<usize, WITLocalImport>,
    memories: Vec<WITMemory>,
}

impl WITInstance {
    pub fn new(
        wasmer_instance: &WasmerInstance,
        interfaces: &Interfaces,
    ) -> Result<Self, WITFCEError> {
        let funcs = Self::extract_funcs(&wasmer_instance, interfaces)?;
        let memories = Self::extract_memories(&wasmer_instance);

        Ok(Self { funcs, memories })
    }

    fn extract_funcs(
        wasmer_instance: &WasmerInstance,
        interfaces: &Interfaces,
    ) -> Result<HashMap<usize, WITLocalImport>, WITFCEError> {
        use wasmer_runtime_core::DynFunc;
        let module_exports = &wasmer_instance.exports;

        interfaces
            .exports
            .iter()
            .enumerate()
            .map(|(export_id, export)| {
                let export_func = module_exports.get(export.name)?;
                unsafe {
                    // TODO: refactor this with new Wasmer API when it is ready
                    // here it is safe because dyn func is never lives WITInstance
                    let export_func =
                        std::mem::transmute::<DynFunc<'_>, DynFunc<'static>>(export_func);
                    Ok((export_id, WITLocalImport::new(export_func)?))
                }
            })
            .collect()
    }

    fn extract_memories(wasmer_instance: &WasmerInstance) -> Vec<WITMemory> {
        use wasmer_runtime_core::export::Export::Memory;

        let mut memories = wasmer_instance
            .exports()
            .filter_map(|(_, export)| match export {
                Memory(memory) => Some(WITMemory(memory)),
                _ => None,
            })
            .collect::<Vec<_>>();

        if let Some(Memory(memory)) = wasmer_instance
            .import_object
            .maybe_with_namespace("env", |env| env.get_export("memory"))
        {
            memories.push(WITMemory(memory));
        }

        memories
    }
}

impl<'instance> wasm::structures::Instance<WITExport, WITLocalImport, WITMemory, WITMemoryView<'_>>
    for WITInstance
{
    fn export(&self, _export_name: &str) -> Option<&WITExport> {
        // exports aren't needed for this version of WIT
        None
    }

    fn local_or_import<I: TypedIndex + LocalImportIndex>(
        &mut self,
        index: I,
    ) -> Option<&WITLocalImport> {
        self.funcs.get(&index.index())
    }

    fn memory(&self, index: usize) -> Option<&WITMemory> {
        if index >= self.memories.len() {
            None
        } else {
            Some(&self.memories[index])
        }
    }

    fn wit_type(&self, _index: u32) -> Option<&Type> {
        None
    }
}
