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
use crate::instance::memory::{WITMemory, WITMemoryView};
use crate::instance::wit_function::WITFunction;

use wasmer_interface_types::interpreter::wasm;
use wasmer_runtime_core::Instance as WasmerInstance;

use crate::instance::wit_module::WITModule;
use std::collections::HashMap;
use std::sync::Arc;
use wasmer_interface_types::ast::Interfaces;
use wasmer_interface_types::ast::Type;
use wasmer_interface_types::interpreter::wasm::structures::{
    LocalImport, LocalImportIndex, TypedIndex,
};
use wasmer_interface_types::types::InterfaceType;

#[derive(Clone)]
pub struct WITInstance {
    // represent all import and export functions that could be called from WIT context
    funcs: HashMap<usize, WITFunction>,
    memories: Vec<WITMemory>,
}

impl WITInstance {
    pub fn new(
        wasmer_instance: &WasmerInstance,
        interfaces: &Interfaces,
        modules: &HashMap<String, Arc<WITModule>>,
    ) -> Result<Self, WITFCEError> {
        let mut exports = Self::extract_exports(&wasmer_instance, interfaces)?;
        println!("exports count {}", exports.len());
        let imports = Self::extract_imports(modules, interfaces, exports.len())?;
        println!("imports count {}", imports.len());
        let memories = Self::extract_memories(&wasmer_instance);

        exports.extend(imports);
        let funcs = exports;

        Ok(Self { funcs, memories })
    }

    pub fn get_func_signature(
        &self,
        func_idx: usize,
    ) -> Result<(Vec<InterfaceType>, Vec<InterfaceType>), WITFCEError> {
        match self.funcs.get(&func_idx) {
            Some(func) => Ok((func.inputs().to_owned(), func.outputs().to_owned())),
            None => Err(WITFCEError::NoSuchFunction(format!(
                "function with idx = {} hasn't been found during its signature looking up",
                func_idx
            ))),
        }
    }

    fn extract_exports(
        wasmer_instance: &WasmerInstance,
        interfaces: &Interfaces,
    ) -> Result<HashMap<usize, WITFunction>, WITFCEError> {
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
                    let tt = WITFunction::from_export(export_func)?;
                    println!("{}, {} - {:?}", export_id, export.name, tt.inputs());
                    Ok((export_id, tt))
                }
            })
            .collect()
    }

    /// Extracts only those imports that don't have implementations.
    fn extract_imports(
        modules: &HashMap<String, Arc<WITModule>>,
        interfaces: &Interfaces,
        start_index: usize,
    ) -> Result<HashMap<usize, WITFunction>, WITFCEError> {
        // uses to filter import functions that have an adapter implementation
        let core_to_adapter = interfaces
            .implementations
            .iter()
            .map(|i| (i.core_function_type, i.adapter_function_type))
            .collect::<HashMap<_, _>>();

        let mut non_wit_callable_imports = HashMap::new();

        for import in interfaces.imports.iter() {
            if let Some(_) = core_to_adapter.get(&import.function_type) {
                continue;
            }

            match modules.get(import.namespace) {
                Some(module) => {
                    let func = WITFunction::from_import(module.clone(), import.name.to_string())?;
                    println!(
                        "{}, {} - {:?}",
                        start_index + non_wit_callable_imports.len(),
                        import.name,
                        func.inputs()
                    );
                    non_wit_callable_imports
                        .insert(start_index + non_wit_callable_imports.len() as usize, func);
                }
                None => return Err(WITFCEError::NoSuchModule),
            }
        }

        Ok(non_wit_callable_imports)
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

impl<'instance> wasm::structures::Instance<WITExport, WITFunction, WITMemory, WITMemoryView<'_>>
    for WITInstance
{
    fn export(&self, _export_name: &str) -> Option<&WITExport> {
        // exports aren't needed for this version of WIT
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

    fn wit_type(&self, _index: u32) -> Option<&Type> {
        None
    }
}
