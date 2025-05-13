//! Events, states, and controller.
use anyhow::Result;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind, KeyModifiers};
use log::*;
use ratatui::{backend::Backend, Terminal};

use crate::model::MonitorState;
use clock::Clock;
use states::{DefaultStateController, StateController};

mod clock;
mod commands;
mod states;

pub fn run_event_loop<'b, TB: Backend>(
    term: &mut Terminal<TB>,
    state: &mut MonitorState<'b>,
) -> Result<()> {
    let mut clock = Clock::new();
    term.clear()?;
    let mut active_controller: Option<Box<dyn StateController>> =
        Some(DefaultStateController::new());
    while let Some(controller) = active_controller {
        active_controller = event_loop_iter(controller, term, &mut clock, state)?;
    }

    Ok(())
}

fn event_loop_iter<'b, TB: Backend>(
    controller: Box<dyn StateController>,
    term: &mut Terminal<TB>,
    clock: &mut Clock,
    state: &mut MonitorState<'b>,
) -> Result<Option<Box<dyn StateController>>> {
    term.draw(|frame| controller.render(state, frame).expect("rendering failed"))?;
    clock.update_now();
    if poll(clock.next_wait())? {
        match read()? {
            Event::Key(e) if e.kind == KeyEventKind::Press => {
                debug!("key event {:?}", e);
                if e.modifiers.contains(KeyModifiers::CONTROL) && e.code == KeyCode::Char('l') {
                    debug!("^L received, redrawing");
                    term.clear()?;
                    return Ok(Some(controller));
                } else {
                    return Ok(controller.handle_key(e.code, state));
                }
            }
            _ => (), // covers resize too, no action needed
        }
    } else if clock.want_refresh(state.options.refresh) {
        clock.mark_refresh();
        state.refresh()?;
    }

    Ok(Some(controller))
}
