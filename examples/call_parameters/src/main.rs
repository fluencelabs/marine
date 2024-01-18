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

#[cfg(target_arch = "wasm32")]
use marine_rs_sdk::marine;
#[cfg(target_arch = "wasm32")]
use marine_rs_sdk::module_manifest;

#[cfg(target_arch = "wasm32")]
module_manifest!();

pub fn main() {}

#[marine]
#[cfg(target_arch = "wasm32")]
pub fn call_parameters() -> String {
    let cp = marine_rs_sdk::get_call_parameters();
    format!(
        "init_peer_id: {}, service_id: {}, service_creator_peer_id: {}, host_id: {}, worker_id: {}, particle_id: {}, tetraplets: {:?}",
        cp.init_peer_id,
        cp.service_id,
        cp.service_creator_peer_id,
        cp.host_id,
        cp.worker_id,
        cp.particle_id,
        cp.tetraplets
    )
}
