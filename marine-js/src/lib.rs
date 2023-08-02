/*
 * Copyright 2023 Fluence Labs Limited
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
    init_tracing();
}


#[allow(dead_code)]
pub fn init_tracing() {
    let config = tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::TRACE)
            .set_report_logs_in_timings(true)
            .set_console_config(tracing_wasm::ConsoleConfig::ReportWithoutConsoleColor)
            .build();

    tracing_wasm::set_as_global_default_with_config(config)
}

