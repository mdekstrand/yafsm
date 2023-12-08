//! Horizontal bar meter widget.
use std::borrow::Cow;
use std::cmp::min;

use ratatui::prelude::*;
use ratatui::widgets::Widget;

// Note: Unicode bars start at U+2588 (full bar).
const BLOCK_CHAR: char = '\u{2588}';

pub struct Meter {
    label: Cow<'static, str>,
    value: f32,
}

impl Meter {
    pub fn new<S: Into<Cow<'static, str>>>(label: S) -> Meter {
        Meter {
            label: label.into(),
            value: f32::NAN,
        }
    }

    pub fn value(self, value: f32) -> Meter {
        Meter { value, ..self }
    }
}

impl Widget for Meter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(self.label.len() as u16 + 1),
                Constraint::Min(8),
            ])
            .split(area);
        let l = layout[0];
        buf.set_string(l.x, l.y, self.label, Style::new().bold());

        let b = layout[1];
        buf.get_mut(b.x, b.y).symbol = "[".into();
        buf.get_mut(b.x + b.width - 1, b.y).symbol = "]".into();

        let txt = format!("{:.1}%", self.value * 100.0);
        let color = if self.value >= 0.8 {
            Color::Red
        } else if self.value >= 0.6 {
            Color::Yellow
        } else {
            Color::Green
        };

        let space = b.width - 2;
        let bw = (space * 8) as f32 * self.value;
        let bw = bw.floor() as u32;
        let blocks = bw / 8;
        let partial = bw % 8;
        let tlen = txt.len() as u32;
        let bmax = space as u32 - tlen;

        let mut bar = String::with_capacity(space as usize);
        for _ in 0..min(blocks, bmax) {
            bar.push(BLOCK_CHAR);
        }
        if blocks <= bmax && partial > 0 {
            bar.push(char::from_u32(BLOCK_CHAR as u32 + 9 - partial).expect("invalid block char"));
        }
        buf.set_string(b.x + 1, b.y, &bar, Style::new().fg(color));
        buf.set_string(
            b.x + 1 + space - tlen as u16,
            b.y,
            &txt,
            Style::new().fg(color),
        );
        if blocks > bmax {
            let tbw = blocks - bmax;
            buf.set_style(
                Rect {
                    x: bmax as u16,
                    y: b.y,
                    width: tbw as u16,
                    height: 1,
                },
                Style::new().fg(Color::White).bg(color),
            );
        }
    }
}
