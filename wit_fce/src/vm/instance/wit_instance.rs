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

use super::wit_prelude::*;
use super::fce_module::FCEModule;

use wasmer_wit::interpreter::wasm;
use super::IAstType;
use wasmer_wit::ast::Interfaces;
use wasmer_wit::interpreter::wasm::structures::{
    LocalImportIndex, TypedIndex,
};
use wasmer_core::Instance as WasmerInstance;

use std::collections::HashMap;
use std::sync::Arc;

/// Contains all import and export functions that could be called from WIT context by call-core.
#[derive(Clone)]
pub(super) struct WITInstance {
    funcs: HashMap<usize, WITFunction>,
    memories: Vec<WITMemory>,
}

impl WITInstance {
    pub(super) fn new(
        wasmer_instance: &WasmerInstance,
        interfaces: &Interfaces,
        modules: &HashMap<String, Arc<FCEModule>>,
    ) -> Result<Self, WITFCEError> {
        let mut exports = Self::extract_raw_exports(&wasmer_instance, interfaces)?;
        let imports = Self::extract_imports(modules, interfaces, exports.len())?;
        let memories = Self::extract_memories(&wasmer_instance);

        exports.extend(imports);
        let funcs = exports;

        Ok(Self { funcs, memories })
    }

    fn extract_raw_exports(
        wasmer_instance: &WasmerInstance,
        interfaces: &Interfaces,
    ) -> Result<HashMap<usize, WITFunction>, WITFCEError> {
        use wasmer_core::DynFunc;

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
                    Ok((export_id, WITFunction::from_export(export_func)?))
                }
            })
            .collect()
    }

    /// Extracts only those imports that don't have implementations.
    fn extract_imports(
        modules: &HashMap<String, Arc<FCEModule>>,
        interfaces: &Interfaces,
        start_index: usize,
    ) -> Result<HashMap<usize, WITFunction>, WITFCEError> {
        // uses to filter import functions that have an adapter implementation
        let core_to_adapter = interfaces
            .implementations
            .iter()
            .map(|i| (i.core_function_type, i.adapter_function_type))
            .collect::<HashMap<u32, u32>>();

        let mut non_wit_callable_imports = HashMap::new();

        for import in interfaces.imports.iter() {
            if core_to_adapter.get(&import.function_type).is_some() {
                continue;
            }

            match modules.get(import.namespace) {
                Some(module) => {
                    let func = WITFunction::from_import(module.clone(), import.name.to_string())?;
                    non_wit_callable_imports
                        .insert(start_index + non_wit_callable_imports.len() as usize, func);
                }
                None => return Err(WITFCEError::NoSuchModule),
            }
        }

        Ok(non_wit_callable_imports)
    }

    fn extract_memories(wasmer_instance: &WasmerInstance) -> Vec<WITMemory> {
        use wasmer_core::export::Export::Memory;

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

impl wasm::structures::Instance<WITExport, WITFunction, WITMemory, WITMemoryView<'_>>
    for WITInstance
{
    fn export(&self, _export_name: &str) -> Option<&WITExport> {
        // exports aren't used in this version of WIT
        None
    }

    fn local_or_import<I: TypedIndex + LocalImportIndex>(
        &mut self,
        index: I,
    ) -> Option<&WITFunction> {
        self.funcs.get(&index.index())
    }

    fn memory(&self, index: usize) -> Option<&WITMemory> {
        if index >= self.memories.len() {
            None
        } else {
            Some(&self.memories[index])
        }
    }

    fn wit_type(&self, _index: u32) -> Option<&IAstType> {
        None
    }
}
