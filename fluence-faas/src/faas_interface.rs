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

use super::IType;
use super::IRecordType;
use crate::FaaSModuleInterface;

use fce::RecordTypes;
use itertools::Itertools;
use serde::Serialize;

use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct FaaSInterface<'a> {
    pub modules: HashMap<&'a str, FaaSModuleInterface<'a>>,
}

impl<'a> fmt::Display for FaaSInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printed_record_types: HashSet<&IRecordType> = HashSet::new();

        for (_, module_interface) in self.modules.iter() {
            for (_, record_type) in module_interface.record_types.iter() {
                if !printed_record_types.insert(record_type) {
                    // do not print record if it has been already printed
                    continue;
                }

                writeln!(f, "{} {{", record_type.name)?;

                for field in record_type.fields.iter() {
                    writeln!(
                        f,
                        "  {}: {}",
                        field.name,
                        type_text_view(&field.ty, &module_interface.record_types)
                    )?;
                }

                writeln!(f, "}}")?;
            }
        }

        for (name, module_interface) in self.modules.iter() {
            writeln!(f, "\n{}:", *name)?;

            for function_signature in module_interface.function_signatures.iter() {
                write!(f, "  fn {}(", function_signature.name)?;

                let args = function_signature
                    .arguments
                    .iter()
                    .map(|arg| {
                        format!(
                            "{}: {}",
                            arg.name,
                            type_text_view(&arg.ty, &module_interface.record_types)
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
                        type_text_view(&outputs[0], &module_interface.record_types)
                    )?;
                } else {
                    // At now, multi values aren't supported - only one output type is possible
                    unimplemented!()
                }
            }
        }

        Ok(())
    }
}

fn type_text_view(arg_ty: &IType, record_types: &RecordTypes) -> String {
    match arg_ty {
        IType::Record(record_type_id) => {
            // unwrap is safe because FaaSInterface here is well-formed
            // (it was checked on the module startup stage)
            let record = record_types.get(record_type_id).unwrap();
            record.name.clone()
        }
        IType::Array(array_ty) => format!("Array<{}>", type_text_view(array_ty, record_types)),
        t => format!("{:?}", t),
    }
}
