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
pub const IT_PATH: &str = "it-path";
pub const OUT_WASM_PATH: &str = "out-wasm-path";
pub const SERVICE_NAME: &str = "service-name";

pub const SDK_VERSION: &str = "sdk-version";

pub fn aqua<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("aqua")
        .about("Shows data types of provided module in a format suitable for Aqua")
        .args(&[
            Arg::with_name(IN_WASM_PATH)
                .required(true)
                .takes_value(true)
                .index(1)
                .help("a path to a Wasm file"),
            Arg::with_name(SERVICE_NAME)
                .required(false)
                .takes_value(true)
                .short("s")
                .help("optional service name"),
        ])
}

pub fn build<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("Builds provided Rust project to Wasm")
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("[optional]... 'cargo build arguments'").multiple(true))
}

pub fn set<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("set")
        .about("Sets interface types and version to the provided Wasm file")
        .subcommand(set_it())
        .subcommand(set_version())
}

fn set_it<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("it")
        .about("Sets given interface types to the provided Wasm file")
        .args(&[
            Arg::with_name(IN_WASM_PATH)
                .required(true)
                .takes_value(true)
                .short("i")
                .help("a path to a Wasm file"),
            Arg::with_name(IT_PATH)
                .required(true)
                .takes_value(true)
                .short("w")
                .help("a path to a file with interface types"),
            Arg::with_name(OUT_WASM_PATH)
                .takes_value(true)
                .short("o")
                .help("a path to the result Wasm file with set interface types"),
        ])
}

fn set_version<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("version")
        .about("Sets given sdk version to the provided Wasm file")
        .args(&[
            Arg::with_name(IN_WASM_PATH)
                .required(true)
                .takes_value(true)
                .short("i")
                .help("a path to a Wasm file"),
            Arg::with_name(SDK_VERSION)
                .required(true)
                .takes_value(true)
                .short("v")
                .help("a version of the sdk"),
            Arg::with_name(OUT_WASM_PATH)
                .takes_value(true)
                .short("o")
                .help("a path to the result file with set version"),
        ])
}

pub fn show_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("it")
        .about("Shows IT of the provided Wasm file")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .short("i")
            .help("path to the Wasm file")])
}

pub fn show_manifest<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("info")
        .about("Shows manifest and sdk version of the provided Wasm file")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .short("i")
            .help("path to the Wasm file")])
}

pub fn repl<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("repl")
        .about("Starts Fluence application service REPL")
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("[optional]... 'fluence repl arguments'").multiple(true))
}
