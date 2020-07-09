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
mod build;

use clap::App;
use clap::AppSettings;

pub fn main() -> Result<(), exitfailure::ExitFailure> {
    let app = App::new("CLI tool for embedding WIT to provided Wasm file")
        .version(args::VERSION)
        .author(args::AUTHORS)
        .about(args::DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(args::build())
        .subcommand(args::show_wit())
        .subcommand(args::validate());

    match app.get_matches().subcommand() {
        ("build", Some(arg)) => {
            let manifest_path = arg
                .value_of(args::IN_WASM_PATH)
                .map(std::path::PathBuf::from);
            crate::build::build(manifest_path);

            Ok(())
        }
        ("show", Some(arg)) => {
            let wasm_path = arg.value_of(args::IN_WASM_PATH).unwrap();
            let wasm_path = std::path::PathBuf::from(wasm_path);

            let result = wit_parser::extract_text_wit(wasm_path)?;
            println!("{}", result);

            Ok(())
        }
        ("validate", Some(arg)) => {
            let _wasm_path = arg.value_of(args::IN_WASM_PATH).unwrap();

            Ok(())
        }
        c => Err(failure::err_msg(format!("Unexpected command: {}", c.0)).into()),
    }
}
