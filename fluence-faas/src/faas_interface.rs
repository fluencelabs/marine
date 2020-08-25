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

#[derive(Debug, PartialEq, Clone)]
pub struct FaaSInterface<'a> {
    pub record_types: Vec<&'a IRecordType>,
    pub modules: HashMap<&'a str, HashMap<&'a str, FaaSFunctionSignature<'a>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FaaSFunctionSignature<'a> {
    pub arguments: &'a Vec<IFunctionArg>,
    pub output_types: &'a Vec<IType>,
}

impl<'a> fmt::Display for FaaSInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for record_type in &self.record_types {
            writeln!(f, "{} {{", record_type.name)?;

            for field in record_type.fields.iter() {
                writeln!(f, "  {}: {:?}", field.name, field.ty)?;
            }
            writeln!(f, "}}")?;
        }

        for (name, functions) in self.modules.iter() {
            writeln!(f, "{}", *name)?;

            for (name, signature) in functions.iter() {
                write!(f, "  pub fn {}(", name)?;

                for arg in signature.arguments {
                    write!(f, "{}, {:?}", arg.name, arg.ty)?;
                }
                write!(f, "  pub fn ) -> {:?}", signature.output_types)?;
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
            .map(|IRecordType { name, fields }| {
                let fields = fields
                    .iter()
                    .map(|field| (&field.name, &field.ty))
                    .collect::<Vec<_>>();

                RecordType {
                    name: name.as_str(),
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
                            let arguments =
                                arguments.iter().map(|arg| (&arg.name, &arg.ty)).collect();
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

        Interface {
            record_types,
            modules,
        }
        .serialize(serializer)
    }
}
