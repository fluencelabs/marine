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
use super::marine_module::MModule;
use super::IRecordType;
use crate::MResult;

use marine_wasm_backend_traits::AsContextMut;
use marine_wasm_backend_traits::STANDARD_MEMORY_EXPORT_NAME;
use marine_wasm_backend_traits::DelayedContextLifetime;
use marine_wasm_backend_traits::WasmBackend;
use marine_wasm_backend_traits::Instance;

use marine_it_interfaces::MITInterfaces;
use marine_it_interfaces::ITAstType;

use wasmer_it::interpreter::wasm;
use wasmer_it::interpreter::wasm::structures::LocalImportIndex;
use wasmer_it::interpreter::wasm::structures::Memory;
use wasmer_it::interpreter::wasm::structures::TypedIndex;

use std::collections::HashMap;
use std::sync::Arc;

pub type MRecordTypes = HashMap<u64, Arc<IRecordType>>;

/// Contains all import and export functions that could be called from IT context by call-core.
#[derive(Clone)]
pub(super) struct ITInstance<WB: WasmBackend> {
    /// IT functions indexed by id.
    funcs: HashMap<usize, WITFunction<WB>>,

    /// IT memories.
    memories: Vec<<WB as WasmBackend>::Memory>,

    /// All record types that instance contains.
    record_types_by_id: MRecordTypes,
}

impl<WB: WasmBackend> ITInstance<WB> {
    pub(super) fn new(
        wasm_instance: &<WB as WasmBackend>::Instance,
        store: &mut <WB as WasmBackend>::Store,
        module_name: &str,
        wit: &MITInterfaces<'_>,
        modules: &HashMap<String, MModule<WB>>,
    ) -> MResult<Self> {
        let mut exports = Self::extract_raw_exports(wasm_instance, store, wit)?;
        let imports = Self::extract_imports(module_name, modules, wit, exports.len())?;
        let memories = Self::extract_memories(wasm_instance, store);

        exports.extend(imports);
        let funcs = exports;

        let record_types_by_id = Self::extract_record_types(wit);

        Ok(Self {
            funcs,
            memories,
            record_types_by_id,
        })
    }

    fn extract_raw_exports(
        wasm_instance: &<WB as WasmBackend>::Instance,
        store: &mut <WB as WasmBackend>::Store,
        it: &MITInterfaces<'_>,
    ) -> MResult<HashMap<usize, WITFunction<WB>>> {
        it.exports()
            .enumerate()
            .map(|(export_id, export)| {
                let export_func = wasm_instance.get_function(store, export.name)?;
                Ok((
                    export_id,
                    WITFunction::from_export(store, export_func, export.name.to_string())?,
                ))
            })
            .collect()
    }

    /// Extracts only those imports that don't have implementations.
    fn extract_imports(
        module_name: &str,
        modules: &HashMap<String, MModule<WB>>,
        wit: &MITInterfaces<'_>,
        start_index: usize,
    ) -> MResult<HashMap<usize, WITFunction<WB>>> {
        wit.imports()
            .filter(|import| wit
                .adapter_types_by_core_type(import.function_type)
                .is_some())
            .enumerate()
            .map(|(idx, import)| match modules.get(import.namespace) {
                Some(module) => {
                    use wasmer_it::ast::Type;
                    let (arguments, output_types) =
                        match wit.type_by_idx_r(import.function_type - 2)? {
                            Type::Function {
                                arguments,
                                output_types,
                            } => (arguments.clone(), output_types.clone()),
                            ty => {
                                return Err(MError::IncorrectWIT(format!(
                                    "IT should has Type::Function, but {:?} met",
                                    ty
                                )))
                            }
                        };

                    let func = WITFunction::from_import(
                        module,
                        module_name,
                        import.name,
                        arguments,
                        output_types,
                    )?;

                    Ok((start_index + idx, func))
                }
                None => Err(MError::NoSuchModule(import.namespace.to_string())),
            })
            .collect::<MResult<HashMap<_, _>>>()
    }

    fn extract_memories(
        wasm_instance: &<WB as WasmBackend>::Instance,
        store: &mut <WB as WasmBackend>::Store,
    ) -> Vec<<WB as WasmBackend>::Memory> {
        use marine_wasm_backend_traits::Export::Memory;

        let mut memories = wasm_instance
            .export_iter(store.as_context_mut())
            .filter_map(|(_, export)| match export {
                Memory(memory) => Some(memory),
                _ => None,
            })
            .collect::<Vec<_>>();

        if let Ok(memory) = wasm_instance.get_memory(store, STANDARD_MEMORY_EXPORT_NAME) {
            memories.push(memory);
        }

        memories
    }

    fn extract_record_types(wit: &MITInterfaces<'_>) -> MRecordTypes {
        let (record_types_by_id, _) = wit.types().fold(
            (HashMap::new(), 0u64),
            |(mut record_types_by_id, id), ty| {
                match ty {
                    ITAstType::Record(record_type) => {
                        record_types_by_id.insert(id, record_type.clone());
                    }
                    ITAstType::Function { .. } => {}
                };
                (record_types_by_id, id + 1)
            },
        );

        record_types_by_id
    }
}

impl<WB: WasmBackend>
    wasm::structures::Instance<
        ITExport,
        WITFunction<WB>,
        <WB as WasmBackend>::Memory,
        <WB as WasmBackend>::MemoryView,
        DelayedContextLifetime<WB>,
    > for ITInstance<WB>
{
    fn export(&self, _export_name: &str) -> Option<&ITExport> {
        // exports aren't used in this version of IT
        None
    }

    fn local_or_import<I: TypedIndex + LocalImportIndex>(
        &self,
        index: I,
    ) -> Option<&WITFunction<WB>> {
        self.funcs.get(&index.index())
    }

    fn memory(&self, index: usize) -> Option<&<WB as WasmBackend>::Memory> {
        if index >= self.memories.len() {
            None
        } else {
            Some(&self.memories[index])
        }
    }

    fn memory_view(&self, index: usize) -> Option<<WB as WasmBackend>::MemoryView> {
        if index >= self.memories.len() {
            return None;
        }

        let memory = &self.memories[index];
        let view: <WB as WasmBackend>::MemoryView = memory.view();
        Some(view)
    }

    fn wit_record_by_id(&self, index: u64) -> Option<&Arc<IRecordType>> {
        self.record_types_by_id.get(&index)
    }
}
