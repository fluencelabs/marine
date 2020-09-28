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

use crate::Result;
use super::AppServiceError;

use fluence_faas::FluenceFaaS;
use fluence_faas::ModulesConfig;
use fluence_faas::IType;
use fluence_faas::IValue;

use std::convert::TryInto;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::ErrorKind;

const SERVICE_ID_ENV_NAME: &str = "service_id";

// TODO: remove and use mutex instead
unsafe impl Send for AquaStepper {}

pub struct AquaStepper {
    faas: FluenceFaaS,
    init_user_id: String,
}

pub struct ClosureDescriptor {
    pub name: String,
    pub closure: Box<dyn Fn(Vec<IValue>) -> IValue>,
    pub arguments_types: Vec<IType>,
    pub output_types: Vec<IType>,
}

impl AquaStepper {
    /// Create Service with given modules and service id.
    pub fn new<C, S>(config: C, closures: impl Iter<Item = &ClosureDescriptor>) -> Result<Self>
    where
        C: TryInto<ModulesConfig>,
        AppServiceError: From<C::Error>,
    {
        let config: ModulesConfig = config.try_into()?;

        let faas = FluenceFaaS::with_raw_config(config)?;

        Ok(Self { faas })
    }

    pub fn call(&mut self, args: serde_json::Value) -> Result<StepperOutcome> {
        self.faas
            .call_with_json("aquamarine", "invoke", args, <_>::default())
            .map_err(Into::into)
    }
}
