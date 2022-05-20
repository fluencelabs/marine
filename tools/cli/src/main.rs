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

use marine_module_info_parser::manifest;
use marine_module_info_parser::sdk_version;

mod args;
mod build;
mod errors;
mod generate;
mod utils;

pub(crate) type CLIResult<T> = std::result::Result<T, crate::errors::CLIError>;

pub fn main() -> Result<(), anyhow::Error> {
    let app = clap::App::new(args::DESCRIPTION)
        .version(args::VERSION)
        .author(args::AUTHORS)
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(args::aqua())
        .subcommand(args::build())
        .subcommand(args::generate())
        .subcommand(args::set())
        .subcommand(args::show_manifest())
        .subcommand(args::show_wit())
        .subcommand(args::repl());
    let arg_matches = app.get_matches();

    match arg_matches.subcommand() {
        ("aqua", Some(args)) => {
            // avoid printing version
            return aqua(args);
        }
        ("build", Some(args)) => build(args),
        ("generate", Some(args)) => generate(args),
        ("set", Some(args)) => set(args),
        ("it", Some(args)) => it(args),
        ("info", Some(args)) => info(args),
        ("repl", Some(args)) => repl(args),
        (c, _) => Err(crate::errors::CLIError::NoSuchCommand(c.to_string()).into()),
    }?;

    // avoid printing version update into not TTY targets
    if !atty::is(atty::Stream::Stdout) {
        return Ok(());
    }

    if let Ok(Some(new_version)) = check_latest::check_max!() {
        use termion::color;

        println!(
            "\nNew version is available! {}{} -> {}{}",
            color::Fg(color::Red),
            check_latest::crate_version!(),
            color::Fg(color::Blue),
            new_version
        );
        println!(
            "{}To update run: {}cargo install marine{}",
            color::Fg(color::Reset),
            color::Fg(color::LightBlack),
            color::Fg(color::Reset)
        );
    }

    Ok(())
}

fn aqua(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    use inflector::Inflector;

    let wasm_path = args.value_of(args::IN_WASM_PATH).unwrap();
    let wasm_path = std::path::Path::new(wasm_path);

    let service_name = match args.value_of(args::SERVICE_NAME) {
        Some(service_name) => service_name.into(),
        None => {
            let service_name = wasm_path
                .file_stem()
                .ok_or_else(|| anyhow::Error::msg("provided path isn't a path to a file"))?;

            service_name.to_string_lossy()
        }
    };
    let service_name = service_name.to_pascal_case();

    // this line allows exporting all the stuff from generated aqua module definition
    println!("module {} declares *\n", service_name);

    let mut module_interface = marine_it_parser::module_interface(wasm_path)?;
    for record in module_interface.record_types.iter() {
        println!("{}", record);
    }

    match args.value_of(args::SERVICE_ID) {
        Some(id) => println!(r#"service {}("{}"):"#, service_name, id),
        None => println!("service {}:", service_name),
    }

    module_interface.function_signatures.sort();
    for sign in module_interface.function_signatures {
        print!("  {}", sign);
    }

    Ok(())
}

fn build(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    let trailing_args: Vec<&str> = args.values_of("optional").unwrap_or_default().collect();

    crate::build::build(trailing_args).map_err(Into::into)
}

fn generate(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    let project_name = args.value_of(args::PROJECT_NAME);
    let should_be_initialized = args.is_present(args::SHOULD_INIT_OPTION);

    crate::generate::generate(project_name, should_be_initialized)
}

fn set(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    match args.subcommand() {
        ("it", Some(args)) => set_it(args),
        ("version", Some(args)) => set_version(args),
        (c, _) => Err(crate::errors::CLIError::NoSuchCommand(c.to_string()).into()),
    }
}

fn set_it(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    let in_wasm_path = args.value_of(args::IN_WASM_PATH).unwrap();
    let it_path = args.value_of(args::IT_PATH).unwrap();
    let out_wasm_path = match args.value_of(args::OUT_WASM_PATH) {
        Some(path) => path,
        None => in_wasm_path,
    };

    let it = std::fs::read(it_path)?;
    let it = String::from_utf8(it)?;

    marine_it_parser::embed_text_it(in_wasm_path, out_wasm_path, &it)?;

    println!("interface types were successfully embedded");

    Ok(())
}

fn set_version(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    use std::str::FromStr;

    let in_wasm_path = args.value_of(args::IN_WASM_PATH).unwrap();
    let version = args.value_of(args::SDK_VERSION).unwrap();
    let out_wasm_path = match args.value_of(args::OUT_WASM_PATH) {
        Some(path) => path,
        None => in_wasm_path,
    };

    let version = semver::Version::from_str(version)?;
    sdk_version::embed_from_path(in_wasm_path, out_wasm_path, &version)?;

    println!("the version was successfully embedded");

    Ok(())
}

fn it(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    let wasm_path = args.value_of(args::IN_WASM_PATH).unwrap();

    let it = marine_it_parser::extract_text_it(wasm_path)?;
    println!("{}", it);

    Ok(())
}

fn info(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    let wasm_path = args.value_of(args::IN_WASM_PATH).unwrap();

    let wasm_module = walrus::ModuleConfig::new().parse_file(wasm_path)?;
    let sdk_version = sdk_version::extract_from_module(&wasm_module)?;
    let module_manifest = manifest::extract_from_module(&wasm_module)?;
    let it_version = marine_it_parser::extract_version_from_module(&wasm_module)?;

    println!("it version:  {}", it_version);
    match sdk_version {
        Some(sdk_version) => println!("sdk version: {}", sdk_version),
        None => println!("module doesn't contain sdk version"),
    }

    match module_manifest {
        Some(manifest) => println!("{}", manifest),
        None => println!("module doesn't contain module manifest"),
    }

    Ok(())
}

fn repl(args: &clap::ArgMatches<'_>) -> Result<(), anyhow::Error> {
    use std::process::Command;
    // use UNIX-specific API for replacing process image
    use std::os::unix::process::CommandExt;

    let trailing_args: Vec<&str> = args.values_of("optional").unwrap_or_default().collect();

    let mut repl = Command::new("mrepl");
    repl.args(trailing_args);
    let error = repl.exec();
    if error.kind() == std::io::ErrorKind::NotFound {
        println!("mrepl not found, run `cargo +nightly install mrepl` to install it");
    } else {
        // this branch should be executed if exec was successful, so just else if fine here
        println!("error occurred: {:?}", error);
    }

    Ok(())
}
