/*
 * Copyright 2022 Fluence Labs Limited
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
use super::IRecordType;
use crate::MResult;
use crate::marine_js::Instance as WasmerInstance;
use crate::module::wit_store::WITStore;

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
pub(super) struct ITInstance {
    /// IT functions indexed by id.
    funcs: HashMap<usize, WITFunction>,

    /// IT memories.
    memories: Vec<WITMemory>,

    /// All record types that instance contains.
    record_types_by_id: MRecordTypes,
}

impl ITInstance {
    pub(super) fn new(wasm_instance: &WasmerInstance, wit: &MITInterfaces<'_>) -> MResult<Self> {
        let exports = Self::extract_raw_exports(wasm_instance, wit)?;
        let memories = Self::extract_memories(wasm_instance);

        let funcs = exports;

        let record_types_by_id = Self::extract_record_types(wit);

        Ok(Self {
            funcs,
            memories,
            record_types_by_id,
        })
    }

    fn extract_raw_exports(
        wasm_instance: &WasmerInstance,
        it: &MITInterfaces<'_>,
    ) -> MResult<HashMap<usize, WITFunction>> {
        let module_exports = &wasm_instance.exports;

        it.exports()
            .enumerate()
            .map(|(export_id, export)| {
                let export_func = module_exports.get(export.name)?;

                Ok((
                    export_id,
                    WITFunction::from_export(export_func, export.name.to_string())?,
                ))
            })
            .collect()
    }

    fn extract_memories(wasm_instance: &WasmerInstance) -> Vec<WITMemory> {
        use crate::marine_js::Export::Memory;

        let memories = wasm_instance
            .exports()
            .filter_map(|(_, export)| match export {
                Memory => Some(WITMemory::new(wasm_instance.module_name.clone())),
                _ => None,
            })
            .collect::<Vec<_>>();

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

impl wasm::structures::Instance<ITExport, WITFunction, WITMemory, WITMemoryView, WITStore>
    for ITInstance
{
    fn export(&self, _export_name: &str) -> Option<&ITExport> {
        // exports aren't used in this version of IT
        None
    }

    fn local_or_import<I: TypedIndex + LocalImportIndex>(&self, index: I) -> Option<&WITFunction> {
        self.funcs.get(&index.index())
    }

    fn memory(&self, index: usize) -> Option<&WITMemory> {
        if index >= self.memories.len() {
            None
        } else {
            Some(&self.memories[index])
        }
    }

    fn memory_view(&self, index: usize) -> Option<WITMemoryView> {
        if index >= self.memories.len() {
            return None;
        }

        let memory = &self.memories[index];

        Some(memory.view())
    }

    fn wit_record_by_id(&self, index: u64) -> Option<&Arc<IRecordType>> {
        self.record_types_by_id.get(&index)
    }
}
