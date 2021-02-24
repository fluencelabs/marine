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

use crate::FCEResult;

use parity_wasm::{
    builder, elements,
    elements::{MemorySection, MemoryType},
};

struct ModuleBootstrapper {
    module: elements::Module,
}

impl<'a> ModuleBootstrapper {
    fn init(module_code: &[u8]) -> FCEResult<Self> {
        let module = elements::deserialize_buffer(module_code)?;

        Ok(Self { module })
    }

    fn set_mem_pages_count(self, mem_pages_count: u32) -> Self {
        let Self { mut module } = self;

        // At now, there is could be only one memory section, so
        // it needs just to extract previous initial page count,
        // delete an old entry and add create a new one with updated limits
        let mem_initial = match module.memory_section_mut() {
            Some(section) => match section.entries_mut().pop() {
                Some(entry) => entry.limits().initial(),
                None => 0,
            },
            None => 0,
        };

        let memory_entry = MemoryType::new(mem_initial, Some(mem_pages_count));
        let mut default_mem_section = MemorySection::default();

        module
            .memory_section_mut()
            .unwrap_or(&mut default_mem_section)
            .entries_mut()
            .push(memory_entry);

        let builder = builder::from_module(module);

        Self {
            module: builder.build(),
        }
    }

    fn into_wasm(self) -> FCEResult<Vec<u8>> {
        elements::serialize(self.module).map_err(Into::into)
    }
}

/// Prepares a Wasm module:
///   - set memory page count
pub(crate) fn prepare_module(module: &[u8], mem_pages_count: u32) -> FCEResult<Vec<u8>> {
    ModuleBootstrapper::init(module)?
        .set_mem_pages_count(mem_pages_count)
        .into_wasm()
}
