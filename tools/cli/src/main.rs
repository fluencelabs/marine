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
mod errors;

pub(crate) type Result<T> = std::result::Result<T, crate::errors::CLIError>;

pub fn main() -> std::result::Result<(), anyhow::Error> {
    let app = clap::App::new("CLI tool for dealing with Wasm modules for the Fluence network")
        .version(args::VERSION)
        .author(args::AUTHORS)
        .about(args::DESCRIPTION)
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(args::build())
        .subcommand(args::show_wit());
    let arg_matches = app.get_matches();

    match arg_matches.subcommand() {
        ("build", Some(args)) => {
            let trailing_args: Vec<&str> = args.values_of("optional").unwrap_or_default().collect();

            crate::build::build(trailing_args)?;

            Ok(())
        }
        ("show", Some(arg)) => {
            let wasm_path = arg.value_of(args::IN_WASM_PATH).unwrap();
            let wasm_path = std::path::PathBuf::from(wasm_path);

            let result = fce_wit_parser::extract_text_wit(wasm_path)?;
            println!("{}", result);

            Ok(())
        }
        c => Err(crate::errors::CLIError::NoSuchCommand(c.0.to_string()).into()),
    }
}
