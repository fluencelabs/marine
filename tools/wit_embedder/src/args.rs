use clap::{App, Arg, SubCommand};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub const IN_WASM_PATH: &str = "in-wasm-path";
pub const WIT_PATH: &str = "wit-path";
pub const OUT_WASM_PATH: &str = "out-wasm-path";

pub fn embed_wit<'a, 'b>() -> App<'a, 'b> {
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

pub fn show_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .about("show WIT in provided Wasm file")
        .args(&[Arg::with_name(IN_WASM_PATH)
            .required(true)
            .takes_value(true)
            .short("i")
            .help("path to the wasm file")])
}

pub fn delete_wit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("delete all WIT sections in provided Wasm file")
        .args(&[
            Arg::with_name(IN_WASM_PATH)
                .required(true)
                .takes_value(true)
                .short("i")
                .help("path to the wasm file"),
            Arg::with_name(OUT_WASM_PATH)
                .takes_value(true)
                .short("o")
                .help("path to result file with deleted WIT sections"),
        ])
}
