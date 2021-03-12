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
pub const WIT_PATH: &str = "wit-path";
pub const OUT_WASM_PATH: &str = "out-wasm-path";

pub fn build<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("Build provided Rust project to Wasm")
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("[optional]... 'cargo build arguments'").multiple(true))
}

pub fn embed_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("embed")
        .about("Embed IT to the provided Wasm file")
        .args(&[
            Arg::with_name(IN_WASM_PATH)
                .required(true)
                .takes_value(true)
                .short("i")
                .help("path to the wasm file"),
            Arg::with_name(WIT_PATH)
                .required(true)
                .takes_value(true)
                .short("w")
                .help("path to file with WIT"),
            Arg::with_name(OUT_WASM_PATH)
                .takes_value(true)
                .short("o")
                .help("path to result file with embedded WIT"),
        ])
}

pub fn show_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("it")
        .about("Show IT of the provided Wasm file")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .short("i")
            .help("path to the Wasm file")])
}

pub fn show_manifest<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("info")
        .about("Show manifest and sdk version of the provided Wasm file")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .short("i")
            .help("path to the Wasm file")])
}

pub fn repl<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("repl")
        .about("Start Fluence application service REPL")
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("[optional]... 'fluence repl arguments'").multiple(true))
}
