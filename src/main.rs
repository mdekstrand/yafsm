use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use backend::sysmon;
use clap::Parser;

mod backend;
mod dump;
mod event_loop;
mod logging;
mod model;
mod term;
mod view;

use dump::DumpOpts;
use event_loop::run_event_loop;
use model::{MonitorState, Options};
use term::with_terminal;

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
    #[arg(short = 'r', long = "refresh", default_value = "3.0")]
    refresh: f32,

    #[command(flatten)]
    dump: DumpOpts,
}

fn main() -> Result<()> {
    let cli = CLIOptions::parse();
    logging::initialize(cli.log_file.as_ref(), cli.dump.requested(), cli.debug)?;

    let mut options = Options::default();
    options.refresh = Duration::from_secs_f32(cli.refresh);

    let backend = sysmon::initialize()?;

    let mut state = MonitorState::create(options, backend)?;

    if cli.dump.requested() {
        cli.dump.dump(&mut state)?;
        return Ok(());
    }

    with_terminal(move |term| run_event_loop(term, &mut state))?;

    Ok(())
}
