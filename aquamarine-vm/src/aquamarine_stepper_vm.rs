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
use crate::config::AquamarineVMConfig;
use crate::stepper_outcome::StepperOutcome;
use crate::stepper_outcome::RawStepperOutcome;

use fluence_faas::FaaSConfig;
use fluence_faas::FluenceFaaS;
use fluence_faas::HostImportDescriptor;
use fluence_faas::IValue;

use std::convert::TryInto;
use std::path::PathBuf;
use std::path::Path;

const AQUAMARINE_WASM_FILE_NAME: &str = "aquamarine";
const CALL_SERVICE_NAME: &str = "call_service";
const CURRENT_PEER_ID_ENV_NAME: &str = "CURRENT_PEER_ID";

unsafe impl Send for AquamarineVM {}

pub struct AquamarineVM {
    faas: FluenceFaaS,
    particle_data_store: PathBuf,
}

impl AquamarineVM {
    /// Create AquamarineVM with provided config.
    pub fn new(config: AquamarineVMConfig) -> Result<Self> {
        use AquamarineVMError::InvalidDataStorePath;

        let faas_config = Self::make_faas_config(
            config.aquamarine_wasm_path,
            config.call_service,
            config.current_peer_id,
        );
        let faas = FluenceFaaS::with_raw_config(faas_config)?;

        let particle_data_store = config.particle_data_store;
        std::fs::create_dir_all(&particle_data_store)
            .map_err(|e| InvalidDataStorePath(e, particle_data_store.clone()))?;

        Ok(Self {
            faas,
            particle_data_store,
        })
    }

    pub fn call(
        &mut self,
        init_user_id: impl Into<String>,
        aqua: impl Into<String>,
        data: impl Into<String>,
        particle_id: impl AsRef<Path>,
    ) -> Result<StepperOutcome> {
        use AquamarineVMError::PersistDataError;

        let prev_data_path = self.particle_data_store.join(particle_id);
        // TODO: check for errors related to invalid file content (such as invalid UTF8 string)
        let prev_data = std::fs::read_to_string(&prev_data_path).unwrap_or(String::from("[]"));
        let args = vec![
            IValue::String(init_user_id.into()),
            IValue::String(aqua.into()),
            IValue::String(prev_data.into()),
            IValue::String(data.into()),
        ];

        let result = self.faas.call_with_ivalues(
            AQUAMARINE_WASM_FILE_NAME,
            "invoke",
            &args,
            <_>::default(),
        )?;

        let raw_outcome = Self::make_raw_outcome(result)?;
        std::fs::write(&prev_data_path, &raw_outcome.data)
            .map_err(|e| PersistDataError(e, prev_data_path))?;

        raw_outcome.try_into()
    }

    fn make_faas_config(
        aquamarine_wasm_path: PathBuf,
        call_service: HostImportDescriptor,
        current_peer_id: String,
    ) -> FaaSConfig {
        use maplit::hashmap;

        let make_faas_module_config = |call_service: HostImportDescriptor| {
            use fluence_faas::FaaSModuleConfig;

            let host_imports = hashmap! {
                String::from(CALL_SERVICE_NAME) => call_service
            };

            FaaSModuleConfig {
                mem_pages_count: None,
                logger_enabled: true,
                host_imports,
                wasi: None,
                logging_mask: i64::max_value(),
            }
        };

        let mut aquamarine_module_config = make_faas_module_config(call_service);

        let envs = hashmap! {
            CURRENT_PEER_ID_ENV_NAME.as_bytes().to_vec() => current_peer_id.into_bytes(),
        };
        aquamarine_module_config.extend_wasi_envs(envs);

        let mut aquamarine_wasm_dir = aquamarine_wasm_path;
        // faas config requires a path to the directory with Wasm modules
        aquamarine_wasm_dir.pop();

        FaaSConfig {
            modules_dir: Some(aquamarine_wasm_dir),
            modules_config: vec![(
                String::from(AQUAMARINE_WASM_FILE_NAME),
                aquamarine_module_config,
            )],
            default_modules_config: None,
        }
    }

    fn make_raw_outcome(mut result: Vec<IValue>) -> Result<RawStepperOutcome> {
        use AquamarineVMError::AquamarineResultError as ResultError;

        match result.remove(0) {
            IValue::Record(record_values) => {
                let mut record_values = record_values.into_vec();
                if record_values.len() != 3 {
                    return Err(ResultError(format!(
                        "expected StepperOutcome struct with 3 fields, got {:?}",
                        record_values
                    )));
                }

                let ret_code = match record_values.remove(0) {
                    IValue::S32(ret_code) => ret_code,
                    v => {
                        return Err(ResultError(format!(
                            "expected i32 for ret_code, got {:?}",
                            v
                        )))
                    }
                };

                let data = match record_values.remove(0) {
                    IValue::String(str) => str,
                    v => {
                        return Err(ResultError(format!(
                            "expected string for data, got {:?}",
                            v
                        )))
                    }
                };

                let next_peer_pks = match record_values.remove(0) {
                    IValue::Array(ar_values) => {
                        let array = ar_values
                            .into_iter()
                            .map(|v| match v {
                                IValue::String(str) => Ok(str),
                                v => Err(ResultError(format!(
                                    "expected string for next_peer_pks, got {:?}",
                                    v
                                ))),
                            })
                            .collect::<Result<Vec<String>>>()?;

                        Ok(array)
                    }
                    v => Err(ResultError(format!(
                        "expected array for next_peer_pks, got {:?}",
                        v
                    ))),
                }?;

                Ok(RawStepperOutcome {
                    ret_code,
                    data,
                    next_peer_pks,
                })
            }
            v => {
                return Err(ResultError(format!(
                    "expected record for StepperOutcome, got {:?}",
                    v
                )))
            }
        }
    }
}
