//! Events, states, and controller.
use anyhow::Result;
use crossterm::event::{poll, read, Event, KeyEventKind};
use log::*;
use ratatui::{backend::Backend, Terminal};

use crate::model::MonitorState;
use crate::view::render_screen;
use clock::Clock;

mod clock;
mod commands;

pub fn run_event_loop<'b, TB>(term: &mut Terminal<TB>, state: &mut MonitorState<'b>) -> Result<()>
where
    TB: Backend,
{
    let mut clock = Clock::new();
    term.clear()?;
    while state.running {
        term.draw(|frame| render_screen(frame, &state).expect("rendering failed"))?;
        clock.update_now();
        if poll(clock.next_wait())? {
            match read()? {
                Event::Key(e) if e.kind == KeyEventKind::Press => {
                    for (kc, desc, action) in commands::KEY_BINDINGS {
                        if e.code == *kc {
                            debug!("found command {}", desc);
                            action(state);
                            break;
                        }
                    }
                }
                _ => (), // covers resize too, no action needed
            }
        } else if clock.want_refresh(state.options.refresh) {
            clock.mark_refresh();
            state.refresh()?;
        }
    }

    Ok(())
}
