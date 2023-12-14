use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use clap::{ArgAction, Parser};
use log::*;

mod backend;
mod dump;
mod event_loop;
mod logging;
mod model;
mod term;
mod view;

#[cfg(target_os = "linux")]
use backend::linux::LinuxBackend;
use backend::sysinfo::SysInfoBackend;
use backend::MonitorBackend;
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

    /// Use fallback sysinfo backend.
    #[arg(long = "sysinfo")]
    sysinfo: bool,

    #[command(flatten)]
    dump: DumpOpts,
}

fn main() -> Result<()> {
    let cli = CLIOptions::parse();
    init_logging(&cli)?;

    let mut options = Options::default();
    options.refresh = Duration::from_secs_f32(cli.refresh);

    let mut backend = create_backend(&cli)?;
    let state = MonitorState::create(options, backend.as_mut())?;
    run_monitor(&cli, state)?;

    Ok(())
}

fn init_logging(cli: &CLIOptions) -> Result<()> {
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
    Ok(())
}

fn run_monitor<'b>(cli: &CLIOptions, state: MonitorState<'b>) -> Result<()> {
    let mut state = state;

    if cli.dump.requested() {
        cli.dump.dump(&mut state)?;
        return Ok(());
    }

    with_terminal(move |term| run_event_loop(term, &mut state))
}

#[cfg(target_os = "linux")]
fn create_backend(cli: &CLIOptions) -> Result<Box<dyn MonitorBackend>> {
    let backend: Box<dyn MonitorBackend> = if cli.sysinfo {
        Box::new(SysInfoBackend::create()?)
    } else {
        Box::new(LinuxBackend::create()?)
    };
    Ok(backend)
}

#[cfg(not(target_os = "linux"))]
fn create_backend(_cli: &CLIOptions) -> Result<Box<SysInfoBackend>> {
    Ok(Box::new(SysInfoBackend::create()?))
}
