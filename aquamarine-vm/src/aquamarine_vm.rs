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

use crate::{Result, AquamarineVMError};
use crate::config::AquamarineVMConfig;

use fluence_faas::FluenceFaaS;

use std::collections::HashMap;

const AQUAMARINE_NAME: &str = "aquamarine";
const CALL_SERVICE_NAME: &str = "call_service";

unsafe impl Send for AquamarineVM {}

// delete this once aquamarine become public
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct StepperOutcome {
    pub data: String,
    pub next_peer_pks: Vec<String>,
}

pub struct AquamarineVM {
    faas: FluenceFaaS,
}

impl AquamarineVM {
    /// Create Aquamarine with provided config.
    pub fn new(config: AquamarineVMConfig) -> Result<Self> {
        use fluence_faas::FaaSConfig;
        use fluence_faas::FaaSModuleConfig;

        let mut host_imports = HashMap::new();
        host_imports.insert(String::from(CALL_SERVICE_NAME), config.call_service);

        let aquamarine_module_config = FaaSModuleConfig {
            mem_pages_count: None,
            logger_enabled: true,
            host_imports,
            wasi: None,
        };

        let mut modules_config = HashMap::new();
        modules_config.insert(String::from(AQUAMARINE_NAME), aquamarine_module_config);

        let faas_config = FaaSConfig {
            modules_dir: Some(config.aquamarine_wasm_path),
            modules_config,
            default_modules_config: None,
        };

        let faas = FluenceFaaS::with_raw_config(faas_config)?;

        Ok(Self { faas })
    }

    #[rustfmt::skip]
    pub fn call(&mut self, args: serde_json::Value) -> Result<StepperOutcome> {
        use fluence_faas::IValue;

        let mut result = self
            .faas
            .call_with_json("aquamarine", "invoke", args, <_>::default())?;

        let outcome = match result.remove(0) {
            IValue::Record(record_values) => {
                let mut record_values = record_values.into_vec();
                if record_values.len() != 2 {
                    return Err(AquamarineVMError::AquamarineResultError(format!("expected StepperOutcome struct with 2 fields, got {:?}", record_values)));
                }

                let data = match record_values.remove(0) {
                    IValue::String(str) => str,
                    v => return Err(AquamarineVMError::AquamarineResultError(format!("expected string for data, got {:?}", v))),
                };

                let next_peer_pks = match record_values.remove(0) {
                    IValue::Array(ar_values) => {
                        let array = ar_values
                            .into_iter()
                            .map(|v| match v {
                                IValue::String(str) => Ok(str),
                                v => Err(AquamarineVMError::AquamarineResultError(format!("expected string for next_peer_pks, got {:?}", v))),
                            })
                            .collect::<Result<Vec<String>>>()?;

                        Ok(array)
                    },
                    v => Err(AquamarineVMError::AquamarineResultError(format!("expected array for next_peer_pks, got {:?}", v))),
                }?;

                StepperOutcome {
                    data,
                    next_peer_pks,
                }
            }
            v => return Err(AquamarineVMError::AquamarineResultError(format!("expected record for StepperOutcome, got {:?}", v))),
        };

        Ok(outcome)
    }
}
