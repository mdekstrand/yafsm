use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod dump;
mod logging;
mod monitors;

use dump::DumpOpts;
use monitors::SystemState;

/// System process monitor.
#[derive(Parser, Debug)]
#[command(name = "hypertop")]
struct CLIOptions {
    /// Specify a log file for debug information.
    #[arg(long = "log-file")]
    log_file: Option<PathBuf>,
    /// Enable debug log messages.
    #[arg(long = "debug")]
    debug: bool,

    #[command(flatten)]
    dump: DumpOpts,
}

fn main() -> Result<()> {
    let cli = CLIOptions::parse();
    logging::initialize(cli.log_file.as_ref(), cli.dump.requested(), cli.debug)?;

    let mut state = SystemState::init()?;

    if cli.dump.requested() {
        cli.dump.dump(&mut state)?;
        return Ok(());
    }

    Ok(())
}
