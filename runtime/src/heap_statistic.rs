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
}

pub struct MemoryStat<'module_name>(pub Vec<ModuleMemoryStat<'module_name>>);

impl<'module_name> From<(&'module_name str, usize)> for ModuleMemoryStat<'module_name> {
    fn from(raw_record: (&'module_name str, usize)) -> Self {
        ModuleMemoryStat {
            name: raw_record.0,
            memory_size: raw_record.1,
        }
    }
}

impl<'module_name> From<Vec<ModuleMemoryStat<'module_name>>> for MemoryStat<'module_name> {
    fn from(records: Vec<ModuleMemoryStat<'module_name>>) -> Self {
        Self(records)
    }
}

impl<'memory_size> Deref for MemoryStat<'memory_size> {
    type Target = [ModuleMemoryStat<'memory_size>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl fmt::Display for MemoryStat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for record in self.0.iter() {
            let byte_size = bytesize::ByteSize::b(record.memory_size as u64);
            writeln!(f, "{} - {}", record.name, byte_size)?;
        }

        Ok(())
    }
}
