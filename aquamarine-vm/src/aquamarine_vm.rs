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

use fluence_faas::FluenceFaaS;
use fluence_faas::RawModuleConfig;
use fluence_faas::RawModulesConfig;
use fluence_faas::HostImportDescriptor;

use std::path::PathBuf;
use std::collections::HashMap;

const AQUAMARINE_NAME: &str = "aquamarine";

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
    /// Create Aquamarine with path to the aquamarine.wasm and a list of host closures.
    pub fn new<P>(path: P, host_closures: Vec<(String, HostImportDescriptor)>) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let to_string =
            |path: &PathBuf| -> Option<_> { path.to_string_lossy().into_owned().into() };

        let mut stepper_config = RawModuleConfig::new(AQUAMARINE_NAME);
        stepper_config.logger_enabled = Some(true);

        let config = RawModulesConfig {
            modules_dir: to_string(&path.into()),
            service_base_dir: None,
            module: vec![stepper_config],
            default: None,
        };
        let mut closures = HashMap::new();
        closures.insert(String::from("aquamarine"), host_closures);

        let faas = FluenceFaaS::with_raw_config(config, closures)?;

        Ok(Self { faas })
    }

    pub fn call(&mut self, args: serde_json::Value) -> Result<StepperOutcome> {
        use fluence_faas::IValue;

        let mut result = self
            .faas
            .call_with_json("aquamarine", "invoke", args, <_>::default())?;

        let outcome = match result.remove(0) {
            IValue::Record(record_values) => {
                let mut record_values = record_values.into_vec();
                let data = match record_values.remove(0) {
                    IValue::String(str) => str,
                    _ => unreachable!(),
                };

                let next_peer_pks = match record_values.remove(0) {
                    IValue::Array(ar_values) => ar_values
                        .into_iter()
                        .map(|v| match v {
                            IValue::String(str) => str,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<String>>(),
                    _ => unreachable!(),
                };

                StepperOutcome {
                    data,
                    next_peer_pks,
                }
            }
            _ => unreachable!(),
        };

        Ok(outcome)
    }
}
