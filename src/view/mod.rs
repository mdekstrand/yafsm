//! View code.

use anyhow::Result;
use ratatui::prelude::*;

use crate::MonitorState;

use self::dashboard::render_dashboard;

mod bin1c;
mod dashboard;
mod help;
mod util;
mod widgets;

pub fn render_monitor_screen<'b>(frame: &mut Frame, state: &MonitorState<'b>) -> Result<()> {
    render_dashboard(frame, state)
}
