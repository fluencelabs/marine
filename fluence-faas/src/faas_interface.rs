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
pub struct FaaSInterface<'a> {
    pub modules: Vec<FaaSModuleInterface<'a>>,
}

#[derive(Debug)]
pub struct FaaSModuleInterface<'a> {
    pub name: &'a str,
    pub functions: Vec<fce::FCEFunction<'a>>,
}

impl<'a> fmt::Display for FaaSInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for module in self.modules.iter() {
            write!(f, "{}", module)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for FaaSModuleInterface<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;

        for function in self.functions.iter() {
            writeln!(
                f,
                "  pub fn {}({:?}) -> {:?}",
                function.name, function.inputs, function.outputs
            )?;
        }

        Ok(())
    }
}
