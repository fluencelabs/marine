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
use crate::FaaSFunctionSignature;

use serde::Serialize;
use serde::Serializer;

use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FaaSInterface<'a> {
    pub modules: HashMap<&'a str, FaaSModuleInterface<'a>>,
}

impl<'a> fmt::Display for FaaSInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_text_view = |arg_ty: &IType, record_types: &HashMap<u64, IRecordType>| {
            match arg_ty {
                IType::Record(record_type_id) => {
                    // unwrap is safe because FaaSInterface here is well-formed
                    // (it was checked on the module startup stage)
                    let record = record_types.get(record_type_id).unwrap();
                    record.name.clone()
                }
                t => format!("{:?}", t),
            }
        };

        let mut printed_record_types: HashSet<&IRecordType> = HashSet::new();

        for (_, module_interface) in self.modules.iter() {
            for (_, record_type) in module_interface.record_types.iter() {
                if printed_record_types.insert(record_type) {
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

                let outputs = function_signature.outputs;
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

impl<'a> Serialize for FaaSInterface<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        pub struct FunctionSignature<'a> {
            pub name: &'a str,
            pub arguments: Vec<(&'a String, &'a IType)>,
            pub output_types: &'a Vec<IType>,
        }

        #[derive(Serialize)]
        pub struct RecordType<'a> {
            pub name: &'a str,
            pub id: u64,
            pub fields: Vec<(&'a String, &'a IType)>,
        }

        #[derive(Serialize)]
        pub struct Module<'a> {
            pub name: &'a str,
            pub function_signatures: Vec<FunctionSignature<'a>>,
            pub record_types: Vec<RecordType<'a>>,
        }

        #[derive(Serialize)]
        pub struct Interface<'a> {
            pub modules: Vec<Module<'a>>,
        }

        fn serialize_function_signature<'a>(
            signature: &'a FaaSFunctionSignature<'_>,
        ) -> FunctionSignature<'a> {
            let arguments = signature
                .arguments
                .iter()
                .map(|arg| (&arg.name, &arg.ty))
                .collect();

            FunctionSignature {
                name: signature.name,
                arguments,
                output_types: signature.outputs,
            }
        }

        fn serialize_record_type<'a, 'b>(record: (&'a u64, &'b IRecordType)) -> RecordType<'b> {
            let fields = record
                .1
                .fields
                .iter()
                .map(|field| (&field.name, &field.ty))
                .collect::<Vec<_>>();

            RecordType {
                name: record.1.name.as_str(),
                id: *record.0,
                fields,
            }
        }

        let modules: Vec<_> = self
            .modules
            .iter()
            .map(|(name, interface)| {
                let function_signatures = interface
                    .function_signatures
                    .iter()
                    .map(serialize_function_signature)
                    .collect();

                let record_types: Vec<_> = interface
                    .record_types
                    .iter()
                    .map(serialize_record_type)
                    .collect();

                Module {
                    name,
                    function_signatures,
                    record_types,
                }
            })
            .collect();

        Interface { modules }.serialize(serializer)
    }
}
