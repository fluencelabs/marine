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

use serde::Serialize;
use serde::Serializer;

use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct FaaSInterface<'a> {
    pub modules: HashMap<&'a str, HashMap<&'a str, FaaSFunctionSignature<'a>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FaaSFunctionSignature<'a> {
    pub input_types: &'a Vec<IType>,
    pub output_types: &'a Vec<IType>,
}

impl<'a> fmt::Display for FaaSInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, functions) in self.modules.iter() {
            writeln!(f, "{}", *name)?;

            for (name, signature) in functions.iter() {
                writeln!(
                    f,
                    "  pub fn {}({:?}) -> {:?}",
                    name, signature.input_types, signature.output_types
                )?;
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
            pub input_types: &'a Vec<IType>,
            pub output_types: &'a Vec<IType>,
        }

        #[derive(Serialize)]
        pub struct Module<'a> {
            pub name: &'a str,
            pub functions: Vec<Function<'a>>,
        }

        #[derive(Serialize)]
        pub struct Interface<'a> {
            pub modules: Vec<Module<'a>>,
        }

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
                                input_types,
                                output_types,
                            },
                        )| {
                            Function {
                                name,
                                input_types,
                                output_types,
                            }
                        },
                    )
                    .collect();
                Module { name, functions }
            })
            .collect();

        Interface { modules }.serialize(serializer)
    }
}
