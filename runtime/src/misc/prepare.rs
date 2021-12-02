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

// Similar to
// https://github.com/paritytech/substrate/blob/master/srml/contracts/src/wasm/prepare.rs
// https://github.com/nearprotocol/nearcore/blob/master/runtime/near-vm-runner/src/prepare.rs

mod heap_base;

use super::PrepareResult;
use crate::misc::PrepareError;
use heap_base::get_heap_base;

use marine_utils::to_wasm_page_count_ceil;
use parity_wasm::builder;
use parity_wasm::elements;

// not all clangs versions emits __heap_base, and this consts is a temporary solution
// until node has a dedicated config for that
const DEFAULT_GLOBALS_SIZE: u32 = 50;

struct ModuleBootstrapper {
    module: elements::Module,
}

impl<'a> ModuleBootstrapper {
    fn init(module_code: &[u8]) -> PrepareResult<Self> {
        let module = elements::deserialize_buffer(module_code)?;

        Ok(Self { module })
    }

    fn set_max_heap_size(self, max_heap_size: u32) -> PrepareResult<Self> {
        use elements::{MemoryType, MemorySection};

        let Self { mut module } = self;
        let globals_size = get_heap_base(&module)
            .map(to_wasm_page_count_ceil)
            .unwrap_or(DEFAULT_GLOBALS_SIZE);
        let max_mem_size =
            globals_size
                .checked_add(max_heap_size)
                .ok_or(PrepareError::MemSizesOverflow {
                    globals_size,
                    max_heap_size,
                })?;

        // At now, there is could be only one memory section, so
        // it needs just to extract previous initial page count,
        // delete an old entry and add create a new one with updated limits
        let mem_initial_size = match module.memory_section_mut() {
            Some(section) => match section.entries_mut().pop() {
                Some(entry) => entry.limits().initial(),
                None => 0,
            },
            None => 0,
        };
        let mem_initial_size = std::cmp::min(mem_initial_size, max_mem_size);

        let memory_entry = MemoryType::new(mem_initial_size, Some(max_mem_size));
        let mut default_mem_section = MemorySection::default();

        module
            .memory_section_mut()
            .unwrap_or(&mut default_mem_section)
            .entries_mut()
            .push(memory_entry);

        let builder = builder::from_module(module);

        let module = builder.build();
        Ok(Self { module })
    }

    fn into_wasm(self) -> PrepareResult<Vec<u8>> {
        elements::serialize(self.module).map_err(Into::into)
    }
}

/// Prepares a Wasm module:
///   - extracts __heap_base global
///   - compute module max memory size by summation of heap and globals sizes
///   - set computed value as max memory page count of a module
pub(crate) fn prepare_module(module: &[u8], max_heap_size: u32) -> PrepareResult<Vec<u8>> {
    ModuleBootstrapper::init(module)?
        .set_max_heap_size(max_heap_size)?
        .into_wasm()
}
