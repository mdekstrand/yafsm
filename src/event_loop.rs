use std::time::Instant;

use anyhow::Result;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::{model::SystemMonitor, view::render_screen};

pub fn run_event_loop<B>(term: &mut Terminal<B>, state: &mut SystemMonitor) -> Result<()>
where
    B: Backend,
{
    let mut last_refresh = Instant::now();
    term.clear()?;
    loop {
        term.draw(|frame| render_screen(frame, &state).expect("rendering failed"))?;
        let now = Instant::now();
        let time = now.duration_since(last_refresh);
        let wait = state.options.refresh.saturating_sub(time);
        if poll(wait)? {
            match read()? {
                Event::Key(e) if e.kind == KeyEventKind::Press => {
                    // keypress
                    match e.code {
                        KeyCode::Char('q') => return Ok(()),
                        _ => (),
                    }
                }
                _ => (), // covers resize too, no action needed
            }
        } else {
            last_refresh = now;
            state.refresh()?;
        }
    }
}
