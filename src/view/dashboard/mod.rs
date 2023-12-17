//! Primary dashboard view.

use std::cmp::min;

use anyhow::Result;
use itertools::Itertools;
use log::*;
use ratatui::prelude::*;

mod banner;
mod iotables;
mod process_table;
mod quicklook;
mod summaries;

use banner::render_banner;
use quicklook::render_quicklook;
use summaries::*;

use crate::{backend::error::BackendErrorFilter, model::MonitorState};

use self::{
    iotables::{render_filesystems, render_network},
    process_table::render_process_table,
};

use super::widgets::tablegrp::TableGroup;

const QL_MIN: u16 = 12;

enum HeaderBlock {
    Meters(u16),
    Gutter(u16),
    Summary {
        cols: u16,
        col_size: u16,
        priority: u16,
    },
}

pub fn render_dashboard<'b>(frame: &mut Frame, state: &MonitorState<'b>) -> Result<()> {
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
        (cpu_summary(state).acceptable_to_opt()?, 1),
        (memory_summary(state).acceptable_to_opt()?, 2),
        (swap_summary(state).acceptable_to_opt()?, 4),
        (pressure_summary(state).acceptable_to_opt()?, 5),
        (load_summary(state).acceptable_to_opt()?, 3),
    ];
    let summaries = summaries
        .into_iter()
        .flat_map(|(ico, p)| ico.map(|ic| (ic, p)))
        .collect_vec();
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

    let mut lsg = TableGroup::new();
    render_network(state, &mut lsg)?;
    render_filesystems(state, &mut lsg)?;

    let pt_area = if lsg.n_tables() > 0 {
        let tables = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(lsg.width()),
                Constraint::Length(3),
                Constraint::Min(30),
            ],
        )
        .split(layout[4]);
        frame.render_widget(lsg, tables[0]);
        tables[2]
    } else {
        layout[4]
    };
    render_process_table(frame, state, pt_area)?;

    Ok(())
}

fn layout_summaries(blocks: &[HeaderBlock], area: Rect) -> Vec<Rect> {
    if blocks.is_empty() {
        // should never happen but we'll quick-return if it does
        return Vec::new();
    }

    let mut widths = Vec::with_capacity(blocks.len());
    widths.resize_with(blocks.len(), Default::default);
    let mut order: Vec<_> = (0..blocks.len()).collect();
    order.sort_by_key(|i| match blocks[*i] {
        HeaderBlock::Meters(_) => 0,
        HeaderBlock::Gutter(_) => 0,
        HeaderBlock::Summary { priority, .. } => priority,
    });
    trace!("summary priority order: {:?}", order);

    let mut remaining = area.width;
    let mut min_incr = 100;
    // iniital allocation
    for i in &order {
        let (expandable, width) = match blocks[*i] {
            HeaderBlock::Meters(min) => (true, min),
            HeaderBlock::Gutter(w) => (false, w),
            HeaderBlock::Summary { col_size, .. } => (true, col_size),
        };
        if remaining >= width {
            widths[*i] = width;
            remaining -= width;
            if expandable {
                min_incr = min(min_incr, width);
            }
        } else {
            widths[*i] = 0;
        }
    }
    assert_eq!(widths.len(), blocks.len());
    trace!("layout: {} blocks, min incr {}", widths.len(), min_incr);
    trace!("initial widths: {:?}", widths);

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
            if incr > 0 && remaining >= incr {
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
    trace!("final widths: {:?}", widths);

    let mut x = area.x;
    let mut split = Vec::with_capacity(blocks.len());
    for width in widths {
        split.push(Rect { x, width, ..area });
        x += width;
    }

    split
}
