//! Table group widget.

use std::{cmp::max, iter::once, ops::AddAssign};

use ratatui::{
    layout::Constraint,
    prelude::*,
    widgets::{Row, Table, Widget},
};

/// Group of data tables with labels & aligned columns.
pub struct TableGroup<'a> {
    tables: Vec<TGTable<'a>>,
    widths: Vec<u16>,
}

pub struct TGTable<'a> {
    header: TGEntry<'a>,
    rows: Vec<TGEntry<'a>>,
}

struct TGEntry<'a> {
    label: Line<'a>,
    values: Vec<Line<'a>>,
}

#[derive(Default)]
struct WidthAccum {
    width: usize,
}

impl Into<u16> for WidthAccum {
    fn into(self) -> u16 {
        self.width as u16
    }
}

impl<'a> AddAssign<&Line<'a>> for WidthAccum {
    fn add_assign(&mut self, rhs: &Line<'a>) {
        self.width = max(self.width, rhs.width());
    }
}

impl<'a> TableGroup<'a> {
    pub fn new() -> TableGroup<'a> {
        TableGroup {
            tables: Vec::new(),
            widths: Vec::new(),
        }
    }

    pub fn n_tables(&self) -> usize {
        self.tables.len()
    }

    pub fn add_table<'g, L, CI, CH>(&'g mut self, label: L, cols: CI) -> &'g mut TGTable<'a>
    where
        L: Into<Line<'a>>,
        CI: IntoIterator<Item = CH>,
        CH: Into<Line<'a>>,
    {
        self.widths.clear();
        self.tables.push(TGTable {
            header: TGEntry {
                label: label.into().patch_style(Style::new().bold()),
                values: cols.into_iter().map(|c| c.into()).collect(),
            },
            rows: Vec::new(),
        });

        let len = self.tables.len();

        &mut self.tables[len - 1]
    }

    /// Get the widths of the columns in this table.
    fn compute_widths(&mut self) {
        self.widths.clear();
        let ncols = self.tables.iter().map(|t| t.header.values.len()).max();
        let ncols = if let Some(c) = ncols {
            c
        } else {
            return;
        };

        let mut widths: Vec<_> = (0..=ncols).map(|_| WidthAccum::default()).collect();

        for table in self.tables.iter() {
            widths[0] += &table.header.label;
            for (i, v) in table.header.values.iter().enumerate() {
                widths[i + 1] += v;
            }

            for row in table.rows.iter() {
                widths[0] += &row.label;
                for (i, v) in row.values.iter().enumerate() {
                    widths[i + 1] += v;
                }
            }
        }

        self.widths = widths.into_iter().map(|w| w.into()).collect();
        self.widths[0] += 1;
    }

    /// Get the total width of this table.
    pub fn width(&mut self) -> u16 {
        if self.widths.is_empty() {
            self.compute_widths();
        }
        let tw: u16 = self.widths.iter().sum();
        tw + self.widths.len() as u16 - 1
    }
}

impl<'a> Widget for TableGroup<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        if self.widths.is_empty() {
            self.compute_widths();
        }
        let mut heights = Vec::with_capacity(self.tables.len() * 2);
        for (i, table) in self.tables.iter().enumerate() {
            if i > 0 {
                heights.push(Constraint::Length(2))
            }
            heights.push(Constraint::Length(table.rows.len() as u16 + 1));
        }
        let areas = Layout::new(Direction::Vertical, heights).split(area);

        for (i, table) in self.tables.into_iter().enumerate() {
            let area = areas[i * 2];
            let widths: Vec<_> = self.widths.iter().map(|w| Constraint::Length(*w)).collect();
            let rows: Vec<Row<'_>> = table.rows.into_iter().map(|r| r.to_row()).collect();
            let header = table.header.to_row();
            let table = Table::new(rows, widths).header(header);
            Widget::render(table, area, buf);
        }
    }
}

impl<'a> TGTable<'a> {
    pub fn add_row<'g, L, CI, CH>(&'g mut self, label: L, cols: CI)
    where
        L: Into<Line<'a>>,
        CI: IntoIterator<Item = CH>,
        CH: Into<Line<'a>>,
    {
        self.rows.push(TGEntry {
            label: label.into(),
            values: cols
                .into_iter()
                .map(|v| v.into().alignment(Alignment::Right))
                .collect(),
        });
    }
}

impl<'a> TGEntry<'a> {
    /// Get this as a row.
    fn to_row(&self) -> Row<'a> {
        let iter = once(&self.label)
            .chain(self.values.iter())
            .map(|v| v.clone());
        Row::new(iter)
    }
}
