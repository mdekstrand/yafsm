use std::time::{Duration, SystemTime};

use anyhow::Result;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::{backend::MonitorBackend, model::MonitorState, view::render_screen};

const REFRESH_TOL: Duration = Duration::from_millis(50);

/// Clock to manage polling and refresh rates.  This thing has a weird interface that
/// is closely intertwined with how it is used by [run_event_loop].
struct Clock {
    last_refresh: SystemTime,
    now: SystemTime,
}

pub fn run_event_loop<TB, SB>(term: &mut Terminal<TB>, state: &mut MonitorState<SB>) -> Result<()>
where
    TB: Backend,
    SB: MonitorBackend,
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

impl Clock {
    fn new() -> Clock {
        Clock {
            last_refresh: SystemTime::UNIX_EPOCH,
            now: SystemTime::now(),
        }
    }

    /// Update the clock's current time.
    fn update_now(&mut self) {
        self.now = SystemTime::now()
    }

    fn next_wait(&mut self) -> Duration {
        let st = self
            .now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);
        let millis = st.subsec_millis();
        Duration::from_millis(1000 - millis as u64)
    }

    fn mark_refresh(&mut self) {
        // we do *not* update — mark when we started the refresh
        self.last_refresh = self.now
    }

    fn want_refresh(&mut self, period: Duration) -> bool {
        // need to update now, this will be called later
        self.update_now();
        if let Ok(diff) = self.now.duration_since(self.last_refresh) {
            diff >= period - REFRESH_TOL
        } else {
            // time went backwards
            true
        }
    }
}
