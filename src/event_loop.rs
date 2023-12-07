use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::{monitors::SystemState, view::render_screen};

pub fn run_event_loop<B>(term: &mut Terminal<B>, state: &mut SystemState, period: f32) -> Result<()>
where
    B: Backend,
{
    let period = Duration::from_secs_f32(period);
    let mut last_refresh = Instant::now();
    term.clear()?;
    loop {
        term.draw(|frame| render_screen(frame, &state).expect("rendering failed"))?;
        let now = Instant::now();
        let time = now.duration_since(last_refresh);
        let wait = period.saturating_sub(time);
        if poll(wait)? {
            match read()? {
                Event::Key(e) if e.kind == KeyEventKind::Press => {
                    // keypress
                    match e.code {
                        KeyCode::Char('q') => return Ok(()),
                        _ => (),
                    }
                }
                _ => (),
            }
        } else {
            last_refresh = now;
            state.refresh()?;
        }
    }
}
