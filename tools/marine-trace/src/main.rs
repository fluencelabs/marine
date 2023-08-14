mod args;
mod run;

use fluence_app_service::AppService;

use anyhow::anyhow;
use clap::Parser;

use std::path::PathBuf;
use crate::args::{Args, Command};

fn main() {
    let args = args::Args::parse();
    init_tracing(args.trace_level.clone(), 0);
    match run_marine(args.config_path, args.command) {
        Err(e) => eprintln!("Failed to execute command: {}", e),
        Ok(()) => eprintln!("Done"),
    }
}

#[allow(dead_code)]
pub fn init_tracing(tracing_params: String, trace_mode: u8) {
    use tracing_subscriber::fmt::format::FmtSpan;

    let builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_params)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_writer(std::io::stdout);
    if trace_mode == 0 {
        builder.json().init();
    } else {
        // Human-readable output.
        builder.init();
    }
}

fn run_marine(config_path: PathBuf, command: args::Command) -> Result<(), anyhow::Error> {
    let result = match command {
        Command::Single {
            func_name,
            args,
            call_parameters,
        } => run::run_single(config_path, &func_name, args, call_parameters),
        Command::CallSchema { call_schema_path } => {
            let call_schema = std::fs::read(&call_schema_path).map_err(|e| {
                anyhow::anyhow!(
                    "Cannot load call schema from path: {}, error: {:?}",
                    call_schema_path.display(),
                    e
                )
            })?;
            let call_schema = serde_json::from_slice(&call_schema).map_err(|e| {
                anyhow::anyhow!(
                    "Cannot parse call schema from path: {}, error: {:?}",
                    call_schema_path.display(),
                    e
                )
            })?;

            run::run_call_schema(config_path, call_schema)
        }
    }?;

    Ok(())
}
