//! Terminal configuration code.
use std::io::{stdout, Stdout};

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub fn with_terminal<F, T>(func: F) -> Result<T>
where
    F: FnOnce(&mut Terminal<CrosstermBackend<Stdout>>) -> Result<T>,
{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut term = Terminal::new(backend)?;

    let res = func(&mut term);

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    res
}
