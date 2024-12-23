//! Actions for different application states / screens.

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::model::MonitorState;

pub mod help;
pub mod monitor;

/// State-specific controller logic.
pub trait StateController {
    /// Render the application for this state.
    fn render<'s>(&self, state: &mut MonitorState<'s>, frame: &mut Frame<'_>) -> Result<()>;
    /// Handle a key and return the next state controller.
    fn handle_key<'s>(
        self: Box<Self>,
        code: KeyCode,
        state: &mut MonitorState<'s>,
    ) -> Option<Box<dyn StateController>>;
}

pub use monitor::DefaultStateController;
