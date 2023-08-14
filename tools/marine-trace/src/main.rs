use anyhow::anyhow;
use clap::{arg, Parser};

use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = ["trace", "debug", "info", "warn", "error", "off"])]
    trace_level: String,
    /// Path to the service config
    #[arg(short, long)]
    config_path: PathBuf,

    /// name of function to call from the service
    #[arg(short, long)]
    func_name: String,

    /// JSON object or JSON array with arguments
    #[arg(short, long, value_parser = parser_json_args)]
    args: serde_json::Value,
}

fn parser_json_args(args: &str) -> Result<serde_json::Value, anyhow::Error> {
    serde_json::from_str(args).map_err(|e| anyhow!(e))
}

fn main() {
    let args = Args::parse();
    init_tracing(args.trace_level.clone(), 0);
    run_marine(args).unwrap();
    //let arg = clap::Arg::new("d").value_parser(clap::builder::ValueParser::new(parser_json_args));
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

fn run_marine(args: Args) -> Result<(), anyhow::Error> {
    let mut config =
        fluence_app_service::TomlAppServiceConfig::load(&args.config_path).map_err(|e| {
            anyhow!(
                "Cannot load marine config from {}, error: {:?}",
                args.config_path.display(),
                e
            )
        })?;

    config.service_base_dir = Some(args.config_path.parent().unwrap().display().to_string());
    config.toml_marine_config.base_path = args.config_path.parent().unwrap().to_path_buf();
    let mut app_service =
        fluence_app_service::AppService::new(config, "traced_service", <_>::default())
            .map_err(|e| anyhow!("Cannot create AppService: {:?}", e))?;
    app_service.call(args.func_name, args.args, <_>::default())?;
    Ok(())
}
