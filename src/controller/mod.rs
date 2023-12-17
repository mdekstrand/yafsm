//! Events, states, and controller.
use anyhow::Result;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::model::MonitorState;
use crate::view::render_screen;
use clock::Clock;

mod clock;

pub fn run_event_loop<'b, TB>(term: &mut Terminal<TB>, state: &mut MonitorState<'b>) -> Result<()>
where
    TB: Backend,
{
    let mut clock = Clock::new();
    term.clear()?;
    loop {
        term.draw(|frame| render_screen(frame, &state).expect("rendering failed"))?;
        clock.update_now();
        if poll(clock.next_wait())? {
            match read()? {
                Event::Key(e) if e.kind == KeyEventKind::Press => {
                    // keypress
                    match e.code {
                        KeyCode::Char('q') => return Ok(()),

                        // key process sorting
                        KeyCode::Char('a') => state.proc_sort = None,
                        KeyCode::Char('c') => {
                            state.proc_sort = Some(crate::model::ProcSortOrder::CPU)
                        }
                        KeyCode::Char('m') => {
                            state.proc_sort = Some(crate::model::ProcSortOrder::Memory)
                        }
                        KeyCode::Char('i') => {
                            state.proc_sort = Some(crate::model::ProcSortOrder::IO)
                        }
                        KeyCode::Char('t') if state.backend.has_process_time() => {
                            state.proc_sort = Some(crate::model::ProcSortOrder::Time)
                        }

                        _ => (),
                    }
                }
                _ => (), // covers resize too, no action needed
            }
        } else if clock.want_refresh(state.options.refresh) {
            clock.mark_refresh();
            state.refresh()?;
        }
    }
}
