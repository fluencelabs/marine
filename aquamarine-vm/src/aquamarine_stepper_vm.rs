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

use crate::{Result, IType};
use crate::AquamarineVMError;
use crate::config::AquamarineVMConfig;

use fluence_faas::{FaaSConfig, HostExportedFunc};
use fluence_faas::FluenceFaaS;
use fluence_faas::HostImportDescriptor;
use fluence_faas::IValue;
use stepper_interface::StepperOutcome;

use std::path::PathBuf;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use parking_lot::{Mutex};

const CALL_SERVICE_NAME: &str = "call_service";
const CURRENT_PEER_ID_ENV_NAME: &str = "CURRENT_PEER_ID";

unsafe impl Send for SendSafeFaaS {}

struct SendSafeFaaS(FluenceFaaS);
impl Deref for SendSafeFaaS {
    type Target = FluenceFaaS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SendSafeFaaS {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParticleParameters {
    pub init_user_id: String,
    pub particle_id: String,
}

pub struct AquamarineVM {
    faas: SendSafeFaaS,
    particle_data_store: PathBuf,
    /// file name of the AIR interpreter .wasm
    wasm_filename: String,
    /// information about the particle that is being executed at the moment
    current_particle: Arc<Mutex<ParticleParameters>>,
}

impl AquamarineVM {
    /// Create AquamarineVM with provided config.
    pub fn new(config: AquamarineVMConfig) -> Result<Self> {
        use AquamarineVMError::InvalidDataStorePath;

        let current_particle: Arc<Mutex<ParticleParameters>> = <_>::default();
        let call_service = config.call_service;
        let params = current_particle.clone();
        let call_service_closure: HostExportedFunc = Box::new(move |_, ivalues: Vec<IValue>| {
            let params = {
                let lock = params.lock();
                lock.deref().clone()
            };
            call_service(params, ivalues)
        });
        let import_descriptor = HostImportDescriptor {
            host_exported_func: call_service_closure,
            argument_types: vec![IType::String, IType::String, IType::String, IType::String],
            output_type: Some(IType::Record(0)),
            error_handler: None,
        };

        let (wasm_dir, wasm_filename) = split_dirname(config.aquamarine_wasm_path)?;

        let faas_config = make_faas_config(
            wasm_dir,
            &wasm_filename,
            import_descriptor,
            config.current_peer_id,
            config.logging_mask,
        );
        let faas = FluenceFaaS::with_raw_config(faas_config)?;

        let particle_data_store = config.particle_data_store;
        std::fs::create_dir_all(&particle_data_store)
            .map_err(|e| InvalidDataStorePath(e, particle_data_store.clone()))?;

        let aqua_vm = Self {
            faas: SendSafeFaaS(faas),
            particle_data_store,
            wasm_filename,
            current_particle,
        };

        Ok(aqua_vm)
    }

    pub fn call(
        &mut self,
        init_user_id: impl Into<String>,
        aqua: impl Into<String>,
        data: impl Into<Vec<u8>>,
        particle_id: impl Into<String>,
    ) -> Result<StepperOutcome> {
        use AquamarineVMError::PersistDataError;

        let particle_id = particle_id.into();
        let init_user_id = init_user_id.into();

        let prev_data_path = self.particle_data_store.join(&particle_id);
        // TODO: check for errors related to invalid file content (such as invalid UTF8 string)
        let prev_data = std::fs::read_to_string(&prev_data_path).unwrap_or_default();

        let prev_data = into_ibytes_array(prev_data.into_bytes());
        let data = into_ibytes_array(data.into());
        let args = vec![
            IValue::String(init_user_id.clone()),
            IValue::String(aqua.into()),
            IValue::Array(prev_data),
            IValue::Array(data),
        ];

        self.update_current_particle(particle_id, init_user_id);

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

    pub fn update_current_particle(&self, particle_id: String, init_user_id: String) {
        let mut params = self.current_particle.lock();
        params.particle_id = particle_id;
        params.init_user_id = init_user_id;
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
