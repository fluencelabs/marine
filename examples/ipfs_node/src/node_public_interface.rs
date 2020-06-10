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

use std::fmt;

#[derive(Debug)]
pub struct NodePublicInterface<'a> {
    pub modules: Vec<NodeModulePublicInterface<'a>>,
}

#[derive(Debug)]
pub struct NodeModulePublicInterface<'a> {
    pub name: &'a str,
    pub functions: Vec<fce::NodeFunction<'a>>,
}

impl<'a> fmt::Display for NodePublicInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for module in self.modules.iter() {
            write!(f, "{}", module)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for NodeModulePublicInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.name)?;

        for function in self.functions.iter() {
            write!(f, "  pub fn {}({:?}) -> {:?}\n", function.name, function.inputs, function.outputs)?;
        }

        Ok(())
    }
}
