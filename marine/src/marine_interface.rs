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

use super::IRecordType;
use super::itype_text_view;
use crate::MarineModuleInterface;

use itertools::Itertools;
use serde::Serialize;

use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct MarineInterface<'a> {
    pub modules: HashMap<&'a str, MarineModuleInterface<'a>>,
}

impl<'a> fmt::Display for MarineInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_record_types(self.modules.values(), f)?;
        print_functions_sign(self.modules.iter(), f)
    }
}

fn print_record_types<'r>(
    modules: impl Iterator<Item = &'r MarineModuleInterface<'r>>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    use std::collections::HashSet;
    writeln!(f, "exported data types (combined from all modules):")?;
    let mut printed_record_types: HashSet<&IRecordType> = HashSet::new();

    for module in modules {
        for (_, record_type) in module.record_types.iter() {
            if !printed_record_types.insert(record_type) {
                // do not print record if it has been already printed
                continue;
            }

            writeln!(f, "data {}:", record_type.name)?;

            for field in record_type.fields.iter() {
                writeln!(
                    f,
                    "  {}: {}",
                    field.name,
                    itype_text_view(&field.ty, module.record_types)
                )?;
            }
        }
    }

    if printed_record_types.is_empty() {
        writeln!(f, "<no exported data types>")?;
    }

    writeln!(f)
}

fn print_functions_sign<'r>(
    modules: impl Iterator<Item = (&'r &'r str, &'r MarineModuleInterface<'r>)>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let modules = modules.sorted_by(|lhs, rhs| lhs.0.cmp(rhs.0));
    writeln!(f, "exported functions:")?;
    for (name, module_interface) in modules {
        writeln!(f, "{}:", *name)?;
        if module_interface.function_signatures.is_empty() {
            writeln!(f, "<no exported functions>")?;
            continue;
        }

        for function_signature in module_interface.function_signatures.iter() {
            write!(f, "  func {}(", function_signature.name)?;

            let args = function_signature
                .arguments
                .iter()
                .map(|arg| {
                    format!(
                        "{}: {}",
                        arg.name,
                        itype_text_view(&arg.ty, module_interface.record_types)
                    )
                })
                .join(", ");

            let outputs = &function_signature.outputs;
            if outputs.is_empty() {
                writeln!(f, "{})", args)?;
            } else if outputs.len() == 1 {
                writeln!(
                    f,
                    "{}) -> {}",
                    args,
                    itype_text_view(&outputs[0], module_interface.record_types)
                )?;
            } else {
                // At now, multi values aren't supported - only one output type is possible
                unimplemented!()
            }
        }
    }

    Ok(())
}
