/*
 * Marine WebAssembly runtime
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

use marine_rs_sdk::WasmLoggerBuilder;
use marine_rs_sdk::MountedBinaryResult;

module_manifest!();

/// Log level can be changed by `RUST_LOG` env as well.
pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn download(url: String) -> String {
    log::info!("download called with url {}\n", url);

    let result = curl(vec![url]);

    String::from_utf8(result.stdout).unwrap()
}

/// Permissions in `Config.toml` should exist to use host functions.
#[marine]
#[host_import]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}
