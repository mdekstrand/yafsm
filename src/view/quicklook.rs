//! Quick-look bar charts

use anyhow::Result;
use ratatui::{
    prelude::*,
    widgets::{Bar, BarChart, BarGroup},
};
use sysinfo::{CpuExt, SystemExt};

use crate::SystemState;

pub(super) fn render_quicklook(frame: &mut Frame, state: &SystemState, area: Rect) -> Result<()> {
    let cpu_usage = state.system.global_cpu_info().cpu_usage();
    let mem_usage = state.system.used_memory() as f32 / state.system.total_memory() as f32;
    let swap_usage = state.system.used_swap() as f32 / state.system.total_swap() as f32;
    let chart = BarChart::default()
        .direction(Direction::Horizontal)
        .bar_gap(0)
        .max(1000)
        .data(
            BarGroup::default().bars(&[
                Bar::default()
                    .label(Line::from("CPU"))
                    .value((cpu_usage * 10.0) as u64)
                    .text_value(format!("{:.1}%", cpu_usage))
                    .style(Style::new().fg(Color::Green)),
                Bar::default()
                    .label(Line::from("MEM"))
                    .value((mem_usage * 1000.0) as u64)
                    .text_value(format!("{:.1}%", mem_usage * 100.0)),
                Bar::default()
                    .label(Line::from("SWP"))
                    .value((swap_usage * 1000.0) as u64)
                    .text_value(format!("{:.1}%", swap_usage * 100.0)),
            ]),
        );
    frame.render_widget(chart, area);
    Ok(())
}
