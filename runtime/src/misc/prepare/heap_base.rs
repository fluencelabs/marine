/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::misc::HeapBaseError::*;

use parity_wasm::elements;

const HEAP_BASE_NAME: &str = "__heap_base";
const WASM_PAGE_SIZE: u32 = 65356;
type HResult<T> = std::result::Result<T, crate::misc::HeapBaseError>;

pub(super) fn get_heap_base(wasm_module: &elements::Module) -> HResult<u32> {
    let heap_base_index = find_global_name_index(wasm_module, HEAP_BASE_NAME)?;
    let global_entry = find_global_by_index(wasm_module, heap_base_index as usize)?;
    let heap_base = u32_from_global_entry(global_entry)?;

    // heap_base is an offset and it's need to be converted to wasm pages count first
    Ok(offset_to_page_count_ceil(heap_base))
}

fn find_global_name_index(wasm_module: &elements::Module, name: &str) -> HResult<u32> {
    use elements::Internal;

    wasm_module
        .export_section()
        .and_then(|export_section| {
            export_section
                .entries()
                .iter()
                .find_map(|entry| match entry.internal() {
                    Internal::Global(index) if entry.field() == name => Some(*index),
                    _ => None,
                })
        })
        .ok_or(ExportNotFound)
}

fn find_global_by_index(
    wasm_module: &elements::Module,
    index: usize,
) -> HResult<&elements::GlobalEntry> {
    wasm_module
        .global_section()
        .and_then(|section| section.entries().get(index))
        .ok_or(ExportNotFound)
}

fn u32_from_global_entry(global_entry: &elements::GlobalEntry) -> HResult<u32> {
    use elements::{Instruction, ValueType};

    let entry_type = global_entry.global_type().content_type();
    if !matches!(entry_type, ValueType::I32) {
        return Err(WrongType);
    }

    let init_expr = global_entry.init_expr().code();
    // check that initialization expression consists of two instructions:
    //  i32.const <heap_base>
    //  end
    if init_expr.len() != 2 {
        return Err(InitializationNotI32Const);
    }

    match (&init_expr[0], &init_expr[1]) {
        (Instruction::I32Const(value), Instruction::End) => Ok(*value as u32),
        _ => Err(InitializationNotI32Const),
    }
}

fn offset_to_page_count_ceil(offset: u32) -> u32 {
    match offset {
        0 => 0,
        // ceiling
        n => 1 + (n - 1) / WASM_PAGE_SIZE,
    }
}
