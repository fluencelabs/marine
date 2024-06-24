/*
 * Copyright (C) 2024  Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub struct GreetingRecord {
    pub str: String,
    pub num: i32,
}

#[marine]
pub fn greeting_record() -> GreetingRecord {
    GreetingRecord {
        str: String::from("Hello, world!"),
        num: 42,
    }
}

#[marine]
pub fn log_info() {
    log::info!("info");
}

#[marine]
pub fn log_warn() {
    log::warn!("warn");
}

#[marine]
pub fn log_error() {
    log::error!("error");
}

#[marine]
pub fn log_debug() {
    log::debug!("debug");
}

#[marine]
pub fn log_trace() {
    log::trace!("trace");
}

#[marine]
pub fn void_fn() {}
