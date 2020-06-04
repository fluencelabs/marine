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

mod custom;
mod embedder;
mod extracter;

use clap::{App, AppSettings, Arg, SubCommand};
use embedder::Config;
use failure::err_msg;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const IN_WASM_PATH: &str = "in-wasm-path";
const WIT_PATH: &str = "wit-path";
const OUT_WASM_PATH: &str = "out-wasm-path";

fn embed_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("embed")
        .about("embed WIT to provided Wasm file")
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

fn show_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .about("show WIT in provided Wasm file")
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .short("i")
            .help("path to the wasm file")])
}

pub fn main() -> Result<(), exitfailure::ExitFailure> {
    let app = App::new("CLI tool for embedding WIT to provided Wasm file")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(embed_wit())
        .subcommand(show_wit());

    match app.get_matches().subcommand() {
        ("embed", Some(arg)) => {
            let in_wasm_path = arg.value_of(IN_WASM_PATH).unwrap();
            let wit_path = arg.value_of(WIT_PATH).unwrap();
            let out_wasm_path = match arg.value_of(OUT_WASM_PATH) {
                Some(path) => path,
                None => in_wasm_path,
            };

            let wit = String::from_utf8(std::fs::read(wit_path)?).unwrap();

            let options = Config {
                in_wasm_path: PathBuf::from(in_wasm_path),
                wit,
                out_wasm_path: PathBuf::from(out_wasm_path),
            };

            match embedder::embed_wit(&options) {
                Ok(_) => {
                    println!("WIT successfully embedded");
                }
                Err(e) => {
                    println!("{}", e);
                }
            };

            Ok(())
        }
        ("show", Some(arg)) => {
            let wasm_path = arg.value_of(IN_WASM_PATH).unwrap();
            let wasm_path = PathBuf::from(wasm_path);

            match extracter::extract_wit(wasm_path) {
                Ok(wat) => {
                    println!("extracted WIT:\n{}", wat);
                }
                Err(e) => {
                    println!("{}", e);
                }
            };

            Ok(())
        }
        c => Err(err_msg(format!("Unexpected command: {}", c.0)).into()),
    }
}
