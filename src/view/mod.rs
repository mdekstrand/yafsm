//! View code.

use anyhow::Result;
use ratatui::prelude::*;

use crate::MonitorState;

mod bin1c;
mod dashboard;
mod help;
mod util;
mod widgets;

pub use dashboard::render_dashboard;
pub use help::render_help;
