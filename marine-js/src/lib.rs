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

mod api;
mod global_state;
mod logger;

use crate::logger::MarineLogger;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() {
    log::set_boxed_logger(Box::new(MarineLogger::new(log::LevelFilter::Info))).unwrap();
    // Trace is required to accept all logs from a service.
    // Max level for this crate is set in MarineLogger constructor.
    log::set_max_level(log::LevelFilter::Trace);
}
