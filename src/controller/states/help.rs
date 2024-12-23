use anyhow::Result;
use crossterm::event::KeyCode;

use crate::view::render_dashboard;
use crate::{model::MonitorState, view::render_help};

use super::{DefaultStateController, StateController};

pub struct HelpStateController {
    bindings: Vec<(KeyCode, &'static str)>,
}

impl HelpStateController {
    pub fn new(bindings: Vec<(KeyCode, &'static str)>) -> Box<HelpStateController> {
        Box::new(HelpStateController { bindings })
    }
}

impl StateController for HelpStateController {
    fn render<'s>(
        &self,
        state: &mut MonitorState<'s>,
        frame: &mut ratatui::Frame<'_>,
    ) -> Result<()> {
        render_dashboard(frame, &state)?;
        render_help(frame, state, &self.bindings)?;
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
