//! Primary dashboard view.

use std::cmp::min;

use anyhow::Result;
use ratatui::prelude::*;

mod banner;
mod process_table;
mod quicklook;
mod summaries;

use banner::render_banner;
use quicklook::render_quicklook;
use summaries::*;

use crate::{backend::MonitorBackend, model::MonitorState};

use self::process_table::render_process_table;

const QL_MIN: u16 = 20;

enum HeaderBlock {
    Meters(u16),
    Gutter(u16),
    Summary {
        cols: u16,
        col_size: u16,
        priority: u16,
    },
}

pub fn render_dashboard<B>(frame: &mut Frame, state: &MonitorState<B>) -> Result<()>
where
    B: MonitorBackend,
{
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Min(0),
        ],
    )
    .split(frame.size());
    render_banner(frame, state, layout[0])?;

    let summaries = [
        (cpu_summary(state)?, 1),
        (memory_summary(state)?, 2),
        (swap_summary(state)?, 4),
        (load_summary(state)?, 3),
    ];
    let mut boxes = Vec::with_capacity(summaries.len() + 1);
    boxes.push(HeaderBlock::Meters(QL_MIN));
    boxes.push(HeaderBlock::Gutter(1));
    boxes.extend(summaries.iter().map(|(ic, priority)| HeaderBlock::Summary {
        cols: ic.num_cols(),
        col_size: ic.col_width(),
        priority: *priority,
    }));
    let summary_split = layout_summaries(&boxes, layout[2]);

    render_quicklook(frame, state, summary_split[0])?;
    for (i, (ic, _)) in summaries.into_iter().enumerate() {
        frame.render_widget(ic, summary_split[i + 2]);
    }

    render_process_table(frame, state, layout[4])?;

    Ok(())
}

fn layout_summaries(blocks: &[HeaderBlock], area: Rect) -> Vec<Rect> {
    if blocks.is_empty() {
        // should never happen but we'll quick-return if it does
        return Vec::new();
    }

    let mut widths = Vec::with_capacity(blocks.len());
    let mut order: Vec<_> = (0..blocks.len()).collect();
    order.sort_by_key(|i| match blocks[*i] {
        HeaderBlock::Meters(_) => 0,
        HeaderBlock::Gutter(_) => 0,
        HeaderBlock::Summary { priority, .. } => priority,
    });

    let mut remaining = area.width;
    let mut min_incr = 100;
    // iniital allocation
    for i in &order {
        let width = match blocks[*i] {
            HeaderBlock::Meters(min) => min,
            HeaderBlock::Gutter(w) => w,
            HeaderBlock::Summary { col_size, .. } => col_size,
        };
        if remaining >= width {
            widths.push(width);
            remaining -= width;
            min_incr = min(min_incr, width);
        }
    }
    assert_eq!(widths.len(), blocks.len());

    // allocate more until nothing has more columns, or we've used up the space
    let mut added = true;
    while added && remaining >= min_incr {
        added = false;
        for i in &order {
            let width = &mut widths[*i];
            let incr = match blocks[*i] {
                HeaderBlock::Summary { cols, col_size, .. } if cols * col_size > *width => {
                    // we have more columns to use
                    col_size
                }
                _ => 0,
            };
            if incr > 0 {
                assert!(remaining >= incr);
                *width += incr;
                remaining -= incr;
                added = true;
            }
        }
    }

    // we have maxed out putting available space into columns. now we have 2 steps:
    // 1. give remaining space to the meter (first rectangle)
    // 2. compute the x-coordinates based on final widths
    widths[0] += remaining;

    let mut x = area.x;
    let mut split = Vec::with_capacity(blocks.len());
    for width in widths {
        split.push(Rect { x, width, ..area });
        x += width;
    }

    split
}
