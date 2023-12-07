//! View code.

use anyhow::Result;
use ratatui::prelude::*;

use crate::SystemState;

use self::banner::render_banner;

mod banner;

pub fn render_screen(frame: &mut Frame, state: &SystemState) -> Result<()> {
    let layout = Layout::new()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(frame.size());
    render_banner(frame, state, layout[0])?;
    Ok(())
}
