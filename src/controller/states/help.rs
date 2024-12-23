use anyhow::Result;
use crossterm::event::KeyCode;

use crate::{model::MonitorState, view::render_monitor_screen};

use super::{DefaultStateController, StateController};

pub struct HelpStateController {}

impl HelpStateController {
    pub fn new() -> Box<HelpStateController> {
        Box::new(HelpStateController {})
    }
}

impl StateController for HelpStateController {
    fn render<'s>(
        &self,
        state: &mut MonitorState<'s>,
        frame: &mut ratatui::Frame<'_>,
    ) -> Result<()> {
        render_monitor_screen(frame, &state)?;
        Ok(())
    }

    fn handle_key<'s>(
        self: Box<Self>,
        code: KeyCode,
        _state: &mut MonitorState<'s>,
    ) -> Option<Box<dyn StateController>> {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => Some(DefaultStateController::new()),
            _ => Some(self),
        }
    }
}
