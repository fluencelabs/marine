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

use marine_wasm_backend_traits::MemoryAllocationStats;

use serde::Serialize;
use serde::Deserialize;

use std::fmt;

/// Contains module name and a size of its linear memory in bytes.
/// Please note that linear memory contains not only heap, but globals, shadow stack and so on.
/// Although it doesn't contain operand stack, additional runtime (Wasmer) structures,
/// and some other stuff, that should be count separately.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModuleMemoryStat<'module_name> {
    pub name: &'module_name str,
    pub memory_size: usize,
}

pub struct MemoryStats<'module_name> {
    pub modules: Vec<ModuleMemoryStat<'module_name>>,
    pub allocation_stats: Option<MemoryAllocationStats>,
}

impl<'module_name> MemoryStats<'module_name> {
    pub fn new(
        modules: Vec<ModuleMemoryStat<'module_name>>,
        allocation_stats: Option<MemoryAllocationStats>,
    ) -> Self {
        Self {
            modules,
            allocation_stats,
        }
    }
}

impl<'module_name> ModuleMemoryStat<'module_name> {
    pub fn new(module_name: &'module_name str, memory_size: usize) -> Self {
        ModuleMemoryStat {
            name: module_name,
            memory_size,
        }
    }
}

impl fmt::Display for MemoryStats<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for module in self.modules.iter() {
            let memory_size = bytesize::ByteSize::b(module.memory_size as u64);
            writeln!(f, "{} - {}", module.name, memory_size)?;
        }

        match &self.allocation_stats {
            None => writeln!(
                f,
                "Allocation rejects - value is not recorded by current wasm backend"
            )?,
            Some(stats) => writeln!(f, "Allocation rejects - {}", stats.allocation_rejects)?,
        }

        Ok(())
    }
}
