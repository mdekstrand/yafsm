use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod dump;
mod event_loop;
mod logging;
mod monitors;
mod term;
mod view;

use dump::DumpOpts;
use event_loop::run_event_loop;
use monitors::SystemState;
use term::with_terminal;
use view::render_screen;

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

    /// Refresh period (in seconds).
    #[arg(short = 'r', long = "refresh", default_value = "1.0")]
    refresh: f32,

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

    with_terminal(|term| run_event_loop(term, &mut state, cli.refresh))?;

    Ok(())
}
