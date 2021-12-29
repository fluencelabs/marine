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
use super::IRecordType;
use crate::{js_log, MResult};

use marine_it_interfaces::MITInterfaces;
use marine_it_interfaces::ITAstType;
use wasmer_it::interpreter::wasm;
use wasmer_it::interpreter::wasm::structures::{LocalImportIndex, Memory, TypedIndex};
use crate::marine_js::{Instance as WasmerInstance, DynFunc};

use std::collections::HashMap;
use std::rc::Rc;

pub type MRecordTypes = HashMap<u64, Rc<IRecordType>>;

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
    pub(super) fn new(wasmer_instance: &WasmerInstance, wit: &MITInterfaces<'_>) -> MResult<Self> {
        let exports = Self::extract_raw_exports(wasmer_instance, wit)?;
        let memories = Self::extract_memories(wasmer_instance);
        js_log(&format!("executed Self::extract_memories"));

        let funcs = exports;

        let record_types_by_id = Self::extract_record_types(wit);
        js_log(&format!("executed Self::extract_record_types"));

        Ok(Self {
            funcs,
            memories,
            record_types_by_id,
        })
    }

    fn extract_raw_exports(
        wasmer_instance: &WasmerInstance,
        it: &MITInterfaces<'_>,
    ) -> MResult<HashMap<usize, WITFunction>> {
        //use wasmer_core::DynFunc;

        let module_exports = &wasmer_instance.exports;

        it.exports()
            .enumerate()
            .map(|(export_id, export)| {
                let export_func = module_exports.get(export.name)?;
                unsafe {
                    // TODO: refactor this with new Wasmer API when it is ready
                    // here it is safe because dyn func is never lives WITInstance
                    let export_func =
                        std::mem::transmute::<DynFunc<'_>, DynFunc<'static>>(export_func);
                    Ok((
                        export_id,
                        WITFunction::from_export(export_func, export.name.to_string())?,
                    ))
                }
            })
            .collect()
    }

    fn extract_memories(wasmer_instance: &WasmerInstance) -> Vec<WITMemory> {
        use crate::marine_js::Export::Memory;

        let memories = wasmer_instance
            .exports()
            .filter_map(|(_, export)| match export {
                Memory => Some(WITMemory::new(wasmer_instance.module_name.clone())),
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

impl wasm::structures::Instance<ITExport, WITFunction, WITMemory, WITMemoryView>
    for ITInstance
{
    fn export(&self, _export_name: &str) -> Option<&ITExport> {
        js_log(&format!("called ITInstance::export with {}", _export_name));
        // exports aren't used in this version of IT
        None
    }

    fn local_or_import<I: TypedIndex + LocalImportIndex>(&self, index: I) -> Option<&WITFunction> {
        js_log(&format!(
            "called ITInstance::local_or_import with {}",
            index.index()
        ));
        js_log(&format!(
            "ITInstance::export funcs size {}",
            self.funcs.len()
        ));
        self.funcs.get(&index.index())
    }

    fn memory(&self, index: usize) -> Option<&WITMemory> {
        js_log(&format!("called ITInstance::memory with {}", index));
        if index >= self.memories.len() {
            None
        } else {
            Some(&self.memories[index])
        }
    }

    fn memory_view(&self, index: usize) -> Option<WITMemoryView> {
        js_log(&format!("called ITInstance::memory_view with {}", index));
        if index >= self.memories.len() {
            return None;
        }

        let memory = &self.memories[index];

        Some(memory.view())
    }

    fn wit_record_by_id(&self, index: u64) -> Option<&Rc<IRecordType>> {
        js_log(&format!(
            "called ITInstance::wit_record_by_id with {}",
            index
        ));
        self.record_types_by_id.get(&index)
    }
}
