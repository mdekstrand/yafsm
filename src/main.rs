use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use backend::sysinfo::SysInfoBackend;
use clap::{ArgAction, Parser};
use log::*;

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
    #[arg(long = "debug", action=ArgAction::Count)]
    debug: u8,

    /// Refresh period (in seconds).
    #[arg(short = 'r', long = "refresh", default_value = "3")]
    refresh: f32,

    #[command(flatten)]
    dump: DumpOpts,
}

fn main() -> Result<()> {
    let cli = CLIOptions::parse();
    logging::initialize(
        cli.log_file.as_ref(),
        if cli.dump.requested() {
            Some(LevelFilter::Info)
        } else {
            None
        },
        if cli.debug > 1 {
            LevelFilter::Trace
        } else if cli.debug > 0 {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
    )?;

    let mut options = Options::default();
    options.refresh = Duration::from_secs_f32(cli.refresh);

    let backend = SysInfoBackend::create()?;

    let mut state = MonitorState::create(options, backend)?;

    if cli.dump.requested() {
        cli.dump.dump(&mut state)?;
        return Ok(());
    }

    with_terminal(move |term| run_event_loop(term, &mut state))?;

    Ok(())
}
