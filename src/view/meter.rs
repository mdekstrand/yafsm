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
        let layout = Layout::new()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(self.label.len() as u16 + 1),
                Constraint::Min(8),
            ])
            .split(area);
        let l = layout[0];
        buf.set_string(l.x, l.y, self.label.as_ref(), Style::new().bold());

        let b = layout[1];
        buf.get_mut(b.x, b.y).symbol = "[".into();
        buf.get_mut(b.x + b.width - 1, b.y).symbol = "]".into();

        let avail_chars = b.width - 2;
        let avail_ticks = avail_chars * 8;
        let mut pre_w = 0;
        let mut pos = 0;

        for i in 0..self.values.len() {
            let ent = &self.values[i];
            trace!("{}: {} (pre_w {})", self.label.as_ref(), ent.value, pre_w);
            let bw = (avail_ticks as f32 * ent.value).round() as u32;
            trace!("using {} of {} ticks", bw, avail_ticks);
            if bw <= pre_w {
                pre_w = 0;
                continue;
            }

            let blocks = (bw - pre_w) / 8;
            let partial = (bw - pre_w) % 8;

            let mut bar = "\u{2588}".repeat(blocks as usize);
            if partial > 0 {
                bar.push(char::from_u32(0x2588 + 8 - partial).expect("invalid block char"));
                pre_w = 8 - partial;
            } else {
                pre_w = 0;
            }

            let mut style = Style::new().fg(ent.color);
            if partial > 0
                && i + 1 < self.values.len()
                && self.values[i + 1].value * avail_ticks as f32 >= 1.0
            {
                style = style.bg(self.values[i + 1].color);
            }
            buf.set_string(b.x + 1 + pos, b.y, &bar, style);
            pos += bar.len() as u16;
        }
    }
}
