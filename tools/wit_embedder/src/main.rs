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
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod args;

use args::*;

use fce_wit_interfaces::embed_text_wit;
use fce_wit_interfaces::extract_text_wit;
use fce_wit_interfaces::delete_wit_section;

use clap::{App, AppSettings};
use std::path::PathBuf;

pub fn main() -> Result<(), exitfailure::ExitFailure> {
    let app = App::new("CLI tool for embedding WIT to provided Wasm file")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(embed_wit())
        .subcommand(show_wit())
        .subcommand(delete_wit());

    match app.get_matches().subcommand() {
        ("embed", Some(arg)) => {
            let in_wasm_path = arg.value_of(IN_WASM_PATH).unwrap();
            let wit_path = arg.value_of(WIT_PATH).unwrap();
            let out_wasm_path = match arg.value_of(OUT_WASM_PATH) {
                Some(path) => path,
                None => in_wasm_path,
            };

            let wit = String::from_utf8(std::fs::read(wit_path)?).unwrap();

            embed_text_wit(
                PathBuf::from(in_wasm_path),
                PathBuf::from(out_wasm_path),
                &wit,
            )?;

            Ok(())
        }
        ("show", Some(arg)) => {
            let wasm_path = arg.value_of(IN_WASM_PATH).unwrap();
            let wasm_path = PathBuf::from(wasm_path);

            let result = extract_text_wit(wasm_path)?;
            println!("{}", result);

            Ok(())
        }
        ("delete", Some(arg)) => {
            let in_wasm_path = arg.value_of(IN_WASM_PATH).unwrap();
            let out_wasm_path = match arg.value_of(OUT_WASM_PATH) {
                Some(path) => path,
                None => in_wasm_path,
            };

            delete_wit_section(PathBuf::from(in_wasm_path), PathBuf::from(out_wasm_path))?;

            Ok(())
        }
        c => Err(failure::err_msg(format!("Unexpected command: {}", c.0)).into()),
    }
}
