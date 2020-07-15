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

use clap::{App, Arg, SubCommand};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub const IN_WASM_PATH: &str = "in-wasm-path";

pub fn build<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("build provided Rust project to Wasm")
        .setting(clap::AppSettings::AllowExternalSubcommands)
}

pub fn show_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .about("show WIT in provided Wasm file")
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .short("i")
            .help("path to the Wasm file")])
}
