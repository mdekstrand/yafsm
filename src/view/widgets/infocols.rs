//! Info columns widget.
use std::{borrow::Cow, cmp::min};

use ratatui::prelude::*;
use ratatui::widgets::Widget;

use crate::view::util::{fmt_bytes, fmt_si_val, level_color};

/// Width of values to display.
///
/// this is set to include a full percentage like "34.2%"".
const VAL_WIDTH: u16 = 5;
/// Max width of labels.
const LABEL_WIDTH: u16 = 6;
/// Width of a full column, incl. padding.
const COL_WIDTH: u16 = 1 + LABEL_WIDTH + 2 + 1 + VAL_WIDTH + 1;
/// Number of rows to display in a column.
const COL_ROWS: u16 = 4;

/// Wrapper for value types in info columns that control their display.
#[derive(Debug, Clone)]
pub enum ICValue {
    /// Empty value
    Blank,
    /// A string, displayed as-is.
    Str(Cow<'static, str>),
    /// A percentage.
    Pct(f32),
    /// A byte count.
    Bytes(u64),
    /// Count of unspecified units.
    Count(u64),
    /// Floating-point value.
    Value(f32),
}

/// Entry for info col widgets.
pub struct ICEntry {
    label: Cow<'static, str>,
    value: ICValue,
    l_style: Option<Style>,
    v_style: Option<Style>,
}

impl ICEntry {
    pub fn new<S: Into<Cow<'static, str>>>(label: S) -> ICEntry {
        ICEntry {
            label: label.into(),
            value: ICValue::Blank,
            l_style: None,
            v_style: None,
        }
    }

    pub fn string<S: Into<Cow<'static, str>>>(self, val: S) -> ICEntry {
        ICEntry {
            value: ICValue::Str(val.into()),
            ..self
        }
    }

    pub fn pct(self, val: f32) -> ICEntry {
        ICEntry {
            value: ICValue::Pct(val),
            ..self
        }
    }

    pub fn bytes(self, val: u64) -> ICEntry {
        ICEntry {
            value: ICValue::Bytes(val),
            ..self
        }
    }

    pub fn count(self, val: u64) -> ICEntry {
        ICEntry {
            value: ICValue::Count(val),
            ..self
        }
    }

    pub fn value(self, val: f32) -> ICEntry {
        ICEntry {
            value: ICValue::Value(val),
            ..self
        }
    }

    pub fn label_style(self, style: Style) -> ICEntry {
        ICEntry {
            l_style: Some(style),
            ..self
        }
    }

    pub fn value_style(self, style: Style) -> ICEntry {
        ICEntry {
            v_style: Some(style),
            ..self
        }
    }
}

/// Mini table-like widget for system information columns.
pub struct InfoCols {
    entries: Vec<ICEntry>,
}

impl InfoCols {
    pub fn new() -> InfoCols {
        InfoCols {
            entries: Vec::with_capacity(3),
        }
    }

    pub fn add(mut self, e: ICEntry) -> InfoCols {
        self.entries.push(e);
        self
    }

    pub fn add_str<S, V>(mut self, label: S, str: V) -> InfoCols
    where
        S: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        self.add(ICEntry::new(label).string(str))
    }

    pub fn add_pct<S: Into<Cow<'static, str>>>(mut self, label: S, pct: f32) -> InfoCols {
        self.add(ICEntry::new(label).pct(pct))
    }

    pub fn add_bytes<S: Into<Cow<'static, str>>>(mut self, label: S, bytes: u64) -> InfoCols {
        self.add(ICEntry::new(label).bytes(bytes))
    }

    pub fn add_count<S: Into<Cow<'static, str>>>(mut self, label: S, count: u64) -> InfoCols {
        self.add(ICEntry::new(label).count(count))
    }

    pub fn add_value<S: Into<Cow<'static, str>>>(mut self, label: S, val: f32) -> InfoCols {
        self.add(ICEntry::new(label).value(val))
    }

    pub fn col_width(&self) -> u16 {
        COL_WIDTH
    }

    pub fn num_cols(&self) -> u16 {
        let mut n = 0;
        n += self.entries.len() as u16 / COL_ROWS;
        if self.entries.len() as u16 % COL_ROWS > 0 {
            n += 1;
        }
        n
    }
}

impl Widget for InfoCols {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < COL_WIDTH {
            // not wide enough to render
            return;
        }

        let mut row = 0;
        let mut col = 0;
        for e in self.entries {
            let x = area.x + col * COL_WIDTH;
            let y = area.y + row;

            let mut l_style = Style::new();
            if let Some(s) = e.l_style {
                l_style = l_style.patch(s);
            } else if row == 0 && col == 0 {
                l_style = l_style.bold();
            }

            let v_str = e.value.format();
            let mut v_style = e.value.style();
            if let Some(vs) = e.v_style {
                v_style = v_style.patch(vs);
            }

            buf.set_stringn(x + 1, y, e.label, LABEL_WIDTH as usize, l_style);
            buf.set_stringn(
                // compute the position to right-align the display
                // value formats use ASCII chars, so we can use len()
                x + 1 + LABEL_WIDTH + 2 + 1 + (VAL_WIDTH - min(v_str.len() as u16, VAL_WIDTH)),
                y,
                v_str,
                LABEL_WIDTH as usize,
                v_style,
            );

            // done drawing — set up position for next entry
            if row == COL_ROWS {
                if area.width < (col + 1) * COL_WIDTH {
                    col += 1;
                    row = 0;
                } else {
                    // not enough room for another column
                    break;
                }
            } else {
                row += 1;
            }
        }
    }
}

impl ICValue {
    fn format(&self) -> Cow<'static, str> {
        match self {
            ICValue::Blank => "".into(),
            ICValue::Str(s) => s.clone(),
            ICValue::Pct(p) if *p < 10.0 => format!("{:4.2}%", p).into(),
            ICValue::Pct(p) if *p > 99.95 => format!("{:.0}%", p.round()).into(),
            ICValue::Pct(p) => format!("{:4.1}%", p).into(),
            ICValue::Bytes(b) => fmt_bytes(*b).into(),
            ICValue::Count(c) => fmt_si_val(*c).into(),
            ICValue::Value(v) if *v >= 100.0 => format!("{:.0}", v).into(),
            ICValue::Value(v) if *v >= 10.0 => format!("{:.1}", v).into(),
            ICValue::Value(v) => format!("{:.2}", v).into(),
        }
    }

    fn style(&self) -> Style {
        match self {
            ICValue::Pct(p) => Style::new().fg(level_color(p / 100.0)),
            _ => Style::new(),
        }
    }
}
