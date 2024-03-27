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

use marine_rs_sdk::CallParameters;
#[cfg(target_arch = "wasm32")]
use marine_rs_sdk::marine;
#[cfg(target_arch = "wasm32")]
use marine_rs_sdk::module_manifest;

#[cfg(target_arch = "wasm32")]
module_manifest!();

pub fn main() {}

#[marine]
#[cfg(target_arch = "wasm32")]
pub fn call_parameters() -> CallParameters {
    marine_rs_sdk::get_call_parameters()
}
