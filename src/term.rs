//! Terminal configuration code.
use std::{
    backtrace::{Backtrace, BacktraceStatus},
    io::{stdout, Stdout},
    panic::{self, PanicInfo, UnwindSafe},
};

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::*;
use ratatui::{backend::CrosstermBackend, Terminal};

#[allow(unused_must_use)]
fn handle_panic(pi: &PanicInfo<'_>) {
    stdout().execute(LeaveAlternateScreen);
    disable_raw_mode();
    eprintln!("{}", pi);
    let bt = Backtrace::capture();
    if bt.status() == BacktraceStatus::Captured {
        eprintln!("{}", bt);
    }
}

pub fn with_terminal<F, T>(func: F) -> Result<T>
where
    F: FnOnce(&mut Terminal<CrosstermBackend<Stdout>>) -> Result<T> + UnwindSafe,
{
    panic::set_hook(Box::new(handle_panic));
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut term = Terminal::new(backend)?;

    let res = func(&mut term);

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    info!("finished");

    res
}
