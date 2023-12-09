//! View code.

use anyhow::Result;
use ratatui::prelude::*;

use crate::{backend::MonitorBackend, MonitorState};

use self::dashboard::render_dashboard;

mod bin1c;
mod dashboard;
mod util;
mod widgets;

pub fn render_screen<B>(frame: &mut Frame, state: &MonitorState<B>) -> Result<()>
where
    B: MonitorBackend,
{
    render_dashboard(frame, state)
}
