/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
            0 => ("", ""),
            1 => ("->", self.output_types[0].as_str()),
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
