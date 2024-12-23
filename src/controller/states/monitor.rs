//! State controller for the default monitor mode.

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::Frame;

use crate::controller::commands::{dispatch_key, kc, kc_nop, CommandAction};
use crate::model::MonitorState;
use crate::view::render_dashboard;

use super::help::HelpStateController;
use super::StateController;

pub struct DefaultStateController {}

impl DefaultStateController {
    pub fn new() -> Box<DefaultStateController> {
        Box::new(DefaultStateController {})
    }
}

impl StateController for DefaultStateController {
    fn render<'s>(&self, state: &mut MonitorState<'s>, frame: &mut Frame<'_>) -> Result<()> {
        render_dashboard(frame, &state)
    }

    fn handle_key<'s>(
        self: Box<Self>,
        code: KeyCode,
        state: &mut MonitorState<'s>,
    ) -> Option<Box<dyn StateController>> {
        let c = dispatch_key(code, KEY_BINDINGS, state);
        match c {
            'q' => None,
            'h' => {
                let bindings = KEY_BINDINGS.iter().map(|(c, d, _)| (*c, *d)).collect();
                Some(HelpStateController::new(bindings))
            }
            _ => Some(self),
        }
    }
}

fn kc_quit(_state: &mut MonitorState<'_>) -> char {
    'q'
}

fn kc_help(_state: &mut MonitorState<'_>) -> char {
    'h'
}

fn kc_sort_auto(state: &mut MonitorState<'_>) -> char {
    state.proc_sort = None;
    '_'
}

fn kc_sort_cpu(state: &mut MonitorState<'_>) -> char {
    state.proc_sort = Some(crate::model::ProcSortOrder::CPU);
    '_'
}

fn kc_sort_memory(state: &mut MonitorState<'_>) -> char {
    state.proc_sort = Some(crate::model::ProcSortOrder::Memory);
    '_'
}

fn kc_sort_io(state: &mut MonitorState<'_>) -> char {
    state.proc_sort = Some(crate::model::ProcSortOrder::IO);
    '_'
}

fn kc_sort_time(state: &mut MonitorState<'_>) -> char {
    state.proc_sort = Some(crate::model::ProcSortOrder::Time);
    '_'
}

static KEY_BINDINGS: &[(KeyCode, &str, CommandAction<char>)] = &[
    (KeyCode::Null, "Application commands", kc_nop),
    (kc('q'), "quit", kc_quit),
    (kc('h'), "help", kc_help),
    (kc('?'), "!help", kc_help),
    (KeyCode::Null, "Process list sorting", kc_nop),
    (kc('a'), "sort automatically", kc_sort_auto),
    (kc('c'), "sort by CPU", kc_sort_cpu),
    (kc('m'), "sort by memory", kc_sort_memory),
    (kc('i'), "sort by IO", kc_sort_io),
    (kc('t'), "sort by time", kc_sort_time),
];
