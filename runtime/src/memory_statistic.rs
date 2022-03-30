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

use std::fmt;
use std::ops::Deref;

/// Contains module name and a size of its linear memory in bytes.
/// Please note that linear memory contains not only heap, but globals, shadow stack and so on.
/// Although it doesn't contain operand stack, additional runtime (Wasmer) structures,
/// and some other stuff, that should be count separately.
#[derive(Debug)]
pub struct ModuleMemoryStat<'module_name> {
    pub name: &'module_name str,
    pub memory_size: usize,
    // None if memory maximum wasn't set
    pub max_memory_size: Option<usize>,
}

pub struct MemoryStats<'module_name>(pub Vec<ModuleMemoryStat<'module_name>>);

impl<'module_name> ModuleMemoryStat<'module_name> {
    pub fn new(
        module_name: &'module_name str,
        memory_size: usize,
        max_memory_size: Option<usize>,
    ) -> Self {
        ModuleMemoryStat {
            name: module_name,
            memory_size,
            max_memory_size,
        }
    }
}

impl<'module_name> From<Vec<ModuleMemoryStat<'module_name>>> for MemoryStats<'module_name> {
    fn from(records: Vec<ModuleMemoryStat<'module_name>>) -> Self {
        Self(records)
    }
}

impl<'memory_size> Deref for MemoryStats<'memory_size> {
    type Target = [ModuleMemoryStat<'memory_size>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl fmt::Display for MemoryStats<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for record in self.0.iter() {
            let byte_size = bytesize::ByteSize::b(record.memory_size as u64);
            writeln!(f, "{} - {}", record.name, byte_size)?;
        }

        Ok(())
    }
}
