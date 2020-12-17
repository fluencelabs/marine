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

use fluence_faas::FaaSConfig;
use fluence_faas::FluenceFaaS;
use fluence_faas::HostImportDescriptor;
use fluence_faas::IValue;
use stepper_interface::StepperOutcome;

use std::path::PathBuf;
use std::path::Path;

const CALL_SERVICE_NAME: &str = "call_service";
const CURRENT_PEER_ID_ENV_NAME: &str = "CURRENT_PEER_ID";

unsafe impl Send for AquamarineVM {}

pub struct AquamarineVM {
    faas: FluenceFaaS,
    particle_data_store: PathBuf,
    /// file name of the AIR interpreter .wasm
    wasm_filename: String,
}

impl AquamarineVM {
    /// Create AquamarineVM with provided config.
    pub fn new(config: AquamarineVMConfig) -> Result<Self> {
        use AquamarineVMError::InvalidDataStorePath;

        let (wasm_dir, wasm_filename) = split_dirname(config.aquamarine_wasm_path)?;

        let faas_config = make_faas_config(
            wasm_dir,
            &wasm_filename,
            config.call_service,
            config.current_peer_id,
            config.logging_mask,
        );
        let faas = FluenceFaaS::with_raw_config(faas_config)?;

        let particle_data_store = config.particle_data_store;
        std::fs::create_dir_all(&particle_data_store)
            .map_err(|e| InvalidDataStorePath(e, particle_data_store.clone()))?;

        let aqua_vm = Self {
            faas,
            particle_data_store,
            wasm_filename,
        };

        Ok(aqua_vm)
    }

    pub fn call(
        &mut self,
        init_user_id: impl Into<String>,
        aqua: impl Into<String>,
        data: impl Into<Vec<u8>>,
        particle_id: impl AsRef<Path>,
    ) -> Result<StepperOutcome> {
        use AquamarineVMError::PersistDataError;

        let prev_data_path = self.particle_data_store.join(particle_id);
        // TODO: check for errors related to invalid file content (such as invalid UTF8 string)
        let prev_data = std::fs::read_to_string(&prev_data_path).unwrap_or_default();

        let prev_data = into_ibytes_array(prev_data.into_bytes());
        let data = into_ibytes_array(data.into());
        let args = vec![
            IValue::String(init_user_id.into()),
            IValue::String(aqua.into()),
            IValue::Array(prev_data),
            IValue::Array(data),
        ];

        let result =
            self.faas
                .call_with_ivalues(&self.wasm_filename, "invoke", &args, <_>::default())?;

        let outcome = StepperOutcome::from_ivalues(result)
            .map_err(AquamarineVMError::StepperResultDeError)?;

        // persist resulted data
        std::fs::write(&prev_data_path, &outcome.data)
            .map_err(|e| PersistDataError(e, prev_data_path))?;

        Ok(outcome)
    }
}

/// Splits given path into its directory and file stem
///
/// # Example
/// For path `/path/to/aquamarine.wasm` result will be `Ok(PathBuf(/path/to), "aquamarine")`
fn split_dirname(path: PathBuf) -> Result<(PathBuf, String)> {
    use AquamarineVMError::InvalidAquamarinePath;

    let metadata = path.metadata().map_err(|err| InvalidAquamarinePath {
        invalid_path: path.clone(),
        reason: "failed to get file's metadata (doesn't exist or invalid permissions)",
        io_error: Some(err),
    })?;

    if !metadata.is_file() {
        return Err(InvalidAquamarinePath {
            invalid_path: path,
            reason: "is not a file",
            io_error: None,
        });
    }

    let file_stem = path
        .file_stem()
        .expect("checked to be a file, file name must be defined");
    let file_stem = file_stem.to_string_lossy().into_owned();

    let mut path = path;
    // drop file name from path
    path.pop();

    Ok((path, file_stem))
}

fn make_faas_config(
    aquamarine_wasm_dir: PathBuf,
    aquamarine_wasm_file: &str,
    call_service: HostImportDescriptor,
    current_peer_id: String,
    logging_mask: i32,
) -> FaaSConfig {
    use fluence_faas::FaaSModuleConfig;
    use maplit::hashmap;

    let host_imports = hashmap! {
        String::from(CALL_SERVICE_NAME) => call_service
    };

    let mut aquamarine_module_config = FaaSModuleConfig {
        mem_pages_count: None,
        logger_enabled: true,
        host_imports,
        wasi: None,
        logging_mask,
    };

    let envs = hashmap! {
        CURRENT_PEER_ID_ENV_NAME.as_bytes().to_vec() => current_peer_id.into_bytes(),
    };
    aquamarine_module_config.extend_wasi_envs(envs);

    FaaSConfig {
        modules_dir: Some(aquamarine_wasm_dir),
        modules_config: vec![(String::from(aquamarine_wasm_file), aquamarine_module_config)],
        default_modules_config: None,
    }
}

fn into_ibytes_array(byte_array: Vec<u8>) -> Vec<IValue> {
    byte_array.into_iter().map(IValue::U8).collect()
}

// This API is intended for testing purposes
#[cfg(feature = "raw-aquamarine-vm-api")]
impl AquamarineVM {
    pub fn call_with_prev_data(
        &mut self,
        init_user_id: impl Into<String>,
        aqua: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
    ) -> Result<StepperOutcome> {
        let prev_data = into_ibytes_array(prev_data.into());
        let data = into_ibytes_array(data.into());
        let args = vec![
            IValue::String(init_user_id.into()),
            IValue::String(aqua.into()),
            IValue::Array(prev_data.into()),
            IValue::Array(data.into()),
        ];

        let result =
            self.faas
                .call_with_ivalues(&self.wasm_filename, "invoke", &args, <_>::default())?;

        let outcome = StepperOutcome::from_ivalues(result)
            .map_err(AquamarineVMError::StepperResultDeError)?;

        Ok(outcome)
    }
}
