use marine_rs_sdk::CallParameters;

use anyhow::anyhow;
use clap::arg;
use clap::Parser;
use clap::Subcommand;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long, value_parser = ["trace", "debug", "info", "warn", "error", "off"])]
    pub(crate) trace_level: String,
    /// Path to the service config
    #[arg(short, long)]
    pub(crate) config_path: PathBuf,

    /// single or call-schema
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Single {
        /// name of function to call from the service
        #[arg(short, long)]
        func_name: String,

        /// JSON object or JSON array with arguments
        #[arg(short, long, value_parser = parse_json_args)]
        args: serde_json::Value,

        #[arg(short, long, value_parser = parse_call_parameters)]
        call_parameters: Option<CallParameters>,
    },
    CallSchema {
        #[arg(short, long)]
        call_schema_path: PathBuf,
    },
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CallSchema {
    pub(crate) calls: Vec<CallDescriptor>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CallDescriptor {
    pub(crate) func_name: String,
    pub(crate) args: serde_json::Value,
    pub(crate) call_parameters: Option<CallParameters>,
    pub(crate) repeat: i32,
}

fn parse_json_args(args: &str) -> Result<serde_json::Value, anyhow::Error> {
    serde_json::from_str(args).map_err(|e| anyhow!(e))
}

fn parse_call_parameters(arg: &str) -> Result<marine_rs_sdk::CallParameters, anyhow::Error> {
    serde_json::from_str(arg).map_err(|e| anyhow!(e))
}
