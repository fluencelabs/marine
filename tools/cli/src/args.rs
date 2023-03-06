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

use clap::App;
use clap::Arg;
use clap::SubCommand;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub const IN_WASM_PATH: &str = "in-wasm-path";
pub const IT_PATH: &str = "it-path";
pub const OUT_WASM_PATH: &str = "out-wasm-path";
pub const SERVICE_NAME: &str = "service-name";
pub const PROJECT_NAME: &str = "generate-project-name";
pub const SHOULD_INIT_OPTION: &str = "should-init";
pub const SERVICE_ID: &str = "service-id";

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
                .long("service")
                .help("optional service name"),
            Arg::with_name(SERVICE_ID)
                .required(false)
                .takes_value(true)
                .short("i")
                .long("id")
                .help("optional service id"),
        ])
}

pub fn build<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("Builds provided Rust project to Wasm")
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("[optional]... 'cargo build arguments'").multiple(true))
}

pub fn generate<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("generate")
        .about("Generates a template project for a Marine Wasm module")
        .args(&[
            Arg::with_name(PROJECT_NAME)
                .required(false)
                .takes_value(true)
                .short("n")
                .long("name")
                .help("a project name; if the name isn't in kebab-case, it'll be converted to kebab-case"),
            Arg::with_name(SHOULD_INIT_OPTION)
                .required(false)
                .takes_value(false)
                .short("i")
                .long("init")
                .help("generate the template into the current dir without creating a new one"),
        ]
        )
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
                .long("input")
                .help("a path to a Wasm file"),
            Arg::with_name(IT_PATH)
                .required(true)
                .takes_value(true)
                .short("w")
                .long("wit")
                .help("a path to a file with interface types"),
            Arg::with_name(OUT_WASM_PATH)
                .takes_value(true)
                .short("o")
                .long("output")
                .help("A path to the result Wasm file with set interface types. If absent, modifies input file."),
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
                .long("input")
                .help("a path to a Wasm file"),
            Arg::with_name(SDK_VERSION)
                .required(true)
                .takes_value(true)
                .short("v")
                .long("version")
                .help("a version of the sdk"),
            Arg::with_name(OUT_WASM_PATH)
                .takes_value(true)
                .short("o")
                .long("output")
                .help(
                    "A path to the result file with set version. If absent, modifies input file.",
                ),
        ])
}

pub fn show_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("it")
        .about("Shows IT of the provided Wasm file")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .index(1)
            .help("path to the Wasm file")])
}

pub fn show_manifest<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("info")
        .about("Shows manifest and sdk version of the provided Wasm file")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .index(1)
            .help("path to the Wasm file")])
}

pub fn repl<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("repl")
        .about("Starts Fluence application service REPL")
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::from_usage("[optional]... 'fluence repl arguments'").multiple(true))
}
