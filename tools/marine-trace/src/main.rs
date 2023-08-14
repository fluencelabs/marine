use std::path::PathBuf;
use clap::Parser;
use serde_json::Value;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the service config
    #[arg(short, long)]
    config_path: PathBuf,

    /// name of function to call from the service
    #[arg(short, long)]
    func_name: String,

    /// JSON object or JSON array with arguments
    #[arg(short, long)]
    args: String,
}


fn main() {
    let args = Args::parse();
    clap::ar
}

#[allow(dead_code)]
pub fn init_tracing(tracing_params: String, trace_mode: u8) {
    use tracing_subscriber::fmt::format::FmtSpan;

    let builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_params)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_writer(std::io::stderr);
    if trace_mode == 0 {
        builder.json().init();
    } else {
        // Human-readable output.
        builder.init();
    }
}
