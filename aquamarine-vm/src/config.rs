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

use std::path::PathBuf;

use crate::aquamarine_stepper_vm::ParticleParams;
use crate::IValue;

type CallServiceClosure = Box<dyn Fn(ParticleParams, Vec<IValue>) -> Option<IValue> + 'static>;

/// Describes behaviour of the Aquamarine VM stepper.
pub struct AquamarineVMConfig {
    /// Path to a aquamarine stepper Wasm file.
    pub aquamarine_wasm_path: PathBuf,

    /// Descriptor of a closure that will be invoked on call_service call from Aquamarine stepper.
    pub call_service: CallServiceClosure,

    /// Current peer id.
    pub current_peer_id: String,

    /// Path to a folder contains prev data.
    /// AquamarineVM uses it to store data obtained after stepper execution, and load it as a prev_data by particle_id.
    pub particle_data_store: PathBuf,

    /// Mask used to filter logs, for details see `log_utf8_string` in fluence-faas.
    pub logging_mask: i32,
}
