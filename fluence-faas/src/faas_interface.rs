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
use super::IFunctionArg;

use serde::Serialize;
use serde::Serializer;

use std::fmt;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub struct FaaSInterface<'a> {
    pub record_types: HashMap<u64, &'a IRecordType>,
    pub modules: HashMap<&'a str, HashMap<&'a str, FaaSFunctionSignature<'a>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FaaSFunctionSignature<'a> {
    pub arguments: &'a Vec<IFunctionArg>,
    pub output_types: &'a Vec<IType>,
}

impl<'a> fmt::Display for FaaSInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_text_view = |arg_ty: &IType| {
            match arg_ty {
                IType::Record(record_type_id) => {
                    // unwrap is safe because FaasInterface here is well-formed
                    // (it was checked on the module startup stage)
                    let record = self.record_types.get(record_type_id).unwrap();
                    record.name.clone()
                }
                t => format!("{:?}", t),
            }
        };

        for (_, record_type) in self.record_types.iter() {
            writeln!(f, "{} {{", record_type.name)?;

            for field in record_type.fields.iter() {
                writeln!(f, "  {}: {}", field.name, type_text_view(&field.ty))?;
            }
            writeln!(f, "}}")?;
        }

        if !self.record_types.is_empty() {
            writeln!(f)?;
        }

        for (name, functions) in self.modules.iter() {
            writeln!(f, "\n{}:", *name)?;

            for (name, signature) in functions.iter() {
                write!(f, "  fn {}(", name)?;

                let args = signature
                    .arguments
                    .iter()
                    .map(|arg| format!("{}: {}", arg.name, type_text_view(&arg.ty)))
                    .join(", ");

                if signature.output_types.is_empty() {
                    writeln!(f, "{})", args)?;
                } else if signature.output_types.len() == 1 {
                    writeln!(f, "{}) -> {}", args, type_text_view(&signature.output_types[0]))?;
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
        pub struct Function<'a> {
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
            pub functions: Vec<Function<'a>>,
        }

        #[derive(Serialize)]
        pub struct Interface<'a> {
            pub record_types: Vec<RecordType<'a>>,
            pub modules: Vec<Module<'a>>,
        }

        let record_types: Vec<_> = self
            .record_types
            .iter()
            .map(|(id, IRecordType { name, fields })| {
                let fields = fields.iter().map(|field| (&field.name, &field.ty)).collect::<Vec<_>>();

                RecordType {
                    name: name.as_str(),
                    id: *id,
                    fields,
                }
            })
            .collect();

        let modules: Vec<_> = self
            .modules
            .iter()
            .map(|(name, functions)| {
                let functions = functions
                    .iter()
                    .map(
                        |(
                            name,
                            FaaSFunctionSignature {
                                arguments,
                                output_types,
                            },
                        )| {
                            let arguments = arguments.iter().map(|arg| (&arg.name, &arg.ty)).collect();
                            Function {
                                name,
                                arguments,
                                output_types,
                            }
                        },
                    )
                    .collect();
                Module { name, functions }
            })
            .collect();

        Interface { record_types, modules }.serialize(serializer)
    }
}
