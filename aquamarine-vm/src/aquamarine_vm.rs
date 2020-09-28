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
use crate::AquamarineVMError;

use fluence_faas::FluenceFaaS;
use fluence_faas::ModulesConfig;
use fluence_faas::HostImportDescriptor;

use std::convert::TryInto;
use std::collections::HashMap;

// TODO: remove and use mutex instead
unsafe impl Send for AquamarineVM {}

// delete this once aquamarine become public
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StepperOutcome {
    pub data: String,
    pub next_peer_pks: Vec<String>,
}

pub struct AquamarineVM {
    faas: FluenceFaaS,
}

impl AquamarineVM {
    /// Create Service with given modules and service id.
    pub fn new<C>(config: C, host_closures: Vec<(String, HostImportDescriptor)>) -> Result<Self>
    where
        C: TryInto<ModulesConfig>,
        AquamarineVMError: From<C::Error>,
    {
        let config: ModulesConfig = config.try_into()?;
        let mut closures = HashMap::new();
        closures.insert(String::from("aquamarine"), host_closures);

        let faas = FluenceFaaS::with_raw_config(config, closures)?;

        Ok(Self { faas })
    }

    pub fn call(&mut self, args: serde_json::Value) -> Result<StepperOutcome> {
        let result = self
            .faas
            .call_with_json("aquamarine", "invoke", args, <_>::default())?;

        Ok(fluence_faas::from_interface_values::<StepperOutcome>(&result).unwrap())
    }
}
