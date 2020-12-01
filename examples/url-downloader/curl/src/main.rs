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

use fluence::fce;
use fluence::WasmLogger;

/// Log level can be changed by `RUST_LOG` env as well.
pub fn main() {
    WasmLogger::new()
        .with_log_level(log::Level::Info)
        .build()
        .unwrap();
}

#[fce]
pub fn download(url: String) -> String {
    log::info!("get called with url {}", url);

    unsafe { curl(url) }
}

/// Permissions in `Config.toml` should exist to use host functions.
#[fce]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: String) -> String;
}
