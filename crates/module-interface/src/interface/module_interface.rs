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

use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub arguments: Vec<(String, String)>,
    pub output_types: Vec<String>,
}

use std::cmp::Ordering;
impl PartialOrd for FunctionSignature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FunctionSignature {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.name < other.name {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct RecordField {
    pub name: String,
    pub ty: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct RecordType {
    pub name: String,
    pub id: u64,
    pub fields: Vec<RecordField>,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ModuleInterface {
    pub function_signatures: Vec<FunctionSignature>,
    // record types are guaranteed to be topological sorted
    pub record_types: Vec<RecordType>,
}

use std::fmt;

impl fmt::Display for FunctionSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use itertools::Itertools;

        let (designator, output) = match self.output_types.len() {
            0 => ("", String::new()),
            1 => ("->", &self.output_types[0]),
            _ => unimplemented!("more than 1 output type is unsupported"),
        };

        let args = self
            .arguments
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .format(", ");
        writeln!(f, "{}({}) {} {}", self.name, args, designator, output)
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "data {}:", self.name)?;

        for field in self.fields.iter() {
            writeln!(f, "  {}: {}", field.name, field.ty)?;
        }

        Ok(())
    }
}
