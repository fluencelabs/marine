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

/// Contains module name and a size of its heap in bytes.
#[derive(Debug)]
pub struct HeapStatisticRecord<'module_name> {
    pub module_name: &'module_name str,
    pub memory_size: usize,
}

pub struct HeapStatistic<'module_name>(pub Vec<HeapStatisticRecord<'module_name>>);

impl<'module_name> From<(&'module_name str, usize)> for HeapStatisticRecord<'module_name> {
    fn from(raw_record: (&'module_name str, usize)) -> Self {
        HeapStatisticRecord {
            module_name: raw_record.0,
            memory_size: raw_record.1,
        }
    }
}

impl<'module_name> From<Vec<HeapStatisticRecord<'module_name>>> for HeapStatistic<'module_name> {
    fn from(records: Vec<HeapStatisticRecord<'module_name>>) -> Self {
        Self(records)
    }
}

impl<'memory_size> Deref for HeapStatistic<'memory_size> {
    type Target = [HeapStatisticRecord<'memory_size>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl fmt::Display for HeapStatistic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for record in self.0.iter() {
            writeln!(f, "{} - {}", record.module_name, record.memory_size)?;
        }

        Ok(())
    }
}
