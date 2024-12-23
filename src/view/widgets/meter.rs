//! Horizontal bar meter widget.
use std::borrow::Cow;

use log::*;
use ratatui::prelude::*;
use ratatui::widgets::Widget;

struct MeterValue {
    value: f32,
    color: Color,
}

pub struct Meter {
    label: Cow<'static, str>,
    values: Vec<MeterValue>,
}

impl Meter {
    pub fn new<S: Into<Cow<'static, str>>>(label: S) -> Meter {
        Meter {
            label: label.into(),
            values: Vec::new(),
        }
    }

    pub fn value(mut self, value: f32, color: Color) -> Meter {
        self.values.push(MeterValue { value, color });
        self
    }
}

impl Widget for Meter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(self.label.len() as u16 + 1),
                Constraint::Min(8),
            ],
        )
        .split(area);
        let l = layout[0];
        buf.set_string(l.x, l.y, self.label.as_ref(), Style::new().bold());

        let b = layout[1];
        buf[(b.x, b.y)].set_char('[');
        buf[(b.x + b.width - 1, b.y)].set_char(']');

        let avail_chars = b.width - 2;
        let mut pos = 0;

        for i in 0..self.values.len() {
            let ent = &self.values[i];
            trace!("{}{}: {}", self.label.as_ref(), i, ent.value,);
            let bw = (avail_chars as f32 * ent.value).round() as usize;
            if bw > 0 {
                let bar = "|".repeat(bw);
                let style = Style::new().fg(ent.color);
                buf.set_string(b.x + 1 + pos, b.y, &bar, style);
                pos += bar.chars().count() as u16;
            }
        }
    }
}
