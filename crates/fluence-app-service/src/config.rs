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

use marine::generic::MarineConfig;

use std::path::PathBuf;
use marine_wasm_backend_traits::WasmBackend;

/// Describes behaviour of the Fluence AppService.
#[derive(Default)]
pub struct AppServiceConfig<WB: WasmBackend> {
    /// Used for preparing filesystem on the service initialization stage.
    pub service_working_dir: PathBuf,
    /// Location for /tmp and /local dirs.
    pub service_base_dir: PathBuf,
    pub marine_config: MarineConfig<WB>,
}
