//! Primary dashboard view.

use anyhow::Result;
use ratatui::prelude::*;

use crate::{backend::MonitorBackend, model::MonitorState};

use super::displays::banner::render_banner;
use super::displays::quicklook::render_quicklook;

pub fn render_dashboard<B>(frame: &mut Frame, state: &MonitorState<B>) -> Result<()>
where
    B: MonitorBackend,
{
    let layout = Layout::new()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Min(0),
        ])
        .split(frame.size());
    render_banner(frame, state, layout[0])?;
    render_quicklook(frame, state, layout[2])?;
    Ok(())
}
